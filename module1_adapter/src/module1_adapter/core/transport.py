"""
Transport Layer - gRPC Event Publisher

This module handles streaming IR events via gRPC.
It's a simple pipe: receive facts, emit events.
"""

import grpc
from concurrent import futures
import time
from typing import Iterator, List
from datetime import datetime

# Import generated protobuf classes
try:
    from module1_adapter.generated import ir_events_pb2
    from module1_adapter.generated import ir_events_pb2_grpc
except ImportError:
    # Fallback for standalone scripts
    import ir_events_pb2
    import ir_events_pb2_grpc


class IREventPublisher:
    """
    Publishes IR events to a gRPC stream.
    
    This is a dumb pipe. It converts facts to protobuf messages and sends them.
    """
    
    def __init__(self, language: str = "python"):
        self.language = language
        self.events_queue: List[ir_events_pb2.IREvent] = []
    
    def publish_fact(self, fact, file_path: str) -> ir_events_pb2.IREvent:
        """
        Convert an IRFact to an IREvent protobuf message.
        
        Args:
            fact: IRFact object from parser_logic
            file_path: Path to the source file
        
        Returns:
            IREvent protobuf message
        """
        # Create metadata
        timestamp = ir_events_pb2.Timestamp(
            seconds=int(fact.timestamp.timestamp()),
            nanos=int((fact.timestamp.timestamp() % 1) * 1e9)
        )
        
        metadata = ir_events_pb2.SourceMetadata(
            file_path=file_path,
            language=self.language,
            timestamp=timestamp
        )
        
        # Create the specific event based on fact type
        event = ir_events_pb2.IREvent(metadata=metadata)
        
        if fact.fact_type == "FunctionDeclared":
            # Convert parameters from dict to protobuf Parameter messages
            params = []
            for p in fact.data.get("parameters", []):
                param = ir_events_pb2.Parameter(name=p.get("name", ""), type=p.get("type", ""))
                params.append(param)
            
            event.function_declared.CopyFrom(
                ir_events_pb2.FunctionDeclared(
                    name=fact.data["name"],
                    param_count=fact.data["param_count"],
                    line_number=fact.line_number,
                    parent_scope=fact.data["parent_scope"],
                    return_type=fact.data.get("return_type", ""),
                    decorators=fact.data.get("decorators", []),
                    parameters=params,
                    docstring=fact.data.get("docstring", "")
                )
            )
        
        elif fact.fact_type == "FunctionCall":
            event.function_call.CopyFrom(
                ir_events_pb2.FunctionCall(
                    caller_function=fact.data["caller_function"],
                    callee_name=fact.data["callee_name"],
                    arg_count=fact.data["arg_count"],
                    line_number=fact.line_number
                )
            )
        
        elif fact.fact_type == "ReturnStatement":
            event.return_statement.CopyFrom(
                ir_events_pb2.ReturnStatement(
                    function_name=fact.data["function_name"],
                    has_value=fact.data["has_value"],
                    line_number=fact.line_number
                )
            )
        
        elif fact.fact_type == "ImportStatement":
            event.import_statement.CopyFrom(
                ir_events_pb2.ImportStatement(
                    module_name=fact.data["module_name"],
                    imported_names=fact.data["imported_names"],
                    is_wildcard=fact.data["is_wildcard"],
                    line_number=fact.line_number
                )
            )
        
        elif fact.fact_type == "ControlStructure":
            # Map string type to enum
            type_map = {
                "IF": ir_events_pb2.ControlStructure.IF,
                "WHILE": ir_events_pb2.ControlStructure.WHILE,
                "FOR": ir_events_pb2.ControlStructure.FOR,
                "SWITCH": ir_events_pb2.ControlStructure.SWITCH,
                "TRY": ir_events_pb2.ControlStructure.TRY,
            }
            control_type = type_map.get(fact.data["type"], ir_events_pb2.ControlStructure.IF)
            
            event.control_structure.CopyFrom(
                ir_events_pb2.ControlStructure(
                    type=control_type,
                    parent_function=fact.data["parent_function"],
                    line_number=fact.line_number,
                    has_else=fact.data["has_else"]
                )
            )
        
        elif fact.fact_type == "ClassDeclared":
            event.class_declared.CopyFrom(
                ir_events_pb2.ClassDeclared(
                    name=fact.data["name"],
                    base_classes=fact.data["base_classes"],
                    line_number=fact.line_number,
                    decorators=fact.data.get("decorators", []),
                    docstring=fact.data.get("docstring", "")
                )
            )
        
        elif fact.fact_type == "VariableAssignment":
            event.variable_assignment.CopyFrom(
                ir_events_pb2.VariableAssignment(
                    variable_name=fact.data["variable_name"],
                    scope=fact.data["scope"],
                    line_number=fact.line_number
                )
            )
        
        # ===== Extended Events =====
        
        elif fact.fact_type == "ThrowStatement":
            event.throw_statement.CopyFrom(
                ir_events_pb2.ThrowStatement(
                    exception_type=fact.data["exception_type"],
                    parent_function=fact.data["parent_function"],
                    line_number=fact.line_number,
                    has_message=fact.data["has_message"]
                )
            )
        
        elif fact.fact_type == "CatchClause":
            event.catch_clause.CopyFrom(
                ir_events_pb2.CatchClause(
                    exception_types=fact.data["exception_types"],
                    parent_function=fact.data["parent_function"],
                    line_number=fact.line_number,
                    is_catch_all=fact.data["is_catch_all"]
                )
            )
        
        elif fact.fact_type == "InterfaceDeclared":
            event.interface_declared.CopyFrom(
                ir_events_pb2.InterfaceDeclared(
                    name=fact.data["name"],
                    base_interfaces=fact.data.get("base_interfaces", []),
                    line_number=fact.line_number,
                    method_count=fact.data.get("method_count", 0)
                )
            )
        
        elif fact.fact_type == "EnumDeclared":
            event.enum_declared.CopyFrom(
                ir_events_pb2.EnumDeclared(
                    name=fact.data["name"],
                    member_count=fact.data.get("member_count", 0),
                    line_number=fact.line_number
                )
            )
        
        elif fact.fact_type == "MemberAccess":
            event.member_access.CopyFrom(
                ir_events_pb2.MemberAccess(
                    object_name=fact.data["object_name"],
                    member_name=fact.data["member_name"],
                    parent_function=fact.data["parent_function"],
                    line_number=fact.line_number,
                    is_method_call=fact.data["is_method_call"]
                )
            )
        
        elif fact.fact_type == "LambdaDeclared":
            event.lambda_declared.CopyFrom(
                ir_events_pb2.LambdaDeclared(
                    param_count=fact.data["param_count"],
                    parent_function=fact.data["parent_function"],
                    line_number=fact.line_number
                )
            )
        
        elif fact.fact_type == "AsyncFunctionDeclared":
            # Convert parameters from dict to protobuf Parameter messages
            params = []
            for p in fact.data.get("parameters", []):
                param = ir_events_pb2.Parameter(name=p.get("name", ""), type=p.get("type", ""))
                params.append(param)
            
            event.async_function_declared.CopyFrom(
                ir_events_pb2.AsyncFunctionDeclared(
                    name=fact.data["name"],
                    param_count=fact.data["param_count"],
                    line_number=fact.line_number,
                    parent_scope=fact.data["parent_scope"],
                    return_type=fact.data.get("return_type", ""),
                    decorators=fact.data.get("decorators", []),
                    parameters=params,
                    docstring=fact.data.get("docstring", "")
                )
            )
        
        elif fact.fact_type == "AwaitExpression":
            event.await_expression.CopyFrom(
                ir_events_pb2.AwaitExpression(
                    awaited_function=fact.data["awaited_function"],
                    parent_function=fact.data["parent_function"],
                    line_number=fact.line_number
                )
            )
        
        return event
    
    def emit_event(self, event: ir_events_pb2.IREvent):
        """Add event to the queue."""
        self.events_queue.append(event)
    
    def get_events(self) -> List[ir_events_pb2.IREvent]:
        """Get all queued events and clear the queue."""
        events = self.events_queue.copy()
        self.events_queue.clear()
        return events


class IREventStreamService(ir_events_pb2_grpc.IREventStreamServicer):
    """
    gRPC service implementation for streaming IR events.
    """
    
    def __init__(self):
        self.publisher = IREventPublisher()
        self.active = True
    
    def StreamEvents(self, request: ir_events_pb2.MonitorFileRequest, context) -> Iterator[ir_events_pb2.IREvent]:
        """
        Stream events for a monitored file.
        
        This would normally be triggered by file changes from LSP.
        For now, it's a simple iterator that yields events from the queue.
        """
        print(f"[Transport] Starting stream for {request.file_path}")
        
        # In a real implementation, this would:
        # 1. Start monitoring the file via LSP
        # 2. Yield events as they occur
        # 3. Keep the stream open
        
        # For now, we'll return any queued events
        while self.active and context.is_active():
            events = self.publisher.get_events()
            for event in events:
                yield event
            time.sleep(0.1)  # Prevent busy waiting
    
    def SendEventBatch(self, request_iterator: Iterator[ir_events_pb2.IREvent], context) -> ir_events_pb2.MonitorFileResponse:
        """
        Receive a batch of events (useful for testing).
        """
        count = 0
        for event in request_iterator:
            self.publisher.emit_event(event)
            count += 1
        
        return ir_events_pb2.MonitorFileResponse(
            success=True,
            message=f"Received {count} events"
        )
    
    def shutdown(self):
        """Gracefully shutdown the service."""
        self.active = False


def start_grpc_server(port: int = 50051):
    """
    Start the gRPC server.
    
    Args:
        port: Port to listen on (default: 50051)
    
    Returns:
        grpc.Server instance
    """
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    service = IREventStreamService()
    ir_events_pb2_grpc.add_IREventStreamServicer_to_server(service, server)
    
    server.add_insecure_port(f'[::]:{port}')
    server.start()
    
    print(f"[Transport] gRPC server started on port {port}")
    return server, service


if __name__ == "__main__":
    # Simple test server
    print("[Transport] Starting test gRPC server...")
    server, service = start_grpc_server()
    
    try:
        while True:
            time.sleep(86400)  # Keep alive
    except KeyboardInterrupt:
        print("\n[Transport] Shutting down...")
        service.shutdown()
        server.stop(0)