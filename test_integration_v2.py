#!/usr/bin/env python3
"""
Integration test script v2 - Stream events on-demand when client connects.
"""

import sys
import os
import time
from pathlib import Path
from typing import Iterator

# Add module1_adapter/src to path
sys.path.insert(0, str(Path(__file__).parent / "module1_adapter" / "src"))

from parser_logic import extract_facts_from_file
import ir_events_pb2
import ir_events_pb2_grpc
from transport import IREventPublisher
import grpc
from concurrent import futures


class OnDemandIREventStreamService(ir_events_pb2_grpc.IREventStreamServicer):
    """
    gRPC service that parses and streams events on-demand.
    """

    def __init__(self, language: str = "python"):
        self.language = language
        self.active = True

    def StreamEvents(self, request: ir_events_pb2.MonitorFileRequest, context) -> Iterator[ir_events_pb2.IREvent]:
        """
        Stream events for a monitored file by parsing it on-demand.
        """
        file_path = request.file_path
        print(f"[Transport] Client connected, streaming events for {file_path}")

        try:
            # Extract facts from the file
            facts = extract_facts_from_file(file_path)
            print(f"[Transport] Extracted {len(facts)} facts, streaming...")

            # Convert facts to events and stream them
            publisher = IREventPublisher(language=request.language or self.language)
            for i, fact in enumerate(facts):
                if not context.is_active():
                    print(f"[Transport] Client disconnected after {i} events")
                    break

                event = publisher.publish_fact(fact, file_path)
                yield event

            print(f"[Transport] Finished streaming {len(facts)} events")
            print(f"[Transport] Closing stream")

            # Close the stream by returning
            # (client will know all events have been sent)

        except Exception as e:
            print(f"[Transport] Error streaming events: {e}")
            raise

    def shutdown(self):
        """Gracefully shutdown the service."""
        self.active = False


def main():
    """
    Start gRPC server that streams events on-demand.
    """
    grpc_port = 50051

    print(f"[Integration Test V2] Starting gRPC server on port {grpc_port}")

    # Start gRPC server with our custom service
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    service = OnDemandIREventStreamService(language="python")
    ir_events_pb2_grpc.add_IREventStreamServicer_to_server(service, server)

    server.add_insecure_port(f'[::]:{grpc_port}')
    server.start()

    print(f"[Integration Test V2] Server ready on http://127.0.0.1:{grpc_port}")
    print("[Integration Test V2] Module 2 can now connect")
    print("[Integration Test V2] Press Ctrl+C to stop")

    # Keep server alive
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("\n[Integration Test V2] Shutting down...")
        service.shutdown()
        server.stop(0)


if __name__ == "__main__":
    main()
