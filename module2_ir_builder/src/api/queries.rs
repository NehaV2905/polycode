use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::graph::IRGraph;
use crate::ir::{EdgeType, IRNode, NodeId, NodeType};

/// Query interface for the IR graph.
/// This is what Module 3 (Analysis Engine) will use.
pub struct GraphQuery<'a> {
    graph: &'a IRGraph,
}

impl<'a> GraphQuery<'a> {
    /// Create a new graph query interface
    pub fn new(graph: &'a IRGraph) -> Self {
        Self { graph }
    }

    /// Find all functions that call a specific function
    pub fn find_callers(&self, function_name: &str, file_path: &str) -> Vec<FunctionInfo> {
        let target_id = match self.graph.lookup_symbol(file_path, function_name) {
            Some(id) => id,
            None => return Vec::new(),
        };

        self.graph
            .get_incoming_edges(&target_id)
            .iter()
            .filter_map(|edge| {
                if matches!(edge.edge_type, EdgeType::Calls { .. }) {
                    self.graph.get_node(&edge.from).and_then(|node| {
                        if matches!(node.node_type, NodeType::Function { .. }) {
                            Some(FunctionInfo::from_node(node, edge.line_number))
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Find all functions called by a specific function
    pub fn find_callees(&self, function_name: &str, file_path: &str) -> Vec<FunctionInfo> {
        let caller_id = match self.graph.lookup_symbol(file_path, function_name) {
            Some(id) => id,
            None => return Vec::new(),
        };

        self.graph
            .get_outgoing_edges(&caller_id)
            .iter()
            .filter_map(|edge| {
                if matches!(edge.edge_type, EdgeType::Calls { .. }) {
                    self.graph.get_node(&edge.to).and_then(|node| {
                        if matches!(node.node_type, NodeType::Function { .. }) {
                            Some(FunctionInfo::from_node(node, edge.line_number))
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Find all modules imported by a specific file
    pub fn find_dependencies(&self, file_path: &str) -> Vec<DependencyInfo> {
        let file_nodes = self.graph.get_file_nodes(file_path);

        let mut dependencies = Vec::new();

        for node in file_nodes {
            for edge in self.graph.get_outgoing_edges(&node.id) {
                if let EdgeType::Imports { imported_names, is_wildcard } = &edge.edge_type {
                    if let Some(target) = self.graph.get_node(&edge.to) {
                        if let NodeType::Module { file_path: module_path, .. } = &target.node_type {
                            dependencies.push(DependencyInfo {
                                module_path: module_path.clone(),
                                imported_names: imported_names.clone(),
                                is_wildcard: *is_wildcard,
                                line_number: edge.line_number,
                            });
                        }
                    }
                }
            }
        }

        dependencies
    }

    /// Find all modules that import a specific module
    pub fn find_dependents(&self, module_path: &str) -> Vec<String> {
        let module_id = match self.graph.lookup_symbol(module_path, module_path) {
            Some(id) => id,
            None => return Vec::new(),
        };

        self.graph
            .get_incoming_edges(&module_id)
            .iter()
            .filter_map(|edge| {
                if matches!(edge.edge_type, EdgeType::Imports { .. }) {
                    self.graph
                        .get_node(&edge.from)
                        .map(|node| node.metadata.file_path.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Find all functions that are never called (unused functions)
    pub fn find_unused_functions(&self, file_path: &str) -> Vec<FunctionInfo> {
        self.graph
            .get_file_nodes(file_path)
            .iter()
            .filter(|node| {
                matches!(node.node_type, NodeType::Function { .. })
            })
            .filter(|node| {
                // A function is unused if it has no incoming Calls edges
                // AND no incoming AccessesMember edges (method calls like self.foo() or obj.foo())
                self.graph
                    .get_incoming_edges(&node.id)
                    .iter()
                    .all(|edge| !matches!(
                        edge.edge_type,
                        EdgeType::Calls { .. } | EdgeType::AccessesMember { .. }
                    ))
            })
            .map(|node| FunctionInfo::from_node(node, node.metadata.line_number))
            .collect()
    }

    /// Get all functions in a file
    pub fn get_functions(&self, file_path: &str) -> Vec<FunctionInfo> {
        self.graph
            .get_file_nodes(file_path)
            .iter()
            .filter_map(|node| {
                if matches!(node.node_type, NodeType::Function { .. }) {
                    Some(FunctionInfo::from_node(node, node.metadata.line_number))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get all classes in a file
    pub fn get_classes(&self, file_path: &str) -> Vec<ClassInfo> {
        self.graph
            .get_file_nodes(file_path)
            .iter()
            .filter_map(|node| {
                if let NodeType::Class { name, base_classes } = &node.node_type {
                    Some(ClassInfo {
                        name: name.clone(),
                        base_classes: base_classes.clone(),
                        line_number: node.metadata.line_number,
                        file_path: node.metadata.file_path.clone(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Find the class hierarchy (all subclasses of a class)
    pub fn find_subclasses(&self, class_name: &str, file_path: &str) -> Vec<ClassInfo> {
        let class_id = match self.graph.lookup_symbol(file_path, class_name) {
            Some(id) => id,
            None => return Vec::new(),
        };

        self.graph
            .get_incoming_edges(&class_id)
            .iter()
            .filter_map(|edge| {
                if matches!(edge.edge_type, EdgeType::InheritsFrom) {
                    self.graph.get_node(&edge.from).and_then(|node| {
                        if let NodeType::Class { name, base_classes } = &node.node_type {
                            Some(ClassInfo {
                                name: name.clone(),
                                base_classes: base_classes.clone(),
                                line_number: node.metadata.line_number,
                                file_path: node.metadata.file_path.clone(),
                            })
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get graph statistics
    pub fn get_stats(&self) -> GraphStats {
        let stats = self.graph.stats();
        GraphStats {
            total_nodes: stats.node_count,
            total_edges: stats.edge_count,
            total_files: stats.file_count,
        }
    }

    /// Export the graph to JSON
    pub fn export_to_json(&self) -> Result<String> {
        let export = GraphExport {
            nodes: self.graph.all_nodes().cloned().collect(),
            edges: self.graph.all_edges().cloned().collect(),
        };

        serde_json::to_string_pretty(&export).map_err(Into::into)
    }
}

// ========== Query Result Types ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub param_count: i32,
    pub is_async: bool,
    pub parent_scope: Option<String>,
    pub line_number: i32,
    pub file_path: String,
}

impl FunctionInfo {
    fn from_node(node: &IRNode, line_number: i32) -> Self {
        match &node.node_type {
            NodeType::Function {
                name,
                param_count,
                is_async,
                parent_scope,
            } => Self {
                name: name.clone(),
                param_count: *param_count,
                is_async: *is_async,
                parent_scope: parent_scope.clone(),
                line_number,
                file_path: node.metadata.file_path.clone(),
            },
            _ => panic!("Not a function node"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassInfo {
    pub name: String,
    pub base_classes: Vec<String>,
    pub line_number: i32,
    pub file_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyInfo {
    pub module_path: String,
    pub imported_names: Vec<String>,
    pub is_wildcard: bool,
    pub line_number: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStats {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub total_files: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct GraphExport {
    nodes: Vec<crate::ir::IRNode>,
    edges: Vec<crate::ir::IREdge>,
}
