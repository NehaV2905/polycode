import sys
import os
import time

sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'module1_adapter', 'src'))

from module1_adapter.core.transport import start_grpc_server, IREventPublisher
from module1_adapter.parsers import get_parser

def parse_and_queue(service, file_path, language=None):
    print(f"[Server] Parsing {file_path}...")
    try:
        content = open(file_path, encoding='utf-8').read()
    except Exception as e:
        print(f"[Server] Error reading file: {e}")
        return 0

    parser = get_parser(language=language, file_path=file_path)
    facts = parser.parse(content, file_path)
    print(f"[Server] Extracted {len(facts)} facts")

    publisher = IREventPublisher(language=language or "python")
    for fact in facts:
        event = publisher.publish_fact(fact, file_path)
        service.publisher.emit_event(event)

    print(f"[Server] {len(facts)} events queued")
    return len(facts)

print("[gRPC] Starting server on port 50051...")
server, service = start_grpc_server(50051)

file_to_parse = os.path.join(
    os.path.dirname(os.path.abspath(__file__)),
    'module1_adapter', 'examples', 'sample.py'
)

count = parse_and_queue(service, file_to_parse, language="python")
print(f"[gRPC] Ready — {count} events waiting.")
print("[gRPC] Streaming indefinitely. Press Ctrl+C to stop.")

try:
    while True:
        time.sleep(1)
except KeyboardInterrupt:
    print("\n[gRPC] Interrupted. Shutting down...")
    service.shutdown()
    server.stop(0)