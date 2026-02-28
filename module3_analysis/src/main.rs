//! Module 3 binary — connects to Module 1, builds graph via Module 2,
//! then runs all four analyses and prints results.
//!
//! Usage:
//!   # Terminal 1 — start Module 1
//!   cd /Users/srishti/polycode && python3 test_integration_v3.py
//!
//!   # Terminal 2 — single file (language is auto-detected from file extension)
//!   cargo run -p module3_analysis -- --file ../module1_adapter/examples/sample.py
//!
//!   # Analyse an entire GitHub repository
//!   cargo run -p module3_analysis -- --repo https://github.com/owner/repo
//!
//!   # Optional: override language manually (single-file mode only)
//!   cargo run -p module3_analysis -- --file ../module1_adapter/examples/sample.py --language python
//!
//!   # Optional: run impact analysis on a specific function
//!   cargo run -p module3_analysis -- --file ../module1_adapter/examples/sample.py --impact-target login

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use tracing::info;
use tracing_subscriber;

use module2_ir_builder::api::GraphQuery;
use module2_ir_builder::detect_language;
use module2_ir_builder::grpc_client::IREventClient;
use module3_analysis::AnalysisEngine;

// ── CLI ───────────────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(name = "module3")]
#[command(about = "Module 3: Semantic Analysis & Impact Engine")]
struct Cli {
    /// Module 1 gRPC server address
    #[arg(short, long, default_value = "http://127.0.0.1:50051")]
    server: String,

    /// Source file to analyse (mutually exclusive with --repo)
    #[arg(short, long)]
    file: Option<String>,

    /// GitHub repository URL to analyse (mutually exclusive with --file)
    #[arg(short = 'r', long)]
    repo: Option<String>,

    /// Programming language (auto-detected from file extension if not specified)
    #[arg(short, long)]
    language: Option<String>,

    /// Function name to run impact analysis on (optional, single-file mode only)
    #[arg(short, long)]
    impact_target: Option<String>,
}

enum Mode {
    SingleFile(String),
    Repo(String),
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli = Cli::parse();

    let mode = match (&cli.file, &cli.repo) {
        (Some(_), Some(_)) => {
            eprintln!("error: --file and --repo are mutually exclusive");
            std::process::exit(1);
        }
        (None, None) => {
            eprintln!("error: one of --file or --repo is required");
            std::process::exit(1);
        }
        (Some(f), None) => Mode::SingleFile(f.clone()),
        (None, Some(u)) => Mode::Repo(u.clone()),
    };

    match mode {
        Mode::SingleFile(path) => analyze_single_file(&cli, path).await?,
        Mode::Repo(url) => analyze_repo(&cli.server, &url).await?,
    }

    Ok(())
}

// ── Single-file analysis (original behaviour) ─────────────────────────────────

async fn analyze_single_file(cli: &Cli, file_path: String) -> Result<()> {
    // ── Step 1: Resolve language (auto-detect or use provided) ───────────────
    let language = match &cli.language {
        Some(lang) => {
            info!("Using specified language: {}", lang);
            lang.clone()
        }
        None => match detect_language(&file_path) {
            Ok(lang) => {
                info!("Auto-detected language: {} from file extension", lang.display_name());
                lang.as_str().to_string()
            }
            Err(e) => return Err(anyhow::anyhow!("Language detection failed: {}", e)),
        },
    };

    // ── Step 2: Connect to Module 1 and build the graph ──────────────────────
    info!("Connecting to Module 1 at {}", cli.server);
    let mut client = IREventClient::connect(cli.server.clone()).await?;

    info!("Streaming IR events for: {}", file_path);
    client.monitor_file(file_path.clone(), language).await?;

    let graph = client.into_graph();

    // ── Step 3: Hand graph to Module 3 ───────────────────────────────────────
    let query = GraphQuery::new(&graph);
    let mut engine = AnalysisEngine::new();

    // ── Dead code ─────────────────────────────────────────────────────────────
    println!("\n========== DEAD CODE ANALYSIS ==========");
    let dead = engine.dead_code(&query, &file_path);
    println!("Unused functions: {}", dead.summary_stats.total_items);
    for f in &dead.payload.functions {
        println!("  ⚠  {} (line {})", f.name, f.line_number);
    }

    // ── Call graph ────────────────────────────────────────────────────────────
    println!("\n========== CALL GRAPH ==========");
    let cg = engine.call_graph(&query, &file_path);
    println!("Functions: {}", cg.nodes.len());
    println!("Call edges: {}", cg.edges.len());
    for edge in &cg.edges {
        println!("  {} → {}", edge.caller, edge.callee);
    }

    // ── Dependencies ─────────────────────────────────────────────────────────
    println!("\n========== DEPENDENCY ANALYSIS ==========");
    let deps = engine.dependencies(&query, &file_path);
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
        let report = engine.impact(&query, &file_path, target);

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

// ── Repository analysis ───────────────────────────────────────────────────────

fn clone_repo(url: &str, dest: &Path) -> Result<()> {
    let status = std::process::Command::new("git")
        .args(["clone", "--depth", "1", url])
        .arg(dest)
        .status()
        .context("Failed to spawn `git clone` — is git installed?")?;
    if !status.success() {
        anyhow::bail!("`git clone --depth 1 {}` failed (exit {:?})", url, status.code());
    }
    Ok(())
}

fn find_source_files(dir: &Path) -> Vec<PathBuf> {
    const MAX_DEPTH: usize = 20;
    const SKIP_DIRS: &[&str] = &[".git", "target", "node_modules", "vendor"];
    const EXTS: &[&str] = &["py", "java", "go", "c", "h", "rb", "rs"];

    let mut result = Vec::new();
    let mut stack: Vec<(PathBuf, usize)> = vec![(dir.to_path_buf(), 0)];

    while let Some((cur, depth)) = stack.pop() {
        if depth > MAX_DEPTH {
            continue;
        }
        for entry in std::fs::read_dir(&cur).into_iter().flatten().flatten() {
            let path = entry.path();
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if !SKIP_DIRS.contains(&name) {
                    stack.push((path, depth + 1));
                }
            } else {
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                if EXTS.contains(&ext.as_str()) {
                    result.push(path);
                }
            }
        }
    }
    result.sort();
    result
}

async fn analyze_repo(server: &str, url: &str) -> Result<()> {
    // 1. Clone into auto-cleanup TempDir
    let tmp = tempfile::tempdir().context("Failed to create temp dir")?;
    println!("Cloning {} ...", url);
    clone_repo(url, tmp.path())?;

    // 2. Discover files
    let files = find_source_files(tmp.path());
    if files.is_empty() {
        println!("No supported source files found (.py .java .go .c .h .rb .rs)");
        return Ok(());
    }
    println!("Found {} supported source file(s).", files.len());

    // 3. Connect to Module 1 once, stream all files
    let mut client = IREventClient::connect(server.to_string()).await?;
    let mut processed: Vec<String> = Vec::new();
    let mut skipped: Vec<(String, String)> = Vec::new();

    for abs_path in &files {
        let display = abs_path
            .strip_prefix(tmp.path())
            .unwrap_or(abs_path)
            .to_string_lossy()
            .to_string();
        let abs_str = abs_path.to_string_lossy().to_string();
        let lang = match detect_language(&abs_str) {
            Ok(l) => l.as_str().to_string(),
            Err(e) => {
                skipped.push((display, e.to_string()));
                continue;
            }
        };
        match client.monitor_file(abs_str.clone(), lang).await {
            Ok(()) => processed.push(abs_str),
            Err(e) => {
                skipped.push((display, e.to_string()));
            }
        }
    }

    if processed.is_empty() {
        println!("All files were skipped — nothing to analyse.");
        return Ok(());
    }

    // 4. Build graph
    let graph = client.into_graph();
    let query = GraphQuery::new(&graph);
    let mut engine = AnalysisEngine::new();
    let stats = query.get_stats();

    // 5. Aggregate totals
    let (mut total_fn, mut total_edges, mut total_imports, mut total_dead) = (0, 0, 0, 0);
    for p in &processed {
        let cg = engine.call_graph(&query, p);
        let deps = engine.dependencies(&query, p);
        let dead = engine.dead_code(&query, p);
        total_fn += cg.nodes.len();
        total_edges += cg.edges.len();
        total_imports += deps.imports.len();
        total_dead += dead.summary_stats.total_items;
    }

    // 6. Print repo-level summary
    println!("\n========== REPOSITORY SUMMARY ==========");
    println!("  Repository   : {}", url);
    println!("  Files parsed : {}/{}", processed.len(), files.len());
    println!("  Total nodes  : {}", stats.total_nodes);
    println!("  Total edges  : {}", stats.total_edges);
    println!("  Functions    : {}", total_fn);
    println!("  Call edges   : {}", total_edges);
    println!("  Imports      : {}", total_imports);
    println!("  Dead code    : {} unused function(s)", total_dead);
    println!("=========================================");

    // 7. Per-file breakdown (cache hits — no recompute)
    println!("\n========== PER-FILE BREAKDOWN ==========");
    for abs_str in &processed {
        let display = Path::new(abs_str)
            .strip_prefix(tmp.path())
            .unwrap_or(Path::new(abs_str))
            .to_string_lossy()
            .to_string();

        println!("\n  FILE: {}", display);
        let dead = engine.dead_code(&query, abs_str);
        println!("  Dead code    : {} unused", dead.summary_stats.total_items);
        for f in &dead.payload.functions {
            println!("    [UNUSED] {} (line {})", f.name, f.line_number);
        }

        let cg = engine.call_graph(&query, abs_str);
        println!("  Call graph   : {} fn, {} edges", cg.nodes.len(), cg.edges.len());
        for e in &cg.edges {
            println!("    {} -> {}", e.caller, e.callee);
        }

        let deps = engine.dependencies(&query, abs_str);
        println!("  Dependencies : {} import(s)", deps.imports.len());
        for (m, info) in &deps.imports {
            if info.is_wildcard {
                println!("    import * from {}", m);
            } else {
                println!("    from {} import {:?}", m, info.imported_names);
            }
        }
    }

    if !skipped.is_empty() {
        println!("\n========== SKIPPED FILES ==========");
        for (p, reason) in &skipped {
            println!("  [SKIP] {} — {}", p, reason);
        }
    }

    // tmp drops here → clone deleted automatically
    Ok(())
}
