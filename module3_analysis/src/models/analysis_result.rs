use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use module2_ir_builder::api::queries::{ClassInfo, DependencyInfo, FunctionInfo};

// ── Analysis type tag ────────────────────────────────────────────────────────

/// Identifies which kind of analysis produced an AnalysisResult.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnalysisType {
    CallGraph,
    DeadCode,
    Dependency,
}

// ── Summary stats ────────────────────────────────────────────────────────────

/// High-level counts attached to every AnalysisResult.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryStats {
    /// Total number of items examined (functions, classes, modules…)
    pub total_items: usize,
    /// Number of items flagged by the analysis (unused functions, etc.)
    pub flagged_items: usize,
}

// ── The payload a single analysis run can return ─────────────────────────────

/// Concrete data produced by an analysis run.
/// Only one variant is populated per result — the others are empty Vecs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisPayload {
    pub functions: Vec<FunctionInfo>,
    pub classes: Vec<ClassInfo>,
    pub dependencies: Vec<DependencyInfo>,
}

impl AnalysisPayload {
    pub fn empty() -> Self {
        Self {
            functions: Vec::new(),
            classes: Vec::new(),
            dependencies: Vec::new(),
        }
    }
}

// ── Top-level result container ────────────────────────────────────────────────

/// Generic container returned by every analysis in Module 3.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Which analysis produced this result.
    pub analysis_type: AnalysisType,

    /// Wall-clock time the analysis finished.
    /// Stored as seconds since UNIX epoch so it is Serialize-friendly.
    pub generated_at_secs: u64,

    /// High-level counts.
    pub summary_stats: SummaryStats,

    /// The actual data.
    pub payload: AnalysisPayload,
}

impl AnalysisResult {
    /// Convenience constructor — caller supplies type and payload;
    /// summary stats and timestamp are derived automatically.
    pub fn new(analysis_type: AnalysisType, payload: AnalysisPayload) -> Self {
        let total_items =
            payload.functions.len() + payload.classes.len() + payload.dependencies.len();

        // All items in the payload are considered "flagged" by definition
        // (dead-code result = unused functions, dependency result = deps found, etc.)
        let flagged_items = total_items;

        let generated_at_secs = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            analysis_type,
            generated_at_secs,
            summary_stats: SummaryStats {
                total_items,
                flagged_items,
            },
            payload,
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_analysis_result_constructs() {
        let result = AnalysisResult::new(AnalysisType::DeadCode, AnalysisPayload::empty());

        assert_eq!(result.analysis_type, AnalysisType::DeadCode);
        assert_eq!(result.summary_stats.total_items, 0);
        assert_eq!(result.summary_stats.flagged_items, 0);
        assert!(result.generated_at_secs > 0);

        println!("{:#?}", result);
    }

    #[test]
    fn test_analysis_type_variants_are_distinct() {
        assert_ne!(AnalysisType::CallGraph, AnalysisType::DeadCode);
        assert_ne!(AnalysisType::DeadCode, AnalysisType::Dependency);
    }

    #[test]
    fn test_summary_stats_counts_payload() {
        // Manually build a FunctionInfo to check the count logic.
        // We use serde_json round-trip since FunctionInfo has no public constructor.
        let func_json = r#"{
            "name": "login",
            "param_count": 2,
            "is_async": false,
            "parent_scope": null,
            "line_number": 10,
            "file_path": "auth.py"
        }"#;
        let func: FunctionInfo = serde_json::from_str(func_json).unwrap();

        let payload = AnalysisPayload {
            functions: vec![func],
            classes: Vec::new(),
            dependencies: Vec::new(),
        };

        let result = AnalysisResult::new(AnalysisType::DeadCode, payload);
        assert_eq!(result.summary_stats.total_items, 1);
        assert_eq!(result.summary_stats.flagged_items, 1);

        println!("{:#?}", result);
    }
}