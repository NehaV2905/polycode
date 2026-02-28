use anyhow::{Context, Result};
use std::collections::HashMap;
use tracing::{debug, warn};

use crate::ir::{EdgeType, IREdge, IRNode, NodeId, NodeMetadata, NodeType};
use crate::ir::types::ControlFlowType;

use super::storage::IRGraph;

/// The GraphBuilder processes IR events and builds the graph incrementally.
/// It maintains context about the current file being processed and handles
/// symbol resolution across scopes.
pub struct GraphBuilder {
    /// The graph being built
    graph: IRGraph,

    /// Current file context (updated when processing a file)
    current_file: Option<String>,

    /// Current language context
    current_language: Option<String>,

    /// Stack of current scopes (for tracking nested functions/classes)
    scope_stack: Vec<String>,

    /// Temporary map of unresolved function calls (caller -> [(callee_name, line)])
    /// These are resolved after all functions are declared
    unresolved_calls: HashMap<String, Vec<(String, i32)>>,
}

impl GraphBuilder {
    /// Create a new graph builder
    pub fn new() -> Self {
        Self {
            graph: IRGraph::new(),
            current_file: None,
            current_language: None,
            scope_stack: Vec::new(),
            unresolved_calls: HashMap::new(),
        }
    }

    /// Set the current file context (call this when starting to process a new file)
    pub fn set_current_file(&mut self, file_path: String, language: String) {
        debug!("Processing file: {} ({})", file_path, language);
        self.current_file = Some(file_path.clone());
        self.current_language = Some(language.clone());
        self.scope_stack.clear();
        self.unresolved_calls.clear();

        // Ensure a Module node exists for the current file
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let _ = self.ensure_module_node(&file_path, timestamp);
    }

    /// Clear the current file (removes all nodes from the file, for incremental updates)
    pub fn clear_current_file(&mut self) -> Result<()> {
        if let Some(ref file_path) = self.current_file {
            self.graph.clear_file(file_path)?;
        }
        Ok(())
    }

    /// Process a FunctionDeclared event
    pub fn process_function_declared(
        &mut self,
        name: String,
        param_count: i32,
        line_number: i32,
        parent_scope: Option<String>,
        timestamp: i64,
    ) -> Result<NodeId> {
        let metadata = self.create_metadata(line_number, timestamp)?;

        let node = IRNode::new(
            NodeType::Function {
                name: name.clone(),
                param_count,
                is_async: false,
                parent_scope: parent_scope.clone(),
            },
            metadata,
        );

        let node_id = self.graph.add_node(node)?;

        // Add to scope stack
        self.scope_stack.push(name.clone());

        // If there's a parent scope, create a HasMember edge
        if let Some(parent) = parent_scope {
            self.link_to_parent(&parent, node_id, line_number)?;
        }

        debug!("Declared function: {} (ID: {})", name, node_id);
        Ok(node_id)
    }

    /// Process an AsyncFunctionDeclared event
    pub fn process_async_function_declared(
        &mut self,
        name: String,
        param_count: i32,
        line_number: i32,
        parent_scope: Option<String>,
        timestamp: i64,
    ) -> Result<NodeId> {
        let metadata = self.create_metadata(line_number, timestamp)?;

        let node = IRNode::new(
            NodeType::Function {
                name: name.clone(),
                param_count,
                is_async: true,
                parent_scope: parent_scope.clone(),
            },
            metadata,
        );

        let node_id = self.graph.add_node(node)?;

        self.scope_stack.push(name.clone());

        if let Some(parent) = parent_scope {
            self.link_to_parent(&parent, node_id, line_number)?;
        }

        debug!("Declared async function: {} (ID: {})", name, node_id);
        Ok(node_id)
    }

    /// Process a ClassDeclared event
    pub fn process_class_declared(
        &mut self,
        name: String,
        base_classes: Vec<String>,
        line_number: i32,
        timestamp: i64,
    ) -> Result<NodeId> {
        let metadata = self.create_metadata(line_number, timestamp)?;

        let node = IRNode::new(
            NodeType::Class {
                name: name.clone(),
                base_classes: base_classes.clone(),
            },
            metadata,
        );

        let node_id = self.graph.add_node(node)?;

        // Create InheritsFrom edges for base classes
        for base_class in base_classes {
            if let Some(base_id) = self.lookup_symbol(&base_class) {
                let edge = IREdge::new(node_id, base_id, EdgeType::InheritsFrom, line_number);
                self.graph.add_edge(edge)?;
            }
        }

        self.scope_stack.push(name.clone());

        debug!("Declared class: {} (ID: {})", name, node_id);
        Ok(node_id)
    }

    /// Process a FunctionCall event
    pub fn process_function_call(
        &mut self,
        caller_function: Option<String>,
        callee_name: String,
        arg_count: i32,
        line_number: i32,
    ) -> Result<()> {
        // Try to resolve both caller and callee
        let caller_id = match caller_function {
            Some(ref name) => self.lookup_symbol(name),
            None => None,
        };

        let callee_id = self.lookup_symbol(&callee_name);

        match (caller_id, callee_id) {
            (Some(caller), Some(callee)) => {
                // Both resolved - create edge immediately
                let edge = IREdge::new(
                    caller,
                    callee,
                    EdgeType::Calls { arg_count },
                    line_number,
                );
                self.graph.add_edge(edge)?;
                debug!("Linked call: {} -> {}", caller_function.unwrap(), callee_name);
            }
            (Some(caller), None) => {
                // Caller exists but callee not found - save for later resolution
                if let Some(ref caller_name) = caller_function {
                    self.unresolved_calls
                        .entry(caller_name.clone())
                        .or_insert_with(Vec::new)
                        .push((callee_name.clone(), line_number));
                }
                debug!("Unresolved call: {} (will retry later)", callee_name);
            }
            _ => {
                warn!("Cannot resolve function call: {:?} -> {}", caller_function, callee_name);
            }
        }

        Ok(())
    }

    /// Process an ImportStatement event
    pub fn process_import(
        &mut self,
        module_name: String,
        imported_names: Vec<String>,
        is_wildcard: bool,
        line_number: i32,
        timestamp: i64,
    ) -> Result<()> {
        // Create or lookup the imported module node
        let module_id = self.ensure_module_node(&module_name, timestamp)?;

        // Link current file to imported module.
        // The module node is stored under its basename (display_name), not the full path,
        // so we look it up by basename.
        if let Some(ref file_path) = self.current_file.clone() {
            let file_basename = file_path
                .split('/')
                .last()
                .unwrap_or(file_path.as_str())
                .to_string();
            if let Some(current_file_id) = self.lookup_symbol(&file_basename) {
                let edge = IREdge::new(
                    current_file_id,
                    module_id,
                    EdgeType::Imports {
                        imported_names,
                        is_wildcard,
                    },
                    line_number,
                );
                self.graph.add_edge(edge)?;
                debug!("Linked import: {} -> {}", file_path, module_name);
            }
        }

        Ok(())
    }

    /// Process a VariableAssignment event
    pub fn process_variable_assignment(
        &mut self,
        variable_name: String,
        scope: String,
        line_number: i32,
        timestamp: i64,
    ) -> Result<NodeId> {
        let metadata = self.create_metadata(line_number, timestamp)?;

        let node = IRNode::new(
            NodeType::Variable {
                name: variable_name.clone(),
                scope: scope.clone(),
            },
            metadata,
        );

        let node_id = self.graph.add_node(node)?;

        // Link to parent scope
        self.link_to_parent(&scope, node_id, line_number)?;

        debug!("Declared variable: {} in scope {}", variable_name, scope);
        Ok(node_id)
    }

    /// Process a ControlStructure event
    pub fn process_control_structure(
        &mut self,
        control_type: i32,
        parent_function: Option<String>,
        line_number: i32,
        timestamp: i64,
    ) -> Result<NodeId> {
        let metadata = self.create_metadata(line_number, timestamp)?;

        let flow_type = ControlFlowType::from_i32(control_type)
            .context("Invalid control flow type")?;

        let node = IRNode::new(
            NodeType::ControlFlow {
                flow_type,
                parent_function: parent_function.clone(),
            },
            metadata,
        );

        let node_id = self.graph.add_node(node)?;

        // Link to parent function
        if let Some(parent) = parent_function {
            self.link_to_parent(&parent, node_id, line_number)?;
        }

        Ok(node_id)
    }

    /// Process an InterfaceDeclared event
    pub fn process_interface_declared(
        &mut self,
        name: String,
        base_interfaces: Vec<String>,
        line_number: i32,
        method_count: i32,
        timestamp: i64,
    ) -> Result<NodeId> {
        let metadata = self.create_metadata(line_number, timestamp)?;

        let node = IRNode::new(
            NodeType::Interface {
                name: name.clone(),
                base_interfaces,
                method_count,
            },
            metadata,
        );

        let node_id = self.graph.add_node(node)?;
        debug!("Declared interface: {} (ID: {})", name, node_id);
        Ok(node_id)
    }

    /// Process an EnumDeclared event
    pub fn process_enum_declared(
        &mut self,
        name: String,
        member_count: i32,
        line_number: i32,
        timestamp: i64,
    ) -> Result<NodeId> {
        let metadata = self.create_metadata(line_number, timestamp)?;

        let node = IRNode::new(
            NodeType::Enum {
                name: name.clone(),
                member_count,
            },
            metadata,
        );

        let node_id = self.graph.add_node(node)?;
        debug!("Declared enum: {} (ID: {})", name, node_id);
        Ok(node_id)
    }

    /// Resolve pending unresolved function calls (call after processing all events in a file)
    pub fn resolve_pending_calls(&mut self) -> Result<()> {
        let unresolved = std::mem::take(&mut self.unresolved_calls);

        for (caller_name, calls) in unresolved {
            if let Some(caller_id) = self.lookup_symbol(&caller_name) {
                for (callee_name, line_number) in calls {
                    if let Some(callee_id) = self.lookup_symbol(&callee_name) {
                        let edge = IREdge::new(
                            caller_id,
                            callee_id,
                            EdgeType::Calls { arg_count: 0 }, // TODO: store arg_count
                            line_number,
                        );
                        self.graph.add_edge(edge)?;
                        debug!("Resolved call: {} -> {}", caller_name, callee_name);
                    }
                }
            }
        }

        Ok(())
    }

    /// Process a ReturnStatement event
    pub fn process_return_statement(
        &mut self,
        function_name: String,
        has_value: bool,
        line_number: i32,
    ) -> Result<()> {
        if let Some(function_id) = self.lookup_symbol(&function_name) {
            // We don't create a separate node for returns, just track them as a property
            // In the future, we could create a Returns edge to track return patterns
            debug!("Recorded return in function: {} (has_value: {})", function_name, has_value);
        }
        Ok(())
    }

    /// Process a ThrowStatement event
    pub fn process_throw_statement(
        &mut self,
        exception_type: Option<String>,
        parent_function: Option<String>,
        line_number: i32,
        has_message: bool,
    ) -> Result<()> {
        if let (Some(parent), Some(exc_type)) = (parent_function, exception_type) {
            if let Some(function_id) = self.lookup_symbol(&parent) {
                // In the future, create Throws edge to exception type
                debug!("Recorded throw in {}: {} (has_message: {})", parent, exc_type, has_message);
            }
        }
        Ok(())
    }

    /// Process a CatchClause event
    pub fn process_catch_clause(
        &mut self,
        exception_types: Vec<String>,
        parent_function: Option<String>,
        line_number: i32,
        is_catch_all: bool,
    ) -> Result<()> {
        if let Some(parent) = parent_function {
            if let Some(_function_id) = self.lookup_symbol(&parent) {
                // In the future, create Catches edge to exception types
                debug!(
                    "Recorded catch in {}: {:?} (catch_all: {})",
                    parent, exception_types, is_catch_all
                );
            }
        }
        Ok(())
    }

    /// Process an AwaitExpression event
    pub fn process_await_expression(
        &mut self,
        awaited_function: String,
        parent_function: Option<String>,
        line_number: i32,
    ) -> Result<()> {
        if let Some(parent) = parent_function {
            if let Some(caller_id) = self.lookup_symbol(&parent) {
                if let Some(callee_id) = self.lookup_symbol(&awaited_function) {
                    let edge = IREdge::new(caller_id, callee_id, EdgeType::Awaits, line_number);
                    self.graph.add_edge(edge)?;
                    debug!("Linked await: {} awaits {}", parent, awaited_function);
                }
            }
        }
        Ok(())
    }

    /// Process a LambdaDeclared event
    pub fn process_lambda_declared(
        &mut self,
        param_count: i32,
        parent_function: Option<String>,
        line_number: i32,
        timestamp: i64,
    ) -> Result<NodeId> {
        let metadata = self.create_metadata(line_number, timestamp)?;

        let node = IRNode::new(
            NodeType::Lambda {
                param_count,
                parent_function: parent_function.clone(),
            },
            metadata,
        );

        let node_id = self.graph.add_node(node)?;

        // Link to parent function if nested
        if let Some(parent) = parent_function {
            self.link_to_parent(&parent, node_id, line_number)?;
        }

        debug!("Declared lambda with {} params", param_count);
        Ok(node_id)
    }

    /// Process a MemberAccess event
    pub fn process_member_access(
        &mut self,
        object_name: String,
        member_name: String,
        parent_function: Option<String>,
        line_number: i32,
        is_method_call: bool,
    ) -> Result<()> {
        if let Some(ref parent) = parent_function {
            if let Some(caller_id) = self.lookup_symbol(parent) {
                // If this is a method call (e.g. self.foo() or obj.foo()) and the method
                // can be resolved in the current file, create an AccessesMember edge so
                // the method is not incorrectly flagged as unused dead code.
                if is_method_call {
                    if let Some(callee_id) = self.lookup_symbol(&member_name) {
                        let edge = IREdge::new(
                            caller_id,
                            callee_id,
                            EdgeType::AccessesMember {
                                member_name: member_name.clone(),
                                is_method_call: true,
                            },
                            line_number,
                        );
                        self.graph.add_edge(edge)?;
                        debug!("Linked method call: {}.{}()", parent, member_name);
                    }
                } else {
                    debug!(
                        "Recorded member access in {}: {}.{} (property)",
                        parent, object_name, member_name
                    );
                }
            }
        }
        Ok(())
    }

    /// Get a reference to the built graph
    pub fn graph(&self) -> &IRGraph {
        &self.graph
    }

    /// Get a mutable reference to the graph
    pub fn graph_mut(&mut self) -> &mut IRGraph {
        &mut self.graph
    }

    /// Consume the builder and return the graph
    pub fn into_graph(self) -> IRGraph {
        self.graph
    }

    // ========== Helper Methods ==========

    fn create_metadata(&self, line_number: i32, timestamp: i64) -> Result<NodeMetadata> {
        let file_path = self
            .current_file
            .as_ref()
            .context("No current file set")?
            .clone();

        Ok(NodeMetadata {
            line_number,
            timestamp,
            file_path,
            custom: HashMap::new(),
        })
    }

    fn lookup_symbol(&self, name: &str) -> Option<NodeId> {
        let file_path = self.current_file.as_ref()?;
        self.graph.lookup_symbol(file_path, name)
    }

    fn link_to_parent(
        &mut self,
        parent_name: &str,
        child_id: NodeId,
        line_number: i32,
    ) -> Result<()> {
        if let Some(parent_id) = self.lookup_symbol(parent_name) {
            let edge = IREdge::new(parent_id, child_id, EdgeType::HasMember, line_number);
            self.graph.add_edge(edge)?;
        }
        Ok(())
    }

    fn ensure_module_node(&mut self, module_name: &str, timestamp: i64) -> Result<NodeId> {
        // Check if module already exists
        if let Some(node_id) = self.lookup_symbol(module_name) {
            return Ok(node_id);
        }

        // Create new module node
        let metadata = NodeMetadata {
            line_number: 0,
            timestamp,
            file_path: module_name.to_string(),
            custom: HashMap::new(),
        };

        let language = self.current_language.clone().unwrap_or_else(|| "unknown".to_string());

        let node = IRNode::new(
            NodeType::Module {
                file_path: module_name.to_string(),
                language,
            },
            metadata,
        );

        self.graph.add_node(node)
    }
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}
