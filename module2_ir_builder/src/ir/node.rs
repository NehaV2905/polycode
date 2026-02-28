use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::types::NodeType;

/// Unique identifier for a node in the IR graph
pub type NodeId = Uuid;

/// Metadata associated with a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetadata {
    /// Line number in the source file
    pub line_number: i32,

    /// Timestamp when this node was created/updated (Unix timestamp in seconds)
    pub timestamp: i64,

    /// Source file path
    pub file_path: String,

    /// Annotations or decorators on this node (e.g. ["@Bean", "@Override"] for Java)
    pub decorators: Vec<String>,

    /// Additional custom metadata (extensible)
    pub custom: HashMap<String, String>,
}

/// A node in the IR graph representing a program element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRNode {
    /// Unique identifier for this node
    pub id: NodeId,

    /// The type and data of this node
    pub node_type: NodeType,

    /// Metadata about this node
    pub metadata: NodeMetadata,
}

impl IRNode {
    /// Create a new IR node
    pub fn new(node_type: NodeType, metadata: NodeMetadata) -> Self {
        Self {
            id: Uuid::new_v4(),
            node_type,
            metadata,
        }
    }

    /// Get a human-readable name for this node
    pub fn display_name(&self) -> String {
        match &self.node_type {
            NodeType::Module { file_path, .. } => {
                file_path.split('/').last().unwrap_or(file_path).to_string()
            }
            NodeType::Function { name, .. } => name.clone(),
            NodeType::Class { name, .. } => name.clone(),
            NodeType::Variable { name, .. } => name.clone(),
            NodeType::Interface { name, .. } => name.clone(),
            NodeType::Enum { name, .. } => name.clone(),
            NodeType::Lambda { parent_function, .. } => {
                format!("λ@{}", parent_function.as_deref().unwrap_or("global"))
            }
            NodeType::ControlFlow { flow_type, .. } => format!("{:?}", flow_type),
        }
    }

    /// Get the scope of this node (for scoped lookups)
    pub fn scope(&self) -> Option<String> {
        match &self.node_type {
            NodeType::Function { parent_scope, .. } => parent_scope.clone(),
            NodeType::Variable { scope, .. } => Some(scope.clone()),
            NodeType::Lambda { parent_function, .. } => parent_function.clone(),
            NodeType::ControlFlow { parent_function, .. } => parent_function.clone(),
            _ => None,
        }
    }
}
