use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use tokio::sync::Mutex;
use tonic::transport::Server;

use module2_ir_builder::grpc_client::IREventClient;
use module3_analysis::AnalysisEngine;
use module3_analysis::grpc::handlers::AnalysisServiceHandler;
use module3_analysis::grpc::proto::analysis_service_server::AnalysisServiceServer;

#[derive(Parser, Debug)]
#[command(name = "module3_server", about = "Polycode — Module 3 gRPC Analysis Server")]
struct Args {
    /// Module 2 gRPC address (where Module 1 adapter is listening)
    #[arg(long, default_value = "http://127.0.0.1:50051")]
    module2_addr: String,

    /// Port to expose the Module 3 gRPC service on
    #[arg(long, default_value = "50052")]
    listen_port: u16,

    /// Source file path to analyse on startup
    #[arg(long)]
    file: String,

    /// Programming language hint
    #[arg(long, default_value = "python")]
    language: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // ── Step 1: Connect to Module 2 and stream IR events ──────────────────
    //
    // IREventClient API (from grpc_client/mod.rs):
    //   connect(addr: String) -> Result<Self>     — owns its own GraphBuilder
    //   monitor_file(path: String, lang: String)  — streams events, resolves calls
    //   into_graph(self) -> IRGraph               — consumes client, returns graph

    println!(
        "[module3_server] Connecting to Module 2 at {} ...",
        args.module2_addr
    );

    let mut client = IREventClient::connect(args.module2_addr.clone())
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to Module 2: {}", e))?;

    println!(
        "[module3_server] Connected. Streaming IR events for '{}' ...",
        args.file
    );

    client
        .monitor_file(args.file.clone(), args.language.clone())
        .await
        .map_err(|e| anyhow::anyhow!("Failed to stream IR events: {}", e))?;

    let ir_graph = client.into_graph();

    let stats = ir_graph.stats();
    println!(
        "[module3_server] IRGraph built — {} nodes, {} edges, {} files.",
        stats.node_count, stats.edge_count, stats.file_count
    );

    // ── Step 2: Wrap in Arc for shared ownership across async handlers ─────

    let graph  = Arc::new(ir_graph);
    let engine = Arc::new(Mutex::new(AnalysisEngine::new()));

    // ── Step 3: Start gRPC server ──────────────────────────────────────────

    let addr = format!("0.0.0.0:{}", args.listen_port)
        .parse()
        .map_err(|e| anyhow::anyhow!("Invalid listen address: {}", e))?;

    let handler = AnalysisServiceHandler::new(Arc::clone(&graph), Arc::clone(&engine));
    let service = AnalysisServiceServer::new(handler);

    println!(
        "[module3_server] Listening on port {} — ready for Module 4.",
        args.listen_port
    );

    Server::builder()
        .add_service(service)
        .serve(addr)
        .await
        .map_err(|e| anyhow::anyhow!("gRPC server error: {}", e))?;

    Ok(())
}