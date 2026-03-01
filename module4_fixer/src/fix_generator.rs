use anyhow::{Context, Result};

use module2_ir_builder::api::GraphQuery;
use module3_analysis::AnalysisEngine;

// ── Types ─────────────────────────────────────────────────────────────────────

pub enum FindingKind {
    DeadCode,
}

pub struct Finding {
    pub kind: FindingKind,
    pub name: String,
    pub file_path: String,
    pub line_number: i32,
    pub param_count: i32,
    pub is_async: bool,
}

// ── Collection ────────────────────────────────────────────────────────────────

/// Extract dead-code findings from one file, sorted by line number.
pub fn collect_findings(
    engine: &mut AnalysisEngine,
    query: &GraphQuery,
    file_path: &str,
) -> Vec<Finding> {
    let result = engine.dead_code(query, file_path);
    let mut findings: Vec<Finding> = result
        .payload
        .functions
        .iter()
        .map(|fi| Finding {
            kind: FindingKind::DeadCode,
            name: fi.name.clone(),
            file_path: fi.file_path.clone(),
            line_number: fi.line_number,
            param_count: fi.param_count,
            is_async: fi.is_async,
        })
        .collect();
    findings.sort_by_key(|f| f.line_number);
    findings
}

// ── Source snippet ────────────────────────────────────────────────────────────

/// Read a ~`window`-line region centred on `line_number` from `file_path`.
/// Returns `(1-based start line, snippet text)`.
pub fn read_snippet(file_path: &str, line_number: i32, window: usize) -> Result<(usize, String)> {
    let content = std::fs::read_to_string(file_path)
        .with_context(|| format!("Cannot read source file: {}", file_path))?;

    let lines: Vec<&str> = content.lines().collect();
    let total = lines.len();
    let center = (line_number as usize).saturating_sub(1);
    let half = window / 2;
    let start = center.saturating_sub(half);
    let end = (center + half + 1).min(total);

    let snippet = lines[start..end].join("\n");
    Ok((start + 1, snippet))
}

// ── Prompt ────────────────────────────────────────────────────────────────────

pub fn build_prompt(finding: &Finding, snippet_start: usize, snippet: &str) -> String {
    let async_tag = if finding.is_async { "async " } else { "" };
    let snippet_end = snippet_start + snippet.lines().count().saturating_sub(1);

    format!(
        "You are a code quality assistant. Static analysis has flagged the following \
function as DEAD CODE (it is never called).\n\n\
File: {file_path}\n\
Function: `{async_tag}{name}` ({param_count} parameter(s))\n\
Declared at: line {line_number}\n\n\
Source context (lines {snippet_start}–{snippet_end}):\n\
```\n{snippet}\n```\n\n\
Please provide a concise, actionable suggestion for what to do with this function. Consider:\n\
1. Should this function be deleted entirely?\n\
2. Is it a utility that should be moved to a shared module or made public?\n\
3. Could it be a future entry-point that just needs a call-site?\n\
4. Are there any refactoring opportunities?\n\n\
Keep your answer short (3–5 sentences). Do NOT rewrite the entire file.",
        file_path = finding.file_path,
        async_tag = async_tag,
        name = finding.name,
        param_count = finding.param_count,
        line_number = finding.line_number,
        snippet_start = snippet_start,
        snippet_end = snippet_end,
        snippet = snippet,
    )
}

// ── Display ───────────────────────────────────────────────────────────────────

pub fn display_suggestion(finding: &Finding, suggestion: &str, index: usize, total: usize) {
    let sep = "─".repeat(60);
    println!("\n{sep}");
    println!(
        "  [{index}/{total}] DEAD CODE  ·  {}  ·  line {}",
        finding.file_path, finding.line_number
    );
    println!("  Function: `{}`", finding.name);
    println!("{sep}");
    println!("{suggestion}");
}
