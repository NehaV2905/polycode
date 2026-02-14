// Library interface for Module 2
pub mod api;
pub mod graph;
pub mod grpc_client;
pub mod ir;
pub mod language_detector;

// Re-export main types
pub use api::{GraphQuery};
pub use graph::{GraphBuilder, IRGraph};
pub use ir::{EdgeType, IREdge, IRNode, NodeId, NodeType};
pub use language_detector::{detect_language, Language};
