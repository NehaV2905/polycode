use serde::{Deserialize, Serialize};

/// The type of node in the IR graph.
/// This is language-agnostic - Python, Java, and Go all map to these types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeType {
    /// A source file/module (e.g., main.py, Main.java, main.go)
    Module {
        file_path: String,
        language: String,
    },

    /// A function or method declaration
    Function {
        name: String,
        param_count: i32,
        is_async: bool,
        parent_scope: Option<String>,
    },

    /// A class definition
    Class {
        name: String,
        base_classes: Vec<String>,
    },

    /// A variable assignment
    Variable {
        name: String,
        scope: String,
    },

    /// An interface/trait/protocol declaration
    Interface {
        name: String,
        base_interfaces: Vec<String>,
        method_count: i32,
    },

    /// An enum declaration
    Enum {
        name: String,
        member_count: i32,
    },

    /// A lambda/anonymous function
    Lambda {
        param_count: i32,
        parent_function: Option<String>,
    },

    /// A control flow structure (if/while/for/try)
    ControlFlow {
        flow_type: ControlFlowType,
        parent_function: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlFlowType {
    If,
    While,
    For,
    Switch,
    Try,
}

impl ControlFlowType {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(ControlFlowType::If),
            1 => Some(ControlFlowType::While),
            2 => Some(ControlFlowType::For),
            3 => Some(ControlFlowType::Switch),
            4 => Some(ControlFlowType::Try),
            _ => None,
        }
    }
}
