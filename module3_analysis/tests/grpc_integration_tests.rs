/// Integration tests for Module 3 gRPC server.
///
/// These tests require the full stack to be running:
///   Terminal 1: python start_grpc.py
///   Terminal 2: cargo run -p module3_analysis --bin module3_server -- --file module1_adapter/examples/sample.py
///
/// Run with:
///   cargo test -p module3_analysis --test grpc_integration_test -- --ignored
///
/// Each test is marked #[ignore] so they don't run in CI or during normal
/// `cargo test`. Pass --ignored explicitly to run them.

use tonic::Request;

// Generated client stub — we need build_client(true) for this file.
// The integration test uses the proto client directly.
mod proto {
    tonic::include_proto!("analysis");
}

use proto::analysis_service_client::AnalysisServiceClient;
use proto::{EmptyRequest, FileRequest, ImpactRequest};

const SERVER_ADDR: &str = "http://127.0.0.1:50052";
const FILE: &str = "module1_adapter/examples/sample.py";

async fn connect() -> AnalysisServiceClient<tonic::transport::Channel> {
    AnalysisServiceClient::connect(SERVER_ADDR)
        .await
        .expect("Failed to connect to module3_server on port 50052. Is it running?")
}

// ── HealthCheck ────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn integration_health_check() {
    let mut client = connect().await;
    let resp = client
        .health_check(Request::new(EmptyRequest {}))
        .await
        .expect("health_check RPC failed");

    let body = resp.into_inner();
    assert!(body.ok, "Server reported not ok");
    assert!(body.node_count > 0, "Expected nodes in graph");
    assert!(body.edge_count > 0, "Expected edges in graph");

    println!("Graph: {} nodes, {} edges, {} files",
        body.node_count, body.edge_count, body.file_count);
}

// ── GetTrackedFiles ────────────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn integration_get_tracked_files() {
    let mut client = connect().await;
    let resp = client
        .get_tracked_files(Request::new(EmptyRequest {}))
        .await
        .expect("get_tracked_files RPC failed");

    let files = resp.into_inner().file_paths;
    println!("Tracked files: {:?}", files);

    assert!(
        files.iter().any(|f| f.contains("sample.py")),
        "Expected sample.py in tracked files"
    );
}

// ── GetDeadCode ────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn integration_get_dead_code() {
    let mut client = connect().await;
    let resp = client
        .get_dead_code(Request::new(FileRequest {
            file_path: FILE.to_string(),
        }))
        .await
        .expect("get_dead_code RPC failed");

    let body = resp.into_inner();
    println!("Unused functions: {:?}", body.unused_functions);

    // main() and __init__ should appear as unused (no callers in graph)
    assert!(
        body.unused_functions.contains(&"main".to_string()),
        "Expected 'main' in unused functions"
    );
}

// ── GetCallGraph ───────────────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn integration_get_call_graph() {
    let mut client = connect().await;
    let resp = client
        .get_call_graph(Request::new(FileRequest {
            file_path: FILE.to_string(),
        }))
        .await
        .expect("get_call_graph RPC failed");

    let body = resp.into_inner();
    println!("Call graph nodes: {:?}", body.nodes);
    println!("Call graph edges: {:?}", body.edges);

    assert!(!body.nodes.is_empty(), "Expected function nodes");
    assert!(!body.edges.is_empty(), "Expected call edges");

    let has_main_login = body.edges.iter().any(|e| e.caller == "main" && e.callee == "login");
    assert!(has_main_login, "Expected main→login edge");
}

// ── GetDependencies ────────────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn integration_get_dependencies() {
    let mut client = connect().await;
    let resp = client
        .get_dependencies(Request::new(FileRequest {
            file_path: FILE.to_string(),
        }))
        .await
        .expect("get_dependencies RPC failed");

    let imports = resp.into_inner().imports;
    println!("Imports: {:?}", imports.keys().collect::<Vec<_>>());

    assert!(imports.contains_key("os"),  "Expected 'os' import");
    assert!(imports.contains_key("sys"), "Expected 'sys' import");
}

// ── GetImpact ──────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn integration_get_impact_hash_password() {
    let mut client = connect().await;
    let resp = client
        .get_impact(Request::new(ImpactRequest {
            file_path:     FILE.to_string(),
            target_symbol: "hash_password".to_string(),
        }))
        .await
        .expect("get_impact RPC failed");

    let body = resp.into_inner();
    println!("Direct impacts:     {:?}", body.direct_impacts);
    println!("Transitive impacts: {:?}", body.transitive_impacts);
    println!("Depth levels:       {:?}", body.impact_depth_levels);

    assert!(
        body.direct_impacts.contains(&"login".to_string()),
        "Expected 'login' as direct impact"
    );
    assert!(
        body.direct_impacts.contains(&"create_user".to_string()),
        "Expected 'create_user' as direct impact"
    );
    assert!(
        body.transitive_impacts.contains(&"main".to_string()),
        "Expected 'main' as transitive impact"
    );
}

// ── GetFullAnalysis ────────────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn integration_get_full_analysis() {
    let mut client = connect().await;
    let resp = client
        .get_full_analysis(Request::new(EmptyRequest {}))
        .await
        .expect("get_full_analysis RPC failed");

    let body = resp.into_inner();
    println!("Tracked files: {:?}", body.tracked_files);

    let cg = body.global_call_graph.expect("global_call_graph missing");
    println!("Global call graph: {} nodes, {} edges", cg.nodes.len(), cg.edges.len());

    let dc = body.global_dead_code.expect("global_dead_code missing");
    println!("Global dead code: {:?}", dc.unused_functions);

    let deps = body.global_dependencies.expect("global_dependencies missing");
    println!("Global imports: {:?}", deps.imports.keys().collect::<Vec<_>>());

    assert!(!body.tracked_files.is_empty());
    assert!(!cg.nodes.is_empty());
    assert!(!cg.edges.is_empty());
}