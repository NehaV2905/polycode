# Module 4 — LLM Interface (Python FastAPI)

## Overview

Module 4 is a **Python FastAPI server** that acts as the intelligent chat layer on top of Polycode's analysis pipeline. It receives natural language questions about code, fetches structured analysis data from Module 3 over gRPC, and uses the Anthropic API (Claude) to answer them.

It exposes a REST API on port `8080` with CORS open for local development, designed to be consumed by a React frontend (future) or directly via curl/Postman today.

---

## Architecture Position

```
React Frontend  (future, localhost:3000 or 5173)
      ↓  HTTP REST — port 8080
Module 4  —  Python FastAPI
      ↓  gRPC — port 50052
Module 3  —  Rust gRPC Server (AnalysisEngine)
      ↓  (built on startup from)
Module 2  —  Rust IR Builder — port 50051
      ↑  gRPC
Module 1  —  Python Language Adapter
```

---

## Key Design Decisions

### 1. Full context loaded once per conversation

On the **first message** of a conversation, Module 4 calls Module 3's `GetFullAnalysis` RPC (or the file-scoped equivalent if a `file_path` is given) and loads the entire analysis JSON into the **system prompt**. This context is fixed for the lifetime of the conversation — no per-turn gRPC calls.

This works because:
- The codebase is small-medium scale — full analysis JSON fits comfortably within Claude Sonnet's 200k token context window
- It allows Claude to answer any question (file-scoped or codebase-wide) without needing to re-fetch
- It eliminates per-turn latency from gRPC calls

### 2. Single Claude call per user message

Every `/chat` request makes exactly **one** Anthropic API call — to Sonnet, with the full analysis JSON as system context and the complete conversation history. Claude reads the question and the data and answers naturally, whether the user wants an overview, a specific function lookup, a flowchart, or anything else. No pre-classification step is needed.

### 3. `file_path` is optional

If `file_path` is `null`, Module 4 fetches the full codebase analysis from Module 3 (`GetFullAnalysis`). If `file_path` is provided, it fetches only that file's analysis. Either way, Claude receives the same JSON structure.

### 4. Conversations are locked to their initial scope

The `file_path` (or `null` for codebase-wide) is **locked at conversation creation** — the scope of a conversation cannot change mid-session. To ask about a different file, start a new conversation.

### 5. Raw JSON as context, no summarisation

Module 3's analysis output is serialised to JSON and sent directly to Claude as-is in the system prompt. No pre-processing or summarisation — Claude reasons over the raw structure.

---

## File Structure

```
module4_llm_interface/
├── main.py                        # FastAPI app instantiation + CORS + router registration
├── routes/
│   └── chat.py                    # POST /chat, GET /health, DELETE /conversation/{id}
├── chat/
│   ├── context.py                 # Fetch from Module 3 gRPC, assemble system prompt JSON
│   └── answerer.py                # Sonnet call → final answer
├── conversation/
│   └── store.py                   # In-memory dict: conversation_id → Conversation
├── grpc_client/
│   └── analysis_client.py         # Thin wrapper around Module 3 gRPC stubs
├── models/
│   └── request.py                 # Pydantic: ChatRequest, ChatResponse
├── proto/                         # Generated Python stubs from analysis.proto
│   ├── analysis_pb2.py
│   └── analysis_pb2_grpc.py
├── .env                           # ANTHROPIC_API_KEY, MODULE3_ADDR
└── requirements.txt
```

---

## Data Models

### `ChatRequest`
```python
class ChatRequest(BaseModel):
    file_path:       str | None   # null = codebase-wide
    message:         str
    conversation_id: str | None   # null on first message → server creates UUID
```

### `ChatResponse`
```python
class ChatResponse(BaseModel):
    reply:           str
    conversation_id: str
```

### `Conversation`
```python
@dataclass
class Conversation:
    id:          str
    file_path:   str | None       # locked at creation
    messages:    list[dict]       # [{"role": "user"|"assistant", "content": str}]
    context_json: dict            # Module 3 analysis — fetched once, reused every turn
    created_at:  datetime
    last_active: datetime
```

---

## Per-Request Flow

```
POST /chat { file_path, message, conversation_id? }
      │
      ├─ conversation_id is None?
      │     → create new UUID
      │     → call Module 3 gRPC to fetch analysis JSON (once)
      │     → store as Conversation with locked file_path + context_json
      │
      ├─ answerer.py
      │     → POST /v1/messages (Sonnet)
      │     → system: "You are a code analysis assistant.\n\n## Analysis Data\n" + JSON.dumps(context_json)
      │     → messages: full conversation history + new user message
      │     → response: string (buffered)
      │
      ├─ Append user message + assistant reply to conversation history
      │
      └─ Return ChatResponse { reply, conversation_id }
```

---

## Claude API Call — Answer Generation

**Model:** `claude-sonnet-4-6`
**Purpose:** Full reasoning over analysis data + conversation history. Claude handles any question naturally — overviews, specific lookups, impact analysis, flowcharts — without needing pre-classification.

**System prompt (built once per conversation):**
```
You are a code analysis assistant for the Polycode system. You have been given structured
analysis data about a codebase in JSON format below. Use this data to answer the user's
questions accurately. Do not make up information that is not present in the data.

If the user asks for a flowchart or diagram, produce a Mermaid diagram in a fenced code block.
If the user asks a follow-up question, use the conversation history for context.

## Analysis Data

{pretty-printed JSON from Module 3}
```

**Messages array:** full conversation history (alternating user/assistant) + new user message.

---

## REST API Endpoints

### `POST /chat`

Main chat endpoint.

**Request:**
```json
{
  "file_path": "module1_adapter/examples/sample.py",
  "message": "What would break if I change the connect function?",
  "conversation_id": null
}
```

Pass `null` for `conversation_id` on the first message — the server creates and returns a UUID. Pass the returned UUID on every follow-up.

Pass `null` for `file_path` for codebase-wide questions — Module 4 will call `GetFullAnalysis` on Module 3.

**Response:**
```json
{
  "reply": "Changing `connect` would directly affect `create_user` and `_insert_user`, which both call it. Transitively, `main` would also be affected since it calls `create_user`. Impact depth: connect → create_user (depth 1), _insert_user (depth 1), main (depth 2).",
  "conversation_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479"
}
```

---

### `GET /health`

Liveness + dependency check.

**Response:**
```json
{
  "status": "ok",
  "module3_connected": true,
  "module3_graph_stats": {
    "node_count": 33,
    "edge_count": 28,
    "file_count": 5
  }
}
```

Calls `HealthCheck` on Module 3's gRPC. If Module 3 is unreachable, returns `module3_connected: false` with HTTP 200 (not 500) so the caller can make an informed decision.

---

### `DELETE /conversation/{conversation_id}`

Clear conversation history and cached context for a session.

**Response:**
```json
{ "deleted": true, "conversation_id": "f47ac10b-..." }
```

Returns HTTP 404 if the conversation ID is not found.

---

## Conversation Lifecycle

```
First message (conversation_id: null)
  → Server creates UUID
  → Fetches analysis context from Module 3 (one gRPC call)
  → Stores Conversation { id, file_path, messages: [], context_json }
  → Processes message, appends to history
  → Returns reply + UUID

Follow-up messages (conversation_id: "f47ac10b-...")
  → Server looks up existing Conversation
  → Reuses stored context_json — no gRPC call
  → Appends new message to history, sends full history to Sonnet
  → Returns reply

DELETE /conversation/{id}
  → Wipes entry from in-memory store
  → Context and history are gone
```

**Important:** Conversations are in-memory only. They are lost when Module 4 restarts. This is intentional for V1.

---

## Environment Variables

```bash
# .env
ANTHROPIC_API_KEY=sk-ant-...
MODULE3_ADDR=http://127.0.0.1:50052
```

---

## Requirements

```
# requirements.txt
fastapi
uvicorn[standard]
anthropic
grpcio
grpcio-tools
python-dotenv
pydantic
```

---

## Running Module 4

```bash
cd module4_llm_interface

# Install dependencies
pip install -r requirements.txt

# Generate gRPC stubs from proto (run once, or when proto changes)
python -m grpc_tools.protoc \
  -I../proto \
  --python_out=./proto \
  --grpc_python_out=./proto \
  ../proto/analysis.proto

# Start server
uvicorn main:app --reload --port 8080
```

---

## Example curl Session

```bash
# First message — no conversation_id, no file_path (codebase-wide)
curl -X POST http://localhost:8080/chat \
  -H "Content-Type: application/json" \
  -d '{
    "file_path": null,
    "message": "Give me an overview of this codebase",
    "conversation_id": null
  }'

# Response includes conversation_id — save it
# { "reply": "...", "conversation_id": "f47ac10b-..." }

# Follow-up — file-scoped, using the same conversation
curl -X POST http://localhost:8080/chat \
  -H "Content-Type: application/json" \
  -d '{
    "file_path": "module1_adapter/examples/sample.py",
    "message": "What functions are unused in sample.py?",
    "conversation_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479"
  }'

# Impact analysis
curl -X POST http://localhost:8080/chat \
  -H "Content-Type: application/json" \
  -d '{
    "file_path": "module1_adapter/examples/sample.py",
    "message": "What would break if I change the connect function?",
    "conversation_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479"
  }'

# Clear session
curl -X DELETE http://localhost:8080/conversation/f47ac10b-58cc-4372-a567-0e02b2c3d479

# Health check
curl http://localhost:8080/health
```

---

## Full Local Dev Startup Order

```bash
# Terminal 1 — Module 1 gRPC (pre-parses file, feeds Module 2)
cd polycode
python start_grpc.py

# Terminal 2 — Module 3 gRPC server (connects to Module 2, listens for Module 4)
cd polycode
cargo run -p module3_analysis --bin module3_server

# Terminal 3 — Module 4 FastAPI
cd polycode/module4_llm_interface
uvicorn main:app --reload --port 8080
```

---

## What's Intentionally Out of Scope for V1

- Authentication / API keys on Module 4's endpoints
- Conversation TTL / eviction (cleared on restart only)
- Streaming responses (buffered only)
- Persistent conversation storage (in-memory only)
- Source code snippets in context (IR graph only)
- React frontend (consume the REST API directly for now)