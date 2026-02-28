import sys
import os
import time
import threading

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

def auto_shutdown(service, delay=4):
    """Shut down the stream after giving Module 3 time to consume all events."""
    time.sleep(delay)
    print(f"[Server] Closing stream after {delay}s...")
    service.shutdown()

print("[gRPC] Starting server on port 50051...")
server, service = start_grpc_server(50051)

file_to_parse = os.path.join(
    os.path.dirname(os.path.abspath(__file__)),
    'module1_adapter', 'examples', 'sample.py'
)

count = parse_and_queue(service, file_to_parse, language="python")
print(f"[gRPC] Ready — {count} events waiting.")
print("[gRPC] Will auto-close stream after Module 3 connects (4s timeout).")
print("[gRPC] Run Module 3 now in Terminal 2.")

# Watch for first connection, then start countdown
def watch_and_shutdown():
    # Wait until stream starts (service will print "[Transport] Starting stream...")
    time.sleep(2)   # give Module 3 time to connect and start consuming
    auto_shutdown(service, delay=4)

watcher = threading.Thread(target=watch_and_shutdown, daemon=True)
watcher.start()

try:
    while service.active:
        time.sleep(0.5)
    # Give Module 3 a moment to finish reading remaining events
    time.sleep(2)
    print("[gRPC] Stream closed. Shutting down server.")
    server.stop(0)
except KeyboardInterrupt:
    print("\n[gRPC] Interrupted. Shutting down...")
    service.shutdown()
    server.stop(0)