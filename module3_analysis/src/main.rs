//! Module 3 binary — connects to Module 1, builds graph via Module 2,
//! then runs all four analyses and prints results.
//!
//! Usage:
//!   # Terminal 1 — start Module 1
//!   python src/main.py --mode lsp --grpc-port 50051
//!
//!   # Terminal 2 — run Module 3
//!   cargo run -p module3_analysis -- \
//!       --file ../module1_adapter/examples/sample.py \
//!       --language python

use anyhow::Result;
use clap::Parser;
use tracing::info;
use tracing_subscriber;

use module2_ir_builder::api::GraphQuery;
use module2_ir_builder::grpc_client::IREventClient;
use module3_analysis::AnalysisEngine;

#[derive(Parser)]
#[command(name = "module3")]
#[command(about = "Module 3: Semantic Analysis & Impact Engine")]
struct Cli {
    /// Module 1 gRPC server address
    #[arg(short, long, default_value = "http://127.0.0.1:50051")]
    server: String,

    /// Source file to analyse
    #[arg(short, long)]
    file: String,

    /// Programming language (e.g. python, java, go)
    #[arg(short, long, default_value = "python")]
    language: String,

    /// Function name to run impact analysis on (optional)
    #[arg(short, long)]
    impact_target: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli = Cli::parse();

    // ── Step 1: Connect to Module 1 and build the graph ──────────────────────
    info!("Connecting to Module 1 at {}", cli.server);
    let mut client = IREventClient::connect(cli.server.clone()).await?;

    info!("Streaming IR events for: {}", cli.file);
    client.monitor_file(cli.file.clone(), cli.language.clone()).await?;

    let graph = client.into_graph();

    // ── Step 2: Hand graph to Module 3 ───────────────────────────────────────
    let query = GraphQuery::new(&graph);
    let mut engine = AnalysisEngine::new();

    // ── Dead code ─────────────────────────────────────────────────────────────
    println!("\n========== DEAD CODE ANALYSIS ==========");
    let dead = engine.dead_code(&query, &cli.file);
    println!("Unused functions: {}", dead.summary_stats.total_items);
    for f in &dead.payload.functions {
        println!("  ⚠  {} (line {})", f.name, f.line_number);
    }

    // ── Call graph ────────────────────────────────────────────────────────────
    println!("\n========== CALL GRAPH ==========");
    let cg = engine.call_graph(&query, &cli.file);
    println!("Functions: {}", cg.nodes.len());
    println!("Call edges: {}", cg.edges.len());
    for edge in &cg.edges {
        println!("  {} → {}", edge.caller, edge.callee);
    }

    // ── Dependencies ─────────────────────────────────────────────────────────
    println!("\n========== DEPENDENCY ANALYSIS ==========");
    let deps = engine.dependencies(&query, &cli.file);
    println!("Imports: {}", deps.imports.len());
    for (module, info) in &deps.imports {
        if info.is_wildcard {
            println!("  import * from {}", module);
        } else {
            println!("  from {} import {:?}", module, info.imported_names);
        }
    }

    // ── Impact analysis (optional) ────────────────────────────────────────────
    if let Some(ref target) = cli.impact_target {
        println!("\n========== IMPACT ANALYSIS: {} ==========", target);
        let report = engine.impact(&query, &cli.file, target);

        println!("Direct impacts (depth 1): {}", report.direct_impacts.len());
        for f in &report.direct_impacts {
            println!("  depth 1 → {} (line {})", f.name, f.line_number);
        }

        println!("Transitive impacts (depth 2+): {}", report.transitive_impacts.len());
        for f in &report.transitive_impacts {
            let depth = report.impact_depth_levels.get(&f.name).copied().unwrap_or(0);
            println!("  depth {} → {} (line {})", depth, f.name, f.line_number);
        }

        if report.has_no_impact() {
            println!("  No callers found — '{}' is an island.", target);
        }
    } else {
        println!("\nTip: pass --impact-target <function_name> to run impact analysis.");
    }

    // ── Cache stats ───────────────────────────────────────────────────────────
    println!("\n========== CACHE ==========");
    println!("File results cached: {}", engine.cache.file_result_count());
    println!("Impact results cached: {}", engine.cache.impact_result_count());

    Ok(())
}