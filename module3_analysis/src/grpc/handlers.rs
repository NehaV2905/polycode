use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

use module2_ir_builder::api::GraphQuery;
use module2_ir_builder::IRGraph;

use crate::AnalysisEngine;
use crate::grpc::proto::{
    analysis_service_server::AnalysisService,
    CallEdge, CallGraphResponse, DeadCodeResponse, DependencyInfo, DependencyResponse,
    EmptyRequest, FileRequest, FullAnalysisResponse, HealthResponse, ImpactRequest,
    ImpactResponse, TrackedFilesResponse,
};

// ── Handler struct ─────────────────────────────────────────────────────────

pub struct AnalysisServiceHandler {
    pub graph:  Arc<IRGraph>,
    pub engine: Arc<Mutex<AnalysisEngine>>,
}

impl AnalysisServiceHandler {
    pub fn new(graph: Arc<IRGraph>, engine: Arc<Mutex<AnalysisEngine>>) -> Self {
        Self { graph, engine }
    }

    /// Derive all tracked file paths by walking every node.
    /// IRGraph.file_nodes is private so we collect from all_nodes().
    fn tracked_files(&self) -> Vec<String> {
        let mut seen: HashSet<String> = HashSet::new();
        self.graph
            .all_nodes()
            .map(|n| n.metadata.file_path.clone())
            .filter(|p| !p.is_empty() && seen.insert(p.clone()))
            .collect()
    }
}

// ── Service implementation ─────────────────────────────────────────────────

#[tonic::async_trait]
impl AnalysisService for AnalysisServiceHandler {

    // ── GetDeadCode ────────────────────────────────────────────────────────

    async fn get_dead_code(
        &self,
        request: Request<FileRequest>,
    ) -> Result<Response<DeadCodeResponse>, Status> {
        let file_path = request.into_inner().file_path;
        let query = GraphQuery::new(&self.graph);
        let mut engine = self.engine.lock().await;

        // detect_dead_code returns AnalysisResult
        // unused functions are stored in payload.functions
        let result = engine.dead_code(&query, &file_path);
        let unused_functions: Vec<String> = result
            .payload
            .functions
            .iter()
            .map(|f| f.name.clone())
            .collect();

        Ok(Response::new(DeadCodeResponse {
            file_path,
            unused_functions,
        }))
    }

    // ── GetCallGraph ───────────────────────────────────────────────────────

    async fn get_call_graph(
        &self,
        request: Request<FileRequest>,
    ) -> Result<Response<CallGraphResponse>, Status> {
        let file_path = request.into_inner().file_path;
        let query = GraphQuery::new(&self.graph);
        let mut engine = self.engine.lock().await;

        // CallGraph.nodes: Vec<FunctionInfo>, .edges: Vec<CallEdge { caller, callee }>
        let cg = engine.call_graph(&query, &file_path);

        let nodes: Vec<String> = cg.nodes.iter().map(|f| f.name.clone()).collect();
        let edges: Vec<CallEdge> = cg.edges.iter().map(|e| CallEdge {
            caller: e.caller.clone(),
            callee: e.callee.clone(),
        }).collect();

        Ok(Response::new(CallGraphResponse {
            file_path,
            nodes,
            edges,
        }))
    }

    // ── GetDependencies ────────────────────────────────────────────────────

    async fn get_dependencies(
        &self,
        request: Request<FileRequest>,
    ) -> Result<Response<DependencyResponse>, Status> {
        let file_path = request.into_inner().file_path;
        let query = GraphQuery::new(&self.graph);
        let mut engine = self.engine.lock().await;

        // DependencyGraph.imports: HashMap<String, DependencyInfo>
        // DependencyInfo has: module_path, imported_names, is_wildcard
        let dg = engine.dependencies(&query, &file_path);

        let imports: std::collections::HashMap<String, DependencyInfo> = dg
            .imports
            .into_iter()
            .map(|(module_path, dep)| {
                let info = DependencyInfo {
                    module_name:    dep.module_path,
                    imported_names: dep.imported_names,
                    is_wildcard:    dep.is_wildcard,
                };
                (module_path, info)
            })
            .collect();

        Ok(Response::new(DependencyResponse {
            file_path,
            imports,
        }))
    }

    // ── GetImpact ──────────────────────────────────────────────────────────

    async fn get_impact(
        &self,
        request: Request<ImpactRequest>,
    ) -> Result<Response<ImpactResponse>, Status> {
        let req = request.into_inner();
        let file_path     = req.file_path;
        let target_symbol = req.target_symbol;

        if target_symbol.is_empty() {
            return Err(Status::invalid_argument(
                "target_symbol must not be empty for GetImpact",
            ));
        }

        let query = GraphQuery::new(&self.graph);
        let mut engine = self.engine.lock().await;

        // ImpactReport: direct_impacts: Vec<FunctionInfo>, transitive_impacts: Vec<FunctionInfo>
        // impact_depth_levels: HashMap<String, usize>
        let report = engine.impact(&query, &file_path, &target_symbol);

        let direct_impacts: Vec<String> = report
            .direct_impacts
            .iter()
            .map(|f| f.name.clone())
            .collect();

        let transitive_impacts: Vec<String> = report
            .transitive_impacts
            .iter()
            .map(|f| f.name.clone())
            .collect();

        let impact_depth_levels: std::collections::HashMap<String, i32> = report
            .impact_depth_levels
            .iter()
            .map(|(k, v)| (k.clone(), *v as i32))
            .collect();

        Ok(Response::new(ImpactResponse {
            target_symbol,
            target_file: file_path,
            direct_impacts,
            transitive_impacts,
            impact_depth_levels,
        }))
    }

    // ── GetFullAnalysis ────────────────────────────────────────────────────
    // Aggregates all four analyses across all tracked files.
    // Used by Module 4 when file_path is null (codebase-wide chat question).

    async fn get_full_analysis(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<FullAnalysisResponse>, Status> {
        let query = GraphQuery::new(&self.graph);
        let mut engine = self.engine.lock().await;

        let tracked_files = self.tracked_files();

        // ── Global call graph ─────────────────────────────────────────────
        let mut all_nodes: Vec<String> = Vec::new();
        let mut all_edges: Vec<CallEdge> = Vec::new();
        let mut seen_nodes: HashSet<String> = HashSet::new();

        for file_path in &tracked_files {
            let cg = engine.call_graph(&query, file_path);
            for f in &cg.nodes {
                if seen_nodes.insert(f.name.clone()) {
                    all_nodes.push(f.name.clone());
                }
            }
            for e in &cg.edges {
                all_edges.push(CallEdge {
                    caller: e.caller.clone(),
                    callee: e.callee.clone(),
                });
            }
        }

        let global_call_graph = CallGraphResponse {
            file_path: String::new(),
            nodes: all_nodes,
            edges: all_edges,
        };

        // ── Global dependencies ───────────────────────────────────────────
        let mut all_imports: std::collections::HashMap<String, DependencyInfo> =
            std::collections::HashMap::new();

        for file_path in &tracked_files {
            let dg = engine.dependencies(&query, file_path);
            for (module_path, dep) in dg.imports {
                all_imports.insert(module_path, DependencyInfo {
                    module_name:    dep.module_path,
                    imported_names: dep.imported_names,
                    is_wildcard:    dep.is_wildcard,
                });
            }
        }

        let global_dependencies = DependencyResponse {
            file_path: String::new(),
            imports:   all_imports,
        };

        // ── Global dead code ──────────────────────────────────────────────
        let mut all_unused: Vec<String> = Vec::new();

        for file_path in &tracked_files {
            let result = engine.dead_code(&query, file_path);
            for f in &result.payload.functions {
                all_unused.push(f.name.clone());
            }
        }

        let global_dead_code = DeadCodeResponse {
            file_path: String::new(),
            unused_functions: all_unused,
        };

        // cross_file_impacts left empty — impact requires a specific target_symbol
        // and cannot be pre-computed globally without knowing what to target

        Ok(Response::new(FullAnalysisResponse {
            tracked_files,
            global_call_graph:   Some(global_call_graph),
            global_dependencies: Some(global_dependencies),
            global_dead_code:    Some(global_dead_code),
            cross_file_impacts:  Vec::new(),
        }))
    }

    // ── GetTrackedFiles ────────────────────────────────────────────────────

    async fn get_tracked_files(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<TrackedFilesResponse>, Status> {
        Ok(Response::new(TrackedFilesResponse {
            file_paths: self.tracked_files(),
        }))
    }

    // ── HealthCheck ────────────────────────────────────────────────────────

    async fn health_check(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        let stats = self.graph.stats();
        Ok(Response::new(HealthResponse {
            ok:         true,
            node_count: stats.node_count as i32,
            edge_count: stats.edge_count as i32,
            file_count: stats.file_count as i32,
        }))
    }
}