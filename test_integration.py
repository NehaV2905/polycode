#!/usr/bin/env python3
"""
Integration test script that starts Module 1 gRPC server and processes a file.
This allows Module 2 to connect and receive events.
"""

import sys
import os
import time
import asyncio
from pathlib import Path

# Add module1_adapter/src to path
sys.path.insert(0, str(Path(__file__).parent / "module1_adapter" / "src"))

from parser_logic import extract_facts_from_file
from transport import start_grpc_server, IREventPublisher
import ir_events_pb2


def main():
    """
    Start gRPC server and process a file, keeping the server alive.
    """
    if len(sys.argv) < 2:
        print("Usage: python test_integration.py <file_path>")
        sys.exit(1)

    file_path = sys.argv[1]
    grpc_port = 50051

    print(f"[Integration Test] Processing file: {file_path}")
    print(f"[Integration Test] Starting gRPC server on port {grpc_port}")

    # Start gRPC server
    server, service = start_grpc_server(grpc_port)

    # Extract facts from the file
    print(f"[Integration Test] Extracting facts from {file_path}...")
    facts = extract_facts_from_file(file_path)
    print(f"[Integration Test] Extracted {len(facts)} facts")

    # Convert facts to events and add to service publisher's queue
    publisher = IREventPublisher(language="python")
    for fact in facts:
        event = publisher.publish_fact(fact, file_path)
        service.publisher.emit_event(event)

    print(f"[Integration Test] Queued {len(facts)} events for streaming")
    print(f"[Integration Test] Server ready. Module 2 can now connect to http://127.0.0.1:{grpc_port}")
    print("[Integration Test] Press Ctrl+C to stop")

    # Keep server alive
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("\n[Integration Test] Shutting down...")
        service.shutdown()
        server.stop(0)


if __name__ == "__main__":
    main()
