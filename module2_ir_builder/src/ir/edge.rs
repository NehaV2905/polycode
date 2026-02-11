use serde::{Deserialize, Serialize};

use super::node::NodeId;

/// The type of relationship between two nodes in the IR graph
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EdgeType {
    /// A function calls another function
    Calls {
        arg_count: i32,
    },

    /// A module imports another module
    Imports {
        imported_names: Vec<String>,
        is_wildcard: bool,
    },

    /// A class/module contains a member (function, variable, class)
    HasMember,

    /// A class inherits from another class
    InheritsFrom,

    /// A function returns from another context
    Returns {
        has_value: bool,
    },

    /// A function accesses a member (property/method)
    AccessesMember {
        member_name: String,
        is_method_call: bool,
    },

    /// A function throws an exception
    Throws {
        exception_type: Option<String>,
    },

    /// A function catches an exception
    Catches {
        exception_types: Vec<String>,
        is_catch_all: bool,
    },

    /// A function awaits another async function
    Awaits,

    /// A control structure is inside a function
    ContainedIn,
}

/// An edge in the IR graph representing a relationship between nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IREdge {
    /// Source node ID
    pub from: NodeId,

    /// Target node ID
    pub to: NodeId,

    /// The type of relationship
    pub edge_type: EdgeType,

    /// Line number where this relationship occurs
    pub line_number: i32,
}

impl IREdge {
    /// Create a new IR edge
    pub fn new(from: NodeId, to: NodeId, edge_type: EdgeType, line_number: i32) -> Self {
        Self {
            from,
            to,
            edge_type,
            line_number,
        }
    }
}
