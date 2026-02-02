# Module 2: IR Definition & Builder

This module is the semantic core of the polycode system. It receives language-specific events from Module 1 and builds a language-agnostic intermediate representation (IR) graph.

## Architecture

```
Module 1 (Language Adapters)
         ↓ (gRPC stream of IREvents)
    IR Event Client
         ↓
    Graph Builder (processes events)
         ↓
    IR Graph Storage (petgraph)
         ↓
    Query API → Module 3 (Analysis Engine)
```

## Key Components

### 1. IR Data Structures (`src/ir/`)

Defines language-agnostic representations:

- **NodeType**: Module, Function, Class, Variable, Interface, Enum, Lambda, ControlFlow
- **EdgeType**: Calls, Imports, HasMember, InheritsFrom, Returns, AccessesMember, Throws, Catches, Awaits, ContainedIn
- **IRNode**: A node with metadata (line number, timestamp, file path)
- **IREdge**: A directed relationship between two nodes

### 2. Graph Storage (`src/graph/storage.rs`)

- Uses `petgraph` for efficient graph operations
- Maintains indexes for O(1) node lookups by ID
- Tracks file ownership for incremental updates
- Symbol table for name resolution

### 3. Graph Builder (`src/graph/builder.rs`)

Processes IR events and builds the graph:

- Handles 15 event types from Module 1
- Maintains scope context
- Resolves symbol references
- Supports incremental updates (clear file and rebuild)

### 4. gRPC Client (`src/grpc_client/`)

- Connects to Module 1's gRPC server
- Receives streaming IREvents
- Delegates processing to GraphBuilder

### 5. Query API (`src/api/queries.rs`)

Provides queries for Module 3:

- `find_callers()` - Who calls this function?
- `find_callees()` - What does this function call?
- `find_dependencies()` - What modules does this file import?
- `find_dependents()` - What files import this module?
- `find_unused_functions()` - Which functions are never called?
- `find_subclasses()` - What classes inherit from this class?

## Installation

### Prerequisites

1. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install protoc (Protocol Buffers compiler):
   ```bash
   # macOS
   brew install protobuf

   # Ubuntu/Debian
   apt-get install protobuf-compiler

   # Or download from: https://github.com/protocolbuffers/protobuf/releases
   ```

### Build

```bash
cd module2_ir_builder
cargo build --release
```

## Usage

### Connect to Module 1 and Build Graph

```bash
cargo run -- connect \
    --server http://127.0.0.1:50051 \
    --file /path/to/your/file.py \
    --language python
```

### Example Output

```
2026-02-02T19:00:00.000Z INFO  Connecting to Module 1 at http://127.0.0.1:50051
2026-02-02T19:00:00.100Z INFO  Connected to Module 1 successfully
2026-02-02T19:00:00.120Z INFO  Monitoring file: sample.py (python)
2026-02-02T19:00:01.500Z INFO  Processed 42 events for file: sample.py
2026-02-02T19:00:01.502Z INFO  Graph stats: 28 nodes, 35 edges, 1 files

=== Graph Build Complete ===
Total nodes: 28
Total edges: 35
Total files: 1

=== Example Queries ===
Functions found: 12
  - login (line 45)
  - hash_password (line 12)
  - check_credentials (line 28)
  ...

Unused functions: 2
  - old_helper (line 89)
  - debug_function (line 102)

Dependencies: 5
  - os (imports: [])
  - sys (imports: [])
  - typing (imports: ["List", "Optional"])
```

## Design Decisions

### Why Rust?

- **Type safety**: Strong typing prevents graph inconsistencies
- **Memory safety**: No segfaults or memory leaks
- **Performance**: Fast graph operations for large codebases
- **Concurrency**: Easy to add parallel processing later

### Why petgraph?

- Industry-standard Rust graph library
- Efficient directed graph implementation
- Rich set of graph algorithms (for Module 3)
- Good documentation and active maintenance

### Why In-Memory Storage?

- Fast access for real-time analysis
- Graph fits in memory for typical projects
- Can add persistence layer later if needed

### Incremental Updates

When a file changes:
1. Clear all nodes/edges for that file
2. Receive new event stream from Module 1
3. Rebuild only that file's portion of the graph
4. Preserve cross-file relationships

This keeps the graph consistent without full rebuilds.

## Testing

### Unit Tests

```bash
cargo test
```

### Integration Test with Module 1

1. Start Module 1's gRPC server:
   ```bash
   cd ../module1_adapter
   python src/main.py --mode lsp --grpc-port 50051
   ```

2. In another terminal, run Module 2:
   ```bash
   cd ../module2_ir_builder
   cargo run -- connect --file ../module1_adapter/examples/sample.py
   ```

## Event Processing

Module 2 currently handles these events:

| Event Type | Status | Creates Node? | Creates Edge? |
|-----------|---------|---------------|---------------|
| FunctionDeclared | ✅ | Yes (Function) | HasMember (if nested) |
| AsyncFunctionDeclared | ✅ | Yes (Function) | HasMember (if nested) |
| ClassDeclared | ✅ | Yes (Class) | InheritsFrom (to bases) |
| FunctionCall | ✅ | No | Calls |
| ImportStatement | ✅ | Yes (Module) | Imports |
| VariableAssignment | ✅ | Yes (Variable) | HasMember |
| ControlStructure | ✅ | Yes (ControlFlow) | ContainedIn |
| InterfaceDeclared | ✅ | Yes (Interface) | - |
| EnumDeclared | ✅ | Yes (Enum) | - |
| ReturnStatement | 🚧 | No | Returns (TODO) |
| ThrowStatement | 🚧 | No | Throws (TODO) |
| CatchClause | 🚧 | No | Catches (TODO) |
| AwaitExpression | 🚧 | No | Awaits (TODO) |
| LambdaDeclared | 🚧 | Yes (Lambda) | (TODO) |
| MemberAccess | 🚧 | No | AccessesMember (TODO) |

## What's Left To Do

### High Priority (Core Functionality)

#### 1. Implement Remaining Event Types (6/15 missing)

**Location**: `src/grpc_client/event_processor.rs` and `src/graph/builder.rs`

Add handlers for these events:

- **ReturnStatement** (`process_return_statement`)
  - Create Returns edge from function to returned value context
  - Track `has_value` flag
  - File: Add method in `builder.rs:~line 280`
  - Event processor: Update `event_processor.rs:~line 140`

- **ThrowStatement** (`process_throw_statement`)
  - Create Throws edge from function to exception type
  - Track exception type and message presence
  - File: Add method in `builder.rs:~line 300`
  - Event processor: Update `event_processor.rs:~line 145`

- **CatchClause** (`process_catch_clause`)
  - Create Catches edge from function to exception types
  - Track catch-all vs specific exceptions
  - File: Add method in `builder.rs:~line 320`
  - Event processor: Update `event_processor.rs:~line 150`

- **AwaitExpression** (`process_await_expression`)
  - Create Awaits edge from caller to awaited function
  - Only for async functions
  - File: Add method in `builder.rs:~line 340`
  - Event processor: Update `event_processor.rs:~line 155`

- **LambdaDeclared** (`process_lambda_declared`)
  - Create Lambda node with param count and parent function
  - Track anonymous functions
  - File: Add method in `builder.rs:~line 360`
  - Event processor: Update `event_processor.rs:~line 160`

- **MemberAccess** (`process_member_access`)
  - Create AccessesMember edge for obj.field or obj.method()
  - Distinguish between property access and method calls
  - File: Add method in `builder.rs:~line 380`
  - Event processor: Update `event_processor.rs:~line 165`

**Testing**: Add tests in `tests/graph_builder_test.rs` for each new event type.

#### 2. Integration Testing with Module 1

**Steps**:
1. Start Module 1 gRPC server: `python module1_adapter/src/main.py --mode lsp --grpc-port 50051`
2. Run Module 2 client: `cargo run -- connect --file ../module1_adapter/examples/sample.py`
3. Verify graph is built correctly
4. Test with multiple files
5. Test incremental updates (modify file, re-parse)

**Create**: `tests/integration_test.rs` for end-to-end testing

#### 3. CLI Query Commands

**Location**: `src/main.rs:~line 109`

Implement the Query and Export commands that are currently stubs:
- Load graph from persistent storage
- Execute queries (callers, callees, dependencies, unused)
- Display results in terminal

### Medium Priority (Quality of Life)

#### 4. Persistent Storage

**Location**: Create new file `src/persistence/mod.rs`

Options:
- Serialize graph to JSON using `serde_json`
- Save to SQLite for queryable storage
- Use RocksDB for production performance

Add CLI commands:
- `--save-graph <path>` - Save graph to disk
- `--load-graph <path>` - Load graph from disk

#### 5. Better Error Messages

**Location**: Throughout codebase

Replace generic errors with specific error types:
- Create `src/error.rs` with custom error enum
- Use `thiserror` crate for better error messages
- Add context to all `anyhow::Result` returns

#### 6. Performance Profiling

**Steps**:
1. Add criterion benchmarks in `benches/`
2. Profile with large codebases (1000+ files)
3. Optimize hot paths (likely in graph traversal)
4. Consider parallel event processing with rayon

### Low Priority (Nice to Have)

#### 7. Graph Export Formats

**Location**: `src/api/queries.rs:~line 207`

Complete the `export_to_json` method and add:
- GraphML export (for visualization tools like Gephi)
- DOT format (for Graphviz)
- CSV export (nodes and edges as tables)

#### 8. Graph Visualization

**Create**: `src/visualization/` module

Generate visual representations:
- ASCII art call graphs (terminal)
- SVG/PNG via Graphviz integration
- Interactive HTML with D3.js

#### 9. Parallel Processing

**Location**: `src/graph/builder.rs`

Process independent files in parallel:
- Use `tokio::spawn` for concurrent file processing
- Merge graphs from multiple files efficiently
- Add `--parallel` CLI flag

### Testing Checklist

Before considering Module 2 "complete":
- [ ] All 15 event types implemented
- [ ] All event types have unit tests
- [ ] Integration tests with Module 1 pass
- [ ] Performance benchmarks added
- [ ] Documentation updated
- [ ] Error handling improved
- [ ] CLI commands fully functional

### Quick Start for Contributors

To implement a new event type:

1. Add handler method in `src/graph/builder.rs`
2. Update `src/grpc_client/event_processor.rs` to call it
3. Add test in `tests/graph_builder_test.rs`
4. Run `cargo test` to verify
5. Update event table in this README

Example commit message:
```
Added support for ReturnStatement events

Implemented process_return_statement in GraphBuilder to track
return statements and create Returns edges. Added unit tests
to verify functionality with both value and void returns.
```

## Future Enhancements

1. **Persistent Storage**: Save graph to disk for offline queries
2. **Graph Diffing**: Compare graphs across commits
3. **More Edge Types**: Track all 15 event types
4. **Performance Optimization**: Parallel event processing
5. **Graph Compression**: Reduce memory footprint for large projects

## API for Module 3

Module 3 can use the graph via the `GraphQuery` interface:

```rust
use module2_ir_builder::{GraphQuery, IRGraph};

// After building the graph
let query = GraphQuery::new(&graph);

// Find all callers of a function
let callers = query.find_callers("login", "/path/to/file.py");

// Find unused functions
let unused = query.find_unused_functions("/path/to/file.py");

// Get dependencies
let deps = query.find_dependencies("/path/to/file.py");
```

## Contributing

When adding support for new event types:

1. Update `event_processor.rs` to handle the event
2. Add corresponding logic to `builder.rs`
3. Update edge/node types if needed
4. Add query methods to `queries.rs`
5. Write tests

## License

Part of the polycode project.
