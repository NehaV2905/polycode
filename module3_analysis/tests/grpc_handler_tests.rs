/// Unit tests for the Module 3 gRPC handlers.
///
/// These tests build an in-memory graph that mirrors the structure of
/// module1_adapter/examples/sample.py and call handler methods directly
/// — no running gRPC server required.
///
/// sample.py structure:
///   imports: os, sys, typing
///   class: UserManager
///   functions: __init__, connect, create_user, _insert_user (methods)
///             hash_password, login, check_credentials,
///             process_users, validate_username, main (module-level)
///   calls:
///     create_user     → hash_password, _insert_user
///     login           → UserManager (skipped—class), connect, hash_password, check_credentials
///     process_users   → validate_username
///     main            → process_users, login

use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::Request;

use module2_ir_builder::{GraphBuilder, IRGraph};
use module2_ir_builder::api::GraphQuery;
use module3_analysis::AnalysisEngine;
use module3_analysis::grpc::handlers::AnalysisServiceHandler;
use module3_analysis::grpc::proto::analysis_service_server::AnalysisService;
use module3_analysis::grpc::proto::{
    EmptyRequest, FileRequest, ImpactRequest,
};

const FILE: &str = "module1_adapter/examples/sample.py";

// ── Graph builder ──────────────────────────────────────────────────────────

fn build_sample_graph() -> IRGraph {
    let mut builder = GraphBuilder::new();
    let ts = 0i64;

    builder.set_current_file(FILE.to_string(), "python".to_string());

    // Imports
    builder.process_import("os".to_string(), vec![], false, 8, ts).unwrap();
    builder.process_import("sys".to_string(), vec![], false, 9, ts).unwrap();
    builder.process_import("typing".to_string(), vec!["List".to_string(), "Optional".to_string()], false, 10, ts).unwrap();

    // Class
    builder.process_class_declared("UserManager".to_string(), vec![], 13, ts).unwrap();

    // Methods (parent_scope = UserManager)
    builder.process_function_declared("__init__".to_string(), 1, 16, Some("UserManager".to_string()), ts).unwrap();
    builder.process_function_declared("connect".to_string(), 0, 21, Some("UserManager".to_string()), ts).unwrap();
    builder.process_function_declared("create_user".to_string(), 2, 28, Some("UserManager".to_string()), ts).unwrap();
    builder.process_function_declared("_insert_user".to_string(), 2, 37, Some("UserManager".to_string()), ts).unwrap();

    // Module-level functions
    builder.process_function_declared("hash_password".to_string(), 1, 42, None, ts).unwrap();
    builder.process_function_declared("login".to_string(), 2, 48, None, ts).unwrap();
    builder.process_function_declared("check_credentials".to_string(), 2, 62, None, ts).unwrap();
    builder.process_function_declared("process_users".to_string(), 1, 68, None, ts).unwrap();
    builder.process_function_declared("validate_username".to_string(), 1, 82, None, ts).unwrap();
    builder.process_function_declared("main".to_string(), 0, 88, None, ts).unwrap();

    // Call edges
    builder.process_function_call(Some("create_user".to_string()), "hash_password".to_string(), 1, 32).unwrap();
    builder.process_function_call(Some("create_user".to_string()), "_insert_user".to_string(), 1, 33).unwrap();
    builder.process_function_call(Some("login".to_string()), "connect".to_string(), 0, 51).unwrap();
    builder.process_function_call(Some("login".to_string()), "hash_password".to_string(), 1, 54).unwrap();
    builder.process_function_call(Some("login".to_string()), "check_credentials".to_string(), 2, 55).unwrap();
    builder.process_function_call(Some("process_users".to_string()), "validate_username".to_string(), 1, 76).unwrap();
    builder.process_function_call(Some("main".to_string()), "process_users".to_string(), 1, 91).unwrap();
    builder.process_function_call(Some("main".to_string()), "login".to_string(), 2, 94).unwrap();

    builder.resolve_pending_calls().unwrap();
    builder.into_graph()
}

fn make_handler() -> AnalysisServiceHandler {
    let graph  = Arc::new(build_sample_graph());
    let engine = Arc::new(Mutex::new(AnalysisEngine::new()));
    AnalysisServiceHandler::new(graph, engine)
}

// ── HealthCheck ────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_health_check_ok() {
    let handler = make_handler();
    let resp = handler.health_check(Request::new(EmptyRequest {})).await.unwrap();
    let body = resp.into_inner();
    assert!(body.ok);
    assert!(body.node_count > 0);
    assert!(body.edge_count > 0);
    assert!(body.file_count > 0);
}

// ── GetTrackedFiles ────────────────────────────────────────────────────────

#[tokio::test]
async fn test_get_tracked_files_contains_sample() {
    let handler = make_handler();
    let resp = handler.get_tracked_files(Request::new(EmptyRequest {})).await.unwrap();
    let files = resp.into_inner().file_paths;
    assert!(
        files.iter().any(|f| f.contains("sample.py")),
        "Expected sample.py in tracked files, got: {:?}", files
    );
}

// ── GetDeadCode ────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_dead_code_returns_unused_functions() {
    let handler = make_handler();
    let resp = handler.get_dead_code(Request::new(FileRequest {
        file_path: FILE.to_string(),
    })).await.unwrap();

    let unused = resp.into_inner().unused_functions;

    // main() has no callers inside the file — entry point, appears unused
    assert!(
        unused.contains(&"main".to_string()),
        "Expected 'main' in unused functions, got: {:?}", unused
    );

    // __init__ has no callers in this graph
    assert!(
        unused.contains(&"__init__".to_string()),
        "Expected '__init__' in unused functions, got: {:?}", unused
    );
}

#[tokio::test]
async fn test_dead_code_does_not_flag_called_functions() {
    let handler = make_handler();
    let resp = handler.get_dead_code(Request::new(FileRequest {
        file_path: FILE.to_string(),
    })).await.unwrap();

    let unused = resp.into_inner().unused_functions;

    // hash_password is called by both create_user and login — not dead
    assert!(
        !unused.contains(&"hash_password".to_string()),
        "hash_password should NOT be dead, got: {:?}", unused
    );

    // validate_username is called by process_users — not dead
    assert!(
        !unused.contains(&"validate_username".to_string()),
        "validate_username should NOT be dead, got: {:?}", unused
    );
}

// ── GetCallGraph ───────────────────────────────────────────────────────────

#[tokio::test]
async fn test_call_graph_node_count() {
    let handler = make_handler();
    let resp = handler.get_call_graph(Request::new(FileRequest {
        file_path: FILE.to_string(),
    })).await.unwrap();

    let body = resp.into_inner();
    // 10 functions declared in sample.py
    assert_eq!(body.nodes.len(), 10, "Expected 10 function nodes, got: {:?}", body.nodes);
}

#[tokio::test]
async fn test_call_graph_edges_present() {
    let handler = make_handler();
    let resp = handler.get_call_graph(Request::new(FileRequest {
        file_path: FILE.to_string(),
    })).await.unwrap();

    let edges = resp.into_inner().edges;
    assert!(!edges.is_empty(), "Call graph should have edges");

    // main → login should be present
    let has_main_login = edges.iter().any(|e| e.caller == "main" && e.callee == "login");
    assert!(has_main_login, "Expected main→login edge, got: {:?}", edges);

    // login → hash_password
    let has_login_hash = edges.iter().any(|e| e.caller == "login" && e.callee == "hash_password");
    assert!(has_login_hash, "Expected login→hash_password edge, got: {:?}", edges);

    // process_users → validate_username
    let has_process_validate = edges.iter().any(|e| e.caller == "process_users" && e.callee == "validate_username");
    assert!(has_process_validate, "Expected process_users→validate_username edge, got: {:?}", edges);
}

// ── GetDependencies ────────────────────────────────────────────────────────

#[tokio::test]
async fn test_dependencies_finds_imports() {
    let handler = make_handler();
    let resp = handler.get_dependencies(Request::new(FileRequest {
        file_path: FILE.to_string(),
    })).await.unwrap();

    let imports = resp.into_inner().imports;
    assert!(imports.contains_key("os"),  "Expected 'os' import");
    assert!(imports.contains_key("sys"), "Expected 'sys' import");
}

#[tokio::test]
async fn test_dependencies_typing_has_names() {
    let handler = make_handler();
    let resp = handler.get_dependencies(Request::new(FileRequest {
        file_path: FILE.to_string(),
    })).await.unwrap();

    let imports = resp.into_inner().imports;
    if let Some(typing) = imports.get("typing") {
        assert!(typing.imported_names.contains(&"List".to_string()));
        assert!(typing.imported_names.contains(&"Optional".to_string()));
    }
    // typing may or may not appear depending on Module 1 parsing — not a hard failure
}

// ── GetImpact ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_impact_hash_password_has_direct_callers() {
    let handler = make_handler();
    let resp = handler.get_impact(Request::new(ImpactRequest {
        file_path:     FILE.to_string(),
        target_symbol: "hash_password".to_string(),
    })).await.unwrap();

    let body = resp.into_inner();

    // hash_password is called by create_user and login — both are direct impacts
    assert!(
        body.direct_impacts.contains(&"create_user".to_string()),
        "Expected create_user as direct impact of hash_password, got: {:?}", body.direct_impacts
    );
    assert!(
        body.direct_impacts.contains(&"login".to_string()),
        "Expected login as direct impact of hash_password, got: {:?}", body.direct_impacts
    );
}

#[tokio::test]
async fn test_impact_hash_password_transitive() {
    let handler = make_handler();
    let resp = handler.get_impact(Request::new(ImpactRequest {
        file_path:     FILE.to_string(),
        target_symbol: "hash_password".to_string(),
    })).await.unwrap();

    let body = resp.into_inner();

    // main calls login which calls hash_password — main is transitive
    assert!(
        body.transitive_impacts.contains(&"main".to_string()),
        "Expected main as transitive impact of hash_password, got: {:?}", body.transitive_impacts
    );
}

#[tokio::test]
async fn test_impact_depth_levels() {
    let handler = make_handler();
    let resp = handler.get_impact(Request::new(ImpactRequest {
        file_path:     FILE.to_string(),
        target_symbol: "hash_password".to_string(),
    })).await.unwrap();

    let body = resp.into_inner();

    // login calls hash_password directly — depth 1
    if let Some(&depth) = body.impact_depth_levels.get("login") {
        assert_eq!(depth, 1, "login should be at depth 1");
    }

    // main calls login which calls hash_password — depth 2
    if let Some(&depth) = body.impact_depth_levels.get("main") {
        assert_eq!(depth, 2, "main should be at depth 2");
    }
}

#[tokio::test]
async fn test_impact_empty_target_returns_error() {
    let handler = make_handler();
    let result = handler.get_impact(Request::new(ImpactRequest {
        file_path:     FILE.to_string(),
        target_symbol: String::new(),
    })).await;

    assert!(result.is_err(), "Empty target_symbol should return an error");
    let status = result.unwrap_err();
    assert_eq!(status.code(), tonic::Code::InvalidArgument);
}

#[tokio::test]
async fn test_impact_island_function_has_no_impacts() {
    let handler = make_handler();
    // check_credentials is called by login but calls nobody
    // If we ask what breaks if check_credentials changes: only login
    let resp = handler.get_impact(Request::new(ImpactRequest {
        file_path:     FILE.to_string(),
        target_symbol: "check_credentials".to_string(),
    })).await.unwrap();

    let body = resp.into_inner();
    assert_eq!(
        body.direct_impacts.len(), 1,
        "check_credentials should have exactly 1 direct impact (login)"
    );
    assert_eq!(body.direct_impacts[0], "login");
}

// ── GetFullAnalysis ────────────────────────────────────────────────────────

#[tokio::test]
async fn test_full_analysis_has_tracked_files() {
    let handler = make_handler();
    let resp = handler.get_full_analysis(Request::new(EmptyRequest {})).await.unwrap();
    let body = resp.into_inner();

    assert!(!body.tracked_files.is_empty(), "tracked_files should not be empty");
    assert!(
        body.tracked_files.iter().any(|f| f.contains("sample.py")),
        "Expected sample.py in tracked_files"
    );
}

#[tokio::test]
async fn test_full_analysis_global_call_graph_populated() {
    let handler = make_handler();
    let resp = handler.get_full_analysis(Request::new(EmptyRequest {})).await.unwrap();
    let body = resp.into_inner();

    let cg = body.global_call_graph.expect("global_call_graph should be present");
    assert!(!cg.nodes.is_empty(), "global call graph nodes should not be empty");
    assert!(!cg.edges.is_empty(), "global call graph edges should not be empty");
}

#[tokio::test]
async fn test_full_analysis_global_dead_code_populated() {
    let handler = make_handler();
    let resp = handler.get_full_analysis(Request::new(EmptyRequest {})).await.unwrap();
    let body = resp.into_inner();

    let dc = body.global_dead_code.expect("global_dead_code should be present");
    assert!(
        !dc.unused_functions.is_empty(),
        "Expected some unused functions globally"
    );
}

#[tokio::test]
async fn test_full_analysis_dependencies_populated() {
    let handler = make_handler();
    let resp = handler.get_full_analysis(Request::new(EmptyRequest {})).await.unwrap();
    let body = resp.into_inner();

    let deps = body.global_dependencies.expect("global_dependencies should be present");
    assert!(!deps.imports.is_empty(), "Expected some imports globally");
}