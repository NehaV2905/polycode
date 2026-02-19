use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use module2_ir_builder::api::queries::FunctionInfo;

// ── Impact report ─────────────────────────────────────────────────────────────

/// Produced by the Impact Analysis Engine.
/// Answers: "If I change `target_symbol`, what else is affected?"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactReport {
    /// The function or symbol being changed.
    pub target_symbol: String,

    /// File that contains the target symbol.
    pub target_file: String,

    /// Functions that directly call `target_symbol` (depth = 1).
    pub direct_impacts: Vec<FunctionInfo>,

    /// Functions that indirectly depend on `target_symbol` (depth >= 2).
    pub transitive_impacts: Vec<FunctionInfo>,

    /// Maps each impacted function name to its depth from `target_symbol`.
    /// depth 1 = direct caller, depth 2 = caller of caller, etc.
    pub impact_depth_levels: HashMap<String, usize>,
}

impl ImpactReport {
    /// Create an empty report for a given target — used as a starting point
    /// before the traversal fills in the impact sets.
    pub fn empty(target_symbol: &str, target_file: &str) -> Self {
        Self {
            target_symbol: target_symbol.to_string(),
            target_file: target_file.to_string(),
            direct_impacts: Vec::new(),
            transitive_impacts: Vec::new(),
            impact_depth_levels: HashMap::new(),
        }
    }

    /// Total number of impacted functions (direct + transitive).
    pub fn total_impact_count(&self) -> usize {
        self.direct_impacts.len() + self.transitive_impacts.len()
    }

    /// Returns true if no callers were found — the symbol is an island.
    pub fn has_no_impact(&self) -> bool {
        self.direct_impacts.is_empty() && self.transitive_impacts.is_empty()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_function_info(name: &str, file: &str) -> FunctionInfo {
        let json = format!(
            r#"{{
                "name": "{}",
                "param_count": 1,
                "is_async": false,
                "parent_scope": null,
                "line_number": 1,
                "file_path": "{}"
            }}"#,
            name, file
        );
        serde_json::from_str(&json).unwrap()
    }

    #[test]
    fn test_empty_report_constructs() {
        let report = ImpactReport::empty("process_order", "orders.py");

        assert_eq!(report.target_symbol, "process_order");
        assert_eq!(report.target_file, "orders.py");
        assert_eq!(report.total_impact_count(), 0);
        assert!(report.has_no_impact());

        println!("{:#?}", report);
    }

    #[test]
    fn test_total_impact_count() {
        let mut report = ImpactReport::empty("process_order", "orders.py");

        report.direct_impacts.push(make_function_info("checkout", "cart.py"));
        report.transitive_impacts.push(make_function_info("place_order", "api.py"));
        report.transitive_impacts.push(make_function_info("confirm_order", "api.py"));

        report.impact_depth_levels.insert("checkout".to_string(), 1);
        report.impact_depth_levels.insert("place_order".to_string(), 2);
        report.impact_depth_levels.insert("confirm_order".to_string(), 2);

        assert_eq!(report.total_impact_count(), 3);
        assert!(!report.has_no_impact());
        assert_eq!(*report.impact_depth_levels.get("checkout").unwrap(), 1);
        assert_eq!(*report.impact_depth_levels.get("place_order").unwrap(), 2);

        println!("{:#?}", report);
    }
}