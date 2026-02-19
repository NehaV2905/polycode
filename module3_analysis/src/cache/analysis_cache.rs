use std::collections::HashMap;

use crate::models::analysis_result::AnalysisResult;
use crate::models::impact_report::ImpactReport;

// ── Cache keys ────────────────────────────────────────────────────────────────

/// Key for file-level analyses (dead code, call graph, dependency).
/// Uniquely identifies one analysis run over one file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileAnalysisKey {
    pub analysis_type: String, // "dead_code" | "call_graph" | "dependency"
    pub file_path: String,
}

impl FileAnalysisKey {
    pub fn new(analysis_type: &str, file_path: &str) -> Self {
        Self {
            analysis_type: analysis_type.to_string(),
            file_path: file_path.to_string(),
        }
    }
}

/// Key for impact analysis — one entry per (file, target function) pair.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImpactKey {
    pub file_path: String,
    pub function_name: String,
}

impl ImpactKey {
    pub fn new(file_path: &str, function_name: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
            function_name: function_name.to_string(),
        }
    }
}

// ── Cache ─────────────────────────────────────────────────────────────────────

/// In-memory cache for all Module 3 analysis results.
/// Lives only for the duration of the process — intentionally no persistence.
///
/// Two separate stores because the key shapes differ:
///   - `file_store`   : file-level analyses  (dead code / call graph / dependency)
///   - `impact_store` : per-function impact reports
#[derive(Debug, Default)]
pub struct AnalysisCache {
    file_store: HashMap<FileAnalysisKey, AnalysisResult>,
    impact_store: HashMap<ImpactKey, ImpactReport>,
}

impl AnalysisCache {
    pub fn new() -> Self {
        Self::default()
    }

    // ── File-level analyses ──────────────────────────────────────────────────

    pub fn get_file_result(&self, key: &FileAnalysisKey) -> Option<&AnalysisResult> {
        self.file_store.get(key)
    }

    pub fn insert_file_result(&mut self, key: FileAnalysisKey, result: AnalysisResult) {
        self.file_store.insert(key, result);
    }

    // ── Impact analysis ──────────────────────────────────────────────────────

    pub fn get_impact(&self, key: &ImpactKey) -> Option<&ImpactReport> {
        self.impact_store.get(key)
    }

    pub fn insert_impact(&mut self, key: ImpactKey, report: ImpactReport) {
        self.impact_store.insert(key, report);
    }

    // ── Invalidation ────────────────────────────────────────────────────────

    /// Invalidate ALL cached results for a given file.
    /// Call this whenever Module 2 signals that a file's graph has been updated.
    pub fn invalidate_file(&mut self, file_path: &str) {
        self.file_store
            .retain(|key, _| key.file_path != file_path);

        self.impact_store
            .retain(|key, _| key.file_path != file_path);
    }

    // ── Diagnostics ─────────────────────────────────────────────────────────

    pub fn file_result_count(&self) -> usize {
        self.file_store.len()
    }

    pub fn impact_result_count(&self) -> usize {
        self.impact_store.len()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::analysis_result::{AnalysisPayload, AnalysisResult, AnalysisType};
    use crate::models::impact_report::ImpactReport;

    fn dummy_result(analysis_type: AnalysisType) -> AnalysisResult {
        AnalysisResult::new(analysis_type, AnalysisPayload::empty())
    }

    #[test]
    fn test_insert_and_retrieve_file_result() {
        let mut cache = AnalysisCache::new();
        let key = FileAnalysisKey::new("dead_code", "auth.py");
        let result = dummy_result(AnalysisType::DeadCode);

        cache.insert_file_result(key.clone(), result);

        assert!(cache.get_file_result(&key).is_some());
        assert_eq!(
            cache.get_file_result(&key).unwrap().analysis_type,
            AnalysisType::DeadCode
        );
    }

    #[test]
    fn test_insert_and_retrieve_impact_result() {
        let mut cache = AnalysisCache::new();
        let key = ImpactKey::new("orders.py", "process_order");
        let report = ImpactReport::empty("process_order", "orders.py");

        cache.insert_impact(key.clone(), report);

        assert!(cache.get_impact(&key).is_some());
        assert_eq!(
            cache.get_impact(&key).unwrap().target_symbol,
            "process_order"
        );
    }

    #[test]
    fn test_invalidate_file_removes_all_its_entries() {
        let mut cache = AnalysisCache::new();

        // Insert two results for auth.py
        cache.insert_file_result(
            FileAnalysisKey::new("dead_code", "auth.py"),
            dummy_result(AnalysisType::DeadCode),
        );
        cache.insert_file_result(
            FileAnalysisKey::new("call_graph", "auth.py"),
            dummy_result(AnalysisType::CallGraph),
        );
        // Insert one result for a different file
        cache.insert_file_result(
            FileAnalysisKey::new("dead_code", "main.py"),
            dummy_result(AnalysisType::DeadCode),
        );
        // Insert an impact result for auth.py
        cache.insert_impact(
            ImpactKey::new("auth.py", "login"),
            ImpactReport::empty("login", "auth.py"),
        );

        assert_eq!(cache.file_result_count(), 3);
        assert_eq!(cache.impact_result_count(), 1);

        cache.invalidate_file("auth.py");

        // auth.py entries gone, main.py entry survives
        assert_eq!(cache.file_result_count(), 1);
        assert_eq!(cache.impact_result_count(), 0);
        assert!(cache
            .get_file_result(&FileAnalysisKey::new("dead_code", "main.py"))
            .is_some());
    }

    #[test]
    fn test_miss_returns_none() {
        let cache = AnalysisCache::new();
        let key = FileAnalysisKey::new("dead_code", "nonexistent.py");
        assert!(cache.get_file_result(&key).is_none());
    }

    #[test]
    fn test_overwrite_updates_value() {
        let mut cache = AnalysisCache::new();
        let key = FileAnalysisKey::new("call_graph", "app.py");

        cache.insert_file_result(key.clone(), dummy_result(AnalysisType::CallGraph));
        cache.insert_file_result(key.clone(), dummy_result(AnalysisType::CallGraph));

        // Should still be exactly one entry
        assert_eq!(cache.file_result_count(), 1);
    }
}