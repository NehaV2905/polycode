use anyhow::Result;
use indexmap::IndexMap;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Direction;
use std::collections::HashMap;

use crate::ir::{EdgeType, IREdge, IRNode, NodeId};

/// The main IR graph storage structure.
/// Uses petgraph for efficient graph operations and maintains indexes for quick lookups.
#[derive(Debug)]
pub struct IRGraph {
    /// The underlying directed graph (petgraph)
    graph: DiGraph<IRNode, IREdge>,

    /// Map from NodeId (UUID) to petgraph NodeIndex for O(1) lookup
    node_index: HashMap<NodeId, NodeIndex>,

    /// Map from file path to list of nodes in that file (for incremental updates)
    file_nodes: IndexMap<String, Vec<NodeId>>,

    /// Map from (file_path, symbol_name) to NodeId for symbol resolution
    symbol_table: HashMap<(String, String), NodeId>,
}

impl IRGraph {
    /// Create a new empty IR graph
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_index: HashMap::new(),
            file_nodes: IndexMap::new(),
            symbol_table: HashMap::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: IRNode) -> Result<NodeId> {
        let node_id = node.id;
        let file_path = node.metadata.file_path.clone();
        let symbol_name = node.display_name();

        // Add to petgraph
        let idx = self.graph.add_node(node);

        // Update indexes
        self.node_index.insert(node_id, idx);

        // Track file ownership
        self.file_nodes
            .entry(file_path.clone())
            .or_insert_with(Vec::new)
            .push(node_id);

        // Update symbol table
        self.symbol_table
            .insert((file_path, symbol_name), node_id);

        Ok(node_id)
    }

    /// Add an edge between two nodes
    pub fn add_edge(&mut self, edge: IREdge) -> Result<()> {
        let from_idx = self
            .node_index
            .get(&edge.from)
            .ok_or_else(|| anyhow::anyhow!("Source node not found: {:?}", edge.from))?;

        let to_idx = self
            .node_index
            .get(&edge.to)
            .ok_or_else(|| anyhow::anyhow!("Target node not found: {:?}", edge.to))?;

        self.graph.add_edge(*from_idx, *to_idx, edge);
        Ok(())
    }

    /// Get a node by its ID
    pub fn get_node(&self, node_id: &NodeId) -> Option<&IRNode> {
        let idx = self.node_index.get(node_id)?;
        self.graph.node_weight(*idx)
    }

    /// Get a mutable reference to a node
    pub fn get_node_mut(&mut self, node_id: &NodeId) -> Option<&mut IRNode> {
        let idx = self.node_index.get(node_id)?;
        self.graph.node_weight_mut(*idx)
    }

    /// Look up a symbol by name in a specific file
    pub fn lookup_symbol(&self, file_path: &str, symbol_name: &str) -> Option<NodeId> {
        self.symbol_table
            .get(&(file_path.to_string(), symbol_name.to_string()))
            .copied()
    }

    /// Look up a symbol by name across all files (cross-file call resolution).
    /// Returns the first match found in any file.
    pub fn lookup_symbol_global(&self, symbol_name: &str) -> Option<NodeId> {
        self.symbol_table
            .iter()
            .find(|((_, name), _)| name == symbol_name)
            .map(|(_, id)| *id)
    }

    /// Get all nodes in a specific file
    pub fn get_file_nodes(&self, file_path: &str) -> Vec<&IRNode> {
        self.file_nodes
            .get(file_path)
            .map(|node_ids| {
                node_ids
                    .iter()
                    .filter_map(|id| self.get_node(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Remove all nodes from a specific file (for incremental updates)
    pub fn clear_file(&mut self, file_path: &str) -> Result<()> {
        if let Some(node_ids) = self.file_nodes.shift_remove(file_path) {
            // Collect indices and sort in reverse order to avoid index invalidation
            let mut indices_to_remove: Vec<_> = node_ids
                .iter()
                .filter_map(|node_id| {
                    self.node_index.remove(node_id).map(|idx| (idx, *node_id))
                })
                .collect();

            // Sort by index in descending order (highest first)
            indices_to_remove.sort_by(|a, b| b.0.index().cmp(&a.0.index()));

            // Remove nodes from graph in reverse order
            for (idx, _) in indices_to_remove {
                self.graph.remove_node(idx);
            }
        }

        // Clean up symbol table
        self.symbol_table
            .retain(|(path, _), _| path != file_path);

        Ok(())
    }

    /// Get all outgoing edges from a node
    pub fn get_outgoing_edges(&self, node_id: &NodeId) -> Vec<&IREdge> {
        let idx = match self.node_index.get(node_id) {
            Some(idx) => idx,
            None => return Vec::new(),
        };

        self.graph
            .edges_directed(*idx, Direction::Outgoing)
            .map(|e| e.weight())
            .collect()
    }

    /// Get all incoming edges to a node
    pub fn get_incoming_edges(&self, node_id: &NodeId) -> Vec<&IREdge> {
        let idx = match self.node_index.get(node_id) {
            Some(idx) => idx,
            None => return Vec::new(),
        };

        self.graph
            .edges_directed(*idx, Direction::Incoming)
            .map(|e| e.weight())
            .collect()
    }

    /// Get all nodes (for iteration)
    pub fn all_nodes(&self) -> impl Iterator<Item = &IRNode> {
        self.graph.node_weights()
    }

    /// Get all edges (for iteration)
    pub fn all_edges(&self) -> impl Iterator<Item = &IREdge> {
        self.graph.edge_weights()
    }

    /// Get graph statistics
    pub fn stats(&self) -> GraphStats {
        GraphStats {
            node_count: self.graph.node_count(),
            edge_count: self.graph.edge_count(),
            file_count: self.file_nodes.len(),
        }
    }
}

impl Default for IRGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct GraphStats {
    pub node_count: usize,
    pub edge_count: usize,
    pub file_count: usize,
}
