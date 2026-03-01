# Module 3 — gRPC API Design

## Overview

Module 3 currently exists as a pure Rust library (`AnalysisEngine`) with a CLI binary. This document describes the addition of a **gRPC server binary** (`module3_server`) that exposes Module 3's analysis capabilities over the network so that Module 4 (Python) can consume them.

Module 3 connects to Module 2's gRPC server on startup, builds the full `IRGraph` in memory, and then listens for analysis requests from Module 4.

---

## Startup Sequence

```
1. Module 1 starts  →  gRPC server on port 50051, file pre-parsed and queued
2. Module 3 server starts  →  connects to Module 2 on port 50051, builds IRGraph
3. Module 3 server listens  →  gRPC server on port 50052, ready for Module 4
4. Module 4 starts  →  connects to Module 3 on port 50052
```

Module 3 will **fail fast on startup** if it cannot reach Module 2. There is no retry loop — fix the dependency order.

---

## What Changes in Module 3

The existing library code (`lib.rs`, all analysis modules, `GraphQuery`, `AnalysisEngine`) is **untouched**. We add:

```
module3_analysis/src/
├── lib.rs                        # unchanged
├── main.rs                       # existing CLI binary — unchanged
├── server.rs                     # NEW — entry point for gRPC server binary
├── grpc/
│   ├── mod.rs                    # NEW
│   └── handlers.rs               # NEW — implements the proto service trait
├── analysis/                     # unchanged
├── cache/                        # unchanged
├── models/                       # unchanged
└── queries/                      # unchanged
```

New binary in `Cargo.toml`:
```toml
[[bin]]
name = "module3_server"
path = "src/server.rs"
```

New dependencies in `Cargo.toml`:
```toml
tonic       = "0.11"
prost       = "0.12"
tokio       = { version = "1", features = ["full"] }
```

---

## Proto Definition — `analysis.proto`

Located at `polycode/proto/analysis.proto`. Shared between Module 3 (server) and Module 4 (client, via generated Python stubs).

```protobuf
syntax = "proto3";

package analysis;

// ── Shared message types ───────────────────────────────────────────────────

message FileRequest {
  string file_path = 1;
}

message ImpactRequest {
  string file_path    = 1;
  string target_symbol = 2;
}

message EmptyRequest {}

// ── Response types ─────────────────────────────────────────────────────────

message DeadCodeResponse {
  string file_path               = 1;
  repeated string unused_functions = 2;
}

message CallGraphResponse {
  string              file_path  = 1;
  repeated CallEdge   edges      = 2;
  repeated string     nodes      = 3;
}

message CallEdge {
  string caller = 1;
  string callee = 2;
}

message DependencyResponse {
  string                      file_path = 1;
  map<string, DependencyInfo> imports   = 2;
}

message DependencyInfo {
  string         module_name     = 1;
  repeated string imported_names = 2;
  bool            is_wildcard    = 3;
}

message ImpactResponse {
  string          target_symbol       = 1;
  string          target_file         = 2;
  repeated string direct_impacts      = 3;
  repeated string transitive_impacts  = 4;
  map<string, int32> impact_depth_levels = 5;
}

message FullAnalysisResponse {
  repeated string       tracked_files = 1;
  CallGraphResponse     global_call_graph   = 2;
  DependencyResponse    global_dependencies = 3;
  DeadCodeResponse      global_dead_code    = 4;
  repeated ImpactResponse cross_file_impacts = 5;
}

// ── Service ────────────────────────────────────────────────────────────────

service AnalysisService {
  // File-scoped — use when user specifies a file
  rpc GetDeadCode      (FileRequest)   returns (DeadCodeResponse);
  rpc GetCallGraph     (FileRequest)   returns (CallGraphResponse);
  rpc GetDependencies  (FileRequest)   returns (DependencyResponse);
  rpc GetImpact        (ImpactRequest) returns (ImpactResponse);

  // Codebase-scoped — use when no file is specified
  rpc GetFullAnalysis  (EmptyRequest)  returns (FullAnalysisResponse);

  // Utility
  rpc GetTrackedFiles  (EmptyRequest)  returns (TrackedFilesResponse);
  rpc HealthCheck      (EmptyRequest)  returns (HealthResponse);
}

message TrackedFilesResponse {
  repeated string file_paths = 1;
}

message HealthResponse {
  bool   ok             = 1;
  int32  node_count     = 2;
  int32  edge_count     = 3;
  int32  file_count     = 4;
}
```

---

## RPC Descriptions

### File-Scoped RPCs

| RPC | Input | Output | Internally calls |
|---|---|---|---|
| `GetDeadCode` | `file_path` | List of unused function names | `engine.dead_code(&query, file_path)` |
| `GetCallGraph` | `file_path` | Nodes + edges for that file | `engine.call_graph(&query, file_path)` |
| `GetDependencies` | `file_path` | Import map for that file | `engine.dependencies(&query, file_path)` |
| `GetImpact` | `file_path` + `target_symbol` | BFS ripple report | `engine.impact(&query, file_path, symbol)` |

### Codebase-Scoped RPCs

| RPC | Input | Output | Notes |
|---|---|---|---|
| `GetFullAnalysis` | none | All four analyses across all files | Used when `file_path` is null in chat request |
| `GetTrackedFiles` | none | List of all tracked file paths | Lightweight utility |
| `HealthCheck` | none | ok + graph stats | Used by Module 4 `/health` endpoint |

---

## Handler Logic (Rust pseudocode)

```rust
// grpc/handlers.rs

pub struct AnalysisServiceHandler {
    engine: Arc<Mutex<AnalysisEngine>>,
    graph:  Arc<IRGraph>,
}

impl AnalysisService for AnalysisServiceHandler {
    async fn get_dead_code(&self, request: Request<FileRequest>) -> Result<Response<DeadCodeResponse>> {
        let file_path = request.into_inner().file_path;
        let query     = GraphQuery::new(&self.graph);
        let mut engine = self.engine.lock().await;
        let result    = engine.dead_code(&query, &file_path);
        // serialize result → DeadCodeResponse
    }

    async fn get_full_analysis(&self, _: Request<EmptyRequest>) -> Result<Response<FullAnalysisResponse>> {
        let query = GraphQuery::new(&self.graph);
        let mut engine = self.engine.lock().await;
        // run all four analyses across all tracked files
        // aggregate into FullAnalysisResponse
    }
    // ... etc
}
```

`AnalysisEngine` is wrapped in `Arc<Mutex<>>` so the gRPC server can handle concurrent requests safely while the cache stays coherent.

---

## Running Module 3 Server

```bash
# From workspace root
cargo run -p module3_analysis --bin module3_server -- \
  --module2-addr http://127.0.0.1:50051 \
  --listen-port 50052 \
  --file module1_adapter/examples/sample.py \
  --language python
```

CLI flags:

| Flag | Default | Description |
|---|---|---|
| `--module2-addr` | `http://127.0.0.1:50051` | Module 2 gRPC address |
| `--listen-port` | `50052` | Port to expose Module 3 gRPC on |
| `--file` | required | Source file to parse on startup |
| `--language` | `python` | Language hint for Module 2 |

---

## Full Startup Command Sequence (Local Dev)

```bash
# Terminal 1 — Module 1 + Module 2
python start_grpc.py

# Terminal 2 — Module 3 gRPC server
cargo run -p module3_analysis --bin module3_server

# Terminal 3 — Module 4 FastAPI
cd module4_llm_interface && uvicorn main:app --reload --port 8080
```