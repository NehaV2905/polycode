use anyhow::{Context, Result};
use tracing::{debug, warn};

use super::ir_events::{self, ir_event::Event};
use crate::graph::GraphBuilder;

/// Process a single IR event and update the graph
pub fn process_event(builder: &mut GraphBuilder, event: ir_events::IrEvent) -> Result<()> {
    // Extract metadata
    let metadata = event
        .metadata
        .as_ref()
        .context("Event missing metadata")?;

    let timestamp = metadata
        .timestamp
        .as_ref()
        .map(|ts| ts.seconds)
        .unwrap_or(0);

    // Process based on event type
    match event.event {
        Some(Event::FunctionDeclared(e)) => {
            debug!("Processing FunctionDeclared: {}", e.name);
            builder.process_function_declared(
                e.name,
                e.param_count,
                e.line_number,
                if e.parent_scope.is_empty() {
                    None
                } else {
                    Some(e.parent_scope)
                },
                timestamp,
            )?;
        }

        Some(Event::AsyncFunctionDeclared(e)) => {
            debug!("Processing AsyncFunctionDeclared: {}", e.name);
            builder.process_async_function_declared(
                e.name,
                e.param_count,
                e.line_number,
                if e.parent_scope.is_empty() {
                    None
                } else {
                    Some(e.parent_scope)
                },
                timestamp,
            )?;
        }

        Some(Event::ClassDeclared(e)) => {
            debug!("Processing ClassDeclared: {}", e.name);
            builder.process_class_declared(
                e.name,
                e.base_classes,
                e.line_number,
                timestamp,
            )?;
        }

        Some(Event::FunctionCall(e)) => {
            debug!("Processing FunctionCall: {}", e.callee_name);
            builder.process_function_call(
                if e.caller_function.is_empty() {
                    None
                } else {
                    Some(e.caller_function)
                },
                e.callee_name,
                e.arg_count,
                e.line_number,
            )?;
        }

        Some(Event::ImportStatement(e)) => {
            debug!("Processing ImportStatement: {}", e.module_name);
            builder.process_import(
                e.module_name,
                e.imported_names,
                e.is_wildcard,
                e.line_number,
                timestamp,
            )?;
        }

        Some(Event::VariableAssignment(e)) => {
            debug!("Processing VariableAssignment: {}", e.variable_name);
            builder.process_variable_assignment(
                e.variable_name,
                e.scope,
                e.line_number,
                timestamp,
            )?;
        }

        Some(Event::ControlStructure(e)) => {
            debug!("Processing ControlStructure: type {}", e.r#type);
            builder.process_control_structure(
                e.r#type,
                if e.parent_function.is_empty() {
                    None
                } else {
                    Some(e.parent_function)
                },
                e.line_number,
                timestamp,
            )?;
        }

        Some(Event::InterfaceDeclared(e)) => {
            debug!("Processing InterfaceDeclared: {}", e.name);
            builder.process_interface_declared(
                e.name,
                e.base_interfaces,
                e.line_number,
                e.method_count,
                timestamp,
            )?;
        }

        Some(Event::EnumDeclared(e)) => {
            debug!("Processing EnumDeclared: {}", e.name);
            builder.process_enum_declared(
                e.name,
                e.member_count,
                e.line_number,
                timestamp,
            )?;
        }

        Some(Event::ReturnStatement(_)) => {
            // TODO: Track return statements if needed
            debug!("Skipping ReturnStatement (not implemented yet)");
        }

        Some(Event::ThrowStatement(_)) => {
            // TODO: Track exception throws
            debug!("Skipping ThrowStatement (not implemented yet)");
        }

        Some(Event::CatchClause(_)) => {
            // TODO: Track exception handlers
            debug!("Skipping CatchClause (not implemented yet)");
        }

        Some(Event::AwaitExpression(_)) => {
            // TODO: Track await expressions
            debug!("Skipping AwaitExpression (not implemented yet)");
        }

        Some(Event::LambdaDeclared(_)) => {
            // TODO: Track lambda functions
            debug!("Skipping LambdaDeclared (not implemented yet)");
        }

        Some(Event::MemberAccess(_)) => {
            // TODO: Track member access
            debug!("Skipping MemberAccess (not implemented yet)");
        }

        None => {
            warn!("Received event with no event type");
        }
    }

    Ok(())
}
