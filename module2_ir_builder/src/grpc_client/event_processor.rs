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

        Some(Event::ReturnStatement(e)) => {
            debug!("Processing ReturnStatement in: {}", e.function_name);
            builder.process_return_statement(
                e.function_name,
                e.has_value,
                e.line_number,
            )?;
        }

        Some(Event::ThrowStatement(e)) => {
            debug!("Processing ThrowStatement: {}", e.exception_type);
            builder.process_throw_statement(
                if e.exception_type.is_empty() {
                    None
                } else {
                    Some(e.exception_type)
                },
                if e.parent_function.is_empty() {
                    None
                } else {
                    Some(e.parent_function)
                },
                e.line_number,
                e.has_message,
            )?;
        }

        Some(Event::CatchClause(e)) => {
            debug!("Processing CatchClause in: {}", e.parent_function);
            builder.process_catch_clause(
                e.exception_types,
                if e.parent_function.is_empty() {
                    None
                } else {
                    Some(e.parent_function)
                },
                e.line_number,
                e.is_catch_all,
            )?;
        }

        Some(Event::AwaitExpression(e)) => {
            debug!("Processing AwaitExpression: {}", e.awaited_function);
            builder.process_await_expression(
                e.awaited_function,
                if e.parent_function.is_empty() {
                    None
                } else {
                    Some(e.parent_function)
                },
                e.line_number,
            )?;
        }

        Some(Event::LambdaDeclared(e)) => {
            debug!("Processing LambdaDeclared");
            builder.process_lambda_declared(
                e.param_count,
                if e.parent_function.is_empty() {
                    None
                } else {
                    Some(e.parent_function)
                },
                e.line_number,
                timestamp,
            )?;
        }

        Some(Event::MemberAccess(e)) => {
            debug!("Processing MemberAccess: {}.{}", e.object_name, e.member_name);
            builder.process_member_access(
                e.object_name,
                e.member_name,
                if e.parent_function.is_empty() {
                    None
                } else {
                    Some(e.parent_function)
                },
                e.line_number,
                e.is_method_call,
            )?;
        }

        None => {
            warn!("Received event with no event type");
        }
    }

    Ok(())
}
