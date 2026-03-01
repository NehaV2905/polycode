//! Module 4 binary — AI-powered fix suggester.
//!
//! Runs the same analysis pipeline as Module 3 (dead code detection),
//! then calls the Groq API (llama-3.3-70b-versatile) for each unused function
//! and prints a concise, actionable suggestion. No files are modified.
//!
//! Usage:
//!   export GROQ_API_KEY=gsk_...
//!
//!   # Single file
//!   cargo run -p module4_fixer -- --file path/to/file.py
//!
//!   # Entire GitHub repository
//!   cargo run -p module4_fixer -- --repo https://github.com/owner/repo
//!
//!   # Raise the suggestion cap (default 10)
//!   cargo run -p module4_fixer -- --file path/to/file.py --max-fixes 25

mod fix_generator;
mod groq_client;

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use tracing::info;

use module2_ir_builder::api::GraphQuery;
use module2_ir_builder::detect_language;
use module2_ir_builder::grpc_client::IREventClient;
use module3_analysis::AnalysisEngine;

use fix_generator::{build_prompt, collect_findings, display_suggestion, read_snippet};
use groq_client::GroqClient;

// ── CLI ───────────────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(name = "module4")]
#[command(about = "Module 4: AI-Powered Fix Suggester (powered by Groq)")]
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

    /// Maximum number of AI fix suggestions to generate
    #[arg(long, default_value_t = 10)]
    max_fixes: usize,
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

    let api_key = std::env::var("GROQ_API_KEY")
        .context("GROQ_API_KEY environment variable is not set — get a free key at console.groq.com")?;

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

    let groq = GroqClient::new(api_key)?;

    match mode {
        Mode::SingleFile(path) => fix_file(&cli, path, &groq).await?,
        Mode::Repo(url) => fix_repo(&cli, &url, &groq).await?,
    }

    Ok(())
}

// ── Single-file fix ───────────────────────────────────────────────────────────

async fn fix_file(cli: &Cli, file_path: String, groq: &GroqClient) -> Result<()> {
    let language = match detect_language(&file_path) {
        Ok(lang) => {
            info!("Detected language: {}", lang.display_name());
            lang.as_str().to_string()
        }
        Err(e) => anyhow::bail!("Language detection failed: {}", e),
    };

    info!("Connecting to Module 1 at {}", cli.server);
    let mut ir_client = IREventClient::connect(cli.server.clone()).await?;
    ir_client.monitor_file(file_path.clone(), language).await?;
    let graph = ir_client.into_graph();

    let query = GraphQuery::new(&graph);
    let mut engine = AnalysisEngine::new();
    let findings = collect_findings(&mut engine, &query, &file_path);

    println!("\n========== MODULE 4: AI FIX SUGGESTER ==========");
    println!("File     : {}", file_path);
    println!("Findings : {} dead function(s)", findings.len());
    println!("Cap      : {} suggestion(s) (--max-fixes)", cli.max_fixes);

    if findings.is_empty() {
        println!("\nNo dead code found — nothing to fix.");
        return Ok(());
    }

    let capped: Vec<_> = findings.iter().take(cli.max_fixes).collect();
    let total = capped.len();

    for (i, finding) in capped.iter().enumerate() {
        let (snippet_start, snippet) = match read_snippet(&finding.file_path, finding.line_number, 50) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!("  [SKIP] Cannot read {}: {}", finding.file_path, e);
                continue;
            }
        };

        let prompt = build_prompt(finding, snippet_start, &snippet);
        let suggestion = groq
            .suggest_fix(prompt)
            .await
            .unwrap_or_else(|e| format!("[Groq API error: {}]", e));

        display_suggestion(finding, &suggestion, i + 1, total);
    }

    println!("\n========================================");
    println!(
        "Done. Showed {}/{} suggestions (dry-run — no files were modified).",
        total,
        findings.len()
    );
    Ok(())
}

// ── Repository fix ────────────────────────────────────────────────────────────

async fn fix_repo(cli: &Cli, url: &str, groq: &GroqClient) -> Result<()> {
    let tmp = tempfile::tempdir().context("Failed to create temp dir")?;
    println!("Cloning {} ...", url);
    clone_repo(url, tmp.path())?;

    let files = find_source_files(tmp.path());
    if files.is_empty() {
        println!("No supported source files found (.py .java .go .c .h .rb .rs)");
        return Ok(());
    }
    println!("Found {} supported source file(s).", files.len());

    let mut ir_client = IREventClient::connect(cli.server.clone()).await?;
    let mut processed: Vec<String> = Vec::new();
    let mut skipped: Vec<(String, String)> = Vec::new();

    for abs_path in &files {
        let abs_str = abs_path.to_string_lossy().to_string();
        let lang = match detect_language(&abs_str) {
            Ok(l) => l.as_str().to_string(),
            Err(e) => {
                skipped.push((abs_str, e.to_string()));
                continue;
            }
        };
        match ir_client.monitor_file(abs_str.clone(), lang).await {
            Ok(()) => processed.push(abs_str),
            Err(e) => skipped.push((abs_str, e.to_string())),
        }
    }

    if processed.is_empty() {
        println!("All files were skipped — nothing to analyse.");
        return Ok(());
    }

    let graph = ir_client.into_graph();
    let query = GraphQuery::new(&graph);
    let mut engine = AnalysisEngine::new();

    let mut all_findings = Vec::new();
    for file_path in &processed {
        all_findings.extend(collect_findings(&mut engine, &query, file_path));
    }

    println!("\n========== MODULE 4: AI FIX SUGGESTER (REPO) ==========");
    println!("Repository: {}", url);
    println!("Files     : {}/{} parsed", processed.len(), files.len());
    println!("Findings  : {} dead function(s) across all files", all_findings.len());
    println!("Cap       : {} suggestion(s) (--max-fixes)", cli.max_fixes);

    if all_findings.is_empty() {
        println!("\nNo dead code found — nothing to fix.");
        return Ok(());
    }

    let capped: Vec<_> = all_findings.iter().take(cli.max_fixes).collect();
    let total = capped.len();

    for (i, finding) in capped.iter().enumerate() {
        let (snippet_start, snippet) = match read_snippet(&finding.file_path, finding.line_number, 50) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!("  [SKIP] Cannot read {}: {}", finding.file_path, e);
                continue;
            }
        };

        let prompt = build_prompt(finding, snippet_start, &snippet);
        let suggestion = groq
            .suggest_fix(prompt)
            .await
            .unwrap_or_else(|e| format!("[Groq API error: {}]", e));

        display_suggestion(finding, &suggestion, i + 1, total);
    }

    println!("\n========================================");
    println!(
        "Done. Showed {}/{} suggestions (dry-run — no files were modified).",
        total,
        all_findings.len()
    );

    if !skipped.is_empty() {
        println!("\n========== SKIPPED FILES ==========");
        for (p, reason) in &skipped {
            println!("  [SKIP] {} — {}", p, reason);
        }
    }

    // tmp drops here → clone deleted automatically
    Ok(())
}

// ── Helpers (mirrored from module3) ──────────────────────────────────────────

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
