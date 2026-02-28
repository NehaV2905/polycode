# Module 3: Semantic Analysis & Impact Engine

### Detailed Design Document

---

## 1. Overview

Module 3 is responsible for performing semantic analysis on the IRGraph produced by Module 2. It provides high-level reasoning capabilities such as call graph analysis, dependency mapping, impact evaluation, and dead code detection.

It consumes an already-built IRGraph and produces structured analysis outputs that can be used for documentation, refactoring assistance, and intelligent code insights.

Unlike Module 2 (which incrementally builds the graph from IR events), Module 3 operates purely on the stable semantic model and must remain decoupled from raw event streams.

Primary philosophy:
> Module 1 observes → Module 2 models → Module 3 reasons

> **Scope Note:** Analysis is limited to the **9 fully implemented event types** in Module 2. Persistence is **intentionally out of scope** — all analysis operates in-memory. The graph is lost when the process exits.

---

## 2. Integration with Module 2

Module 3 is implemented as a **Rust crate within the same Cargo workspace** as Module 2. It does not communicate over a network boundary — it directly links against Module 2's library and uses its `GraphQuery` API.

### Workspace Layout

```
polycode/
├── Cargo.toml               # Workspace root
├── module2_ir_builder/      # Module 2 (library + binary)
│   └── src/
│       ├── lib.rs
│       └── api/queries.rs   # GraphQuery API consumed by Module 3
└── module3_analysis/        # Module 3 (this module)
    └── src/
        └── ...
```

### Cargo.toml Dependency

```toml
# module3_analysis/Cargo.toml
[dependencies]
module2_ir_builder = { path = "../module2_ir_builder" }
```

### Module 2 API Surface Consumed by Module 3

Module 3 exclusively uses the `GraphQuery` struct exposed by Module 2. The full API surface is:

```rust
use module2_ir_builder::{GraphQuery, IRGraph};

let graph: IRGraph = /* received from Module 2 builder */;
let query = GraphQuery::new(&graph);

// Fetch all functions defined in a file
query.get_functions_in_file(file_path: &str) -> Vec<IRNode>

// Get all functions that call the given function (reverse lookup)
query.find_callers(function_name: &str, file_path: &str) -> Vec<IRNode>

// Get all functions called by the given function
query.find_callees(function_name: &str, file_path: &str) -> Vec<IRNode>

// Get all modules/files that this file imports
query.find_dependencies(file_path: &str) -> Vec<IRNode>

// Get all functions that are never called (dead code candidates)
query.find_unused_functions(file_path: &str) -> Vec<IRNode>

// Get all classes and their inheritance relationships
query.get_classes_with_base(class_name: &str) -> Vec<IRNode>

// Export full graph to JSON (for debug/inspection)
query.export_to_json() -> String

// Graph statistics (node count, edge count)
query.get_stats() -> GraphStats
```

> **Important:** Module 3 never accesses `IRGraph` internals directly. All access goes through `GraphQuery`. This keeps Module 3 decoupled from Module 2's internal storage decisions.

---

## 3. Supported Event Types (V1 Scope)

Module 3 reasons only over the **9 event types fully processed by Module 2**. The remaining 6 (ReturnStatement, ThrowStatement, CatchClause, AwaitExpression, LambdaDeclared, MemberAccess) are not yet reflected in the graph and are therefore out of scope.

| Event Type | Node/Edge Created by M2 | Used by Module 3 For |
|---|---|---|
| `FunctionDeclared` | Function node + HasMember edge | Call graph, dead code, impact |
| `AsyncFunctionDeclared` | Async Function node + HasMember edge | Call graph, dead code, impact |
| `ClassDeclared` | Class node + InheritsFrom edge | Class hierarchy queries |
| `FunctionCall` | Calls edge | Call graph, impact traversal |
| `ImportStatement` | Module node + Imports edge | Dependency graph |
| `VariableAssignment` | Variable node + HasMember edge | (foundation, future use) |
| `ControlStructure` | ControlFlow node + ContainedIn edge | (foundation, future use) |
| `InterfaceDeclared` | Interface node | (foundation, future use) |
| `EnumDeclared` | Enum node | (foundation, future use) |

---

## 4. Goals

### Functional Goals

- Provide semantic queries over IRGraph
- Support real-time incremental analysis
- Generate structured analysis results (impact, dependencies, dead code)
- Language-agnostic reasoning (IR is already normalized by Module 2)

### Non-Functional Goals

- Decoupled from event stream logic
- In-memory only — no disk persistence
- Incremental and fast for real-time usage
- Extensible for future analyses once Module 2 completes remaining event types

---

## 5. High-Level Responsibilities

Module 3 performs four core roles:

1. **Query Layer** — Provides reusable semantic queries over IRGraph via Module 2's `GraphQuery`
2. **Analysis Engine** — Runs algorithms: call graph construction, dependency tracing, dead code detection
3. **Impact Evaluation Engine** — Determines ripple effects of modifying a function/class/module
4. **In-Memory Cache** — Caches analysis results per graph version to avoid redundant recomputation

> **Out of Scope (V1):** Persistence layer, project-level graph aggregation, documentation generation.

---

## 6. Architecture

```
IRGraph (from Module 2)
        ↓
  GraphQuery API        ← Only entry point into Module 2
        ↓
  Query Layer           ← Semantic access helpers built on top of GraphQuery
        ↓
  Analysis Engine       ← Call graph, dependency graph, dead code
        ↓
  Impact Engine         ← Change propagation reasoning
        ↓
  In-Memory Cache       ← Invalidated on graph updates
```

No component in Module 3 directly accesses IRGraph internals. Everything goes through `GraphQuery`.

---

## 7. File Structure

```
module3_analysis/
├── Cargo.toml
└── src/
    ├── lib.rs

    ├── analysis/
    │   ├── call_graph.rs         # Build & traverse call graph
    │   ├── dependency_graph.rs   # Module/file dependency mapping
    │   ├── dead_code.rs          # Unused function/class detection
    │   └── impact_analysis.rs    # Ripple effect evaluation

    ├── queries/
    │   ├── function_queries.rs   # Caller/callee/unused queries
    │   └── class_queries.rs      # Inheritance hierarchy queries

    ├── cache/
    │   └── analysis_cache.rs     # In-memory result cache

    └── models/
        ├── analysis_result.rs    # Generic analysis output container
        └── impact_report.rs      # Impact analysis output
```

> **Removed from original design:** `persistence/`, `project/` directories — deferred to future versions.

---

## 8. Core Components

### 8.1 Query Layer

Thin wrappers built on top of `GraphQuery` that express domain-specific questions.

```rust
// Example internal helper — wraps GraphQuery
fn get_all_callers_recursive(
    query: &GraphQuery,
    function_name: &str,
    file_path: &str,
    depth: usize,
) -> Vec<IRNode>
```

Responsibilities:
- Fetch functions by file
- Retrieve callers and callees
- Resolve class hierarchies (via `InheritsFrom` edges)

---

### 8.2 Analysis Engine

Performs higher-order reasoning using query primitives from 8.1.

#### 8.2.1 Call Graph Analysis

Builds a directed graph of function invocation relationships.

- **Input:** IRGraph (via GraphQuery)
- **Output:** `CallGraph { nodes: Vec<FunctionNode>, edges: Vec<CallEdge> }`
- **Backed by:** `FunctionDeclared`, `AsyncFunctionDeclared`, `FunctionCall` events

#### 8.2.2 Dependency Graph Analysis

Maps import relationships across files/modules.

- **Input:** IRGraph (via GraphQuery)
- **Output:** `DependencyGraph { file → Vec<imported_module> }`
- **Backed by:** `ImportStatement` events
- **Answers:** "If module X changes, which files are affected?"

#### 8.2.3 Dead Code Detection

Identifies functions and async functions that are never called.

- **Input:** IRGraph (via GraphQuery)
- **Output:** `Vec<IRNode>` (unreachable functions)
- **Backed by:** `FunctionDeclared`, `AsyncFunctionDeclared`, `FunctionCall` events
- **Algorithm:** Set difference — all declared functions minus all functions appearing as callees

---

### 8.3 Impact Analysis Engine

The most critical reasoning component. Determines semantic ripple effects of modifying a function or class.

**Example:** If `process_order()` is modified, what else breaks?

**Algorithm:**
1. Locate function node in call graph
2. Traverse reverse `Calls` edges (who calls this?)
3. Recurse transitively to find all indirect callers
4. Produce `ImpactReport` with depth levels

**Output:**
```rust
ImpactReport {
    target_symbol: "process_order",
    direct_impacts: Vec<IRNode>,       // functions that directly call it
    transitive_impacts: Vec<IRNode>,   // functions that indirectly depend on it
    impact_depth_levels: HashMap<IRNode, usize>,
}
```

**Backed by:** `FunctionCall` (Calls edges), `FunctionDeclared`, `ClassDeclared`, `ImportStatement`

---

### 8.4 In-Memory Cache

Avoids rerunning expensive graph traversals on every query.

**Strategy:**
- Cache keyed by `(analysis_type, file_path, graph_version)`
- On graph update from Module 2: invalidate only the affected file's cache entries
- No TTL — cache lives for the duration of the process

```rust
struct AnalysisCache {
    store: HashMap<CacheKey, AnalysisResult>,
}

impl AnalysisCache {
    fn invalidate_file(&mut self, file_path: &str) { ... }
    fn get(&self, key: &CacheKey) -> Option<&AnalysisResult> { ... }
    fn insert(&mut self, key: CacheKey, result: AnalysisResult) { ... }
}
```

> Since the graph is in-memory only, the cache is also in-memory only. Both are lost when the process exits. This is intentional for V1.

---

## 9. Real-Time Incremental Analysis

Since Module 2 updates the graph incrementally per file, Module 3 mirrors this with localized recomputation.

Strategy:
- On graph update notification: invalidate affected file's cache entries
- Recompute only the analyses touching that file
- Leave unrelated file analyses untouched

---

## 10. Data Models

### AnalysisResult

Generic container for any analysis output.

```rust
struct AnalysisResult {
    analysis_type: AnalysisType,   // CallGraph | DeadCode | Dependency
    generated_at: SystemTime,
    summary_stats: SummaryStats,
    detailed_nodes: Vec<IRNode>,
}
```

### ImpactReport

Represents ripple effects of a code modification.

```rust
struct ImpactReport {
    target_symbol: String,
    direct_impacts: Vec<IRNode>,
    transitive_impacts: Vec<IRNode>,
    impact_depth_levels: HashMap<String, usize>,
}
```

---

## 11. Interaction Flow

```
1. Module 1  →  streams IR events (15 types, 9 implemented)
2. Module 2  →  builds IRGraph incrementally, exposes GraphQuery
3. Module 3  →  receives IRGraph reference via GraphQuery
4. Query Layer      extracts semantic relationships
5. Analysis Engine  computes call graph, dependencies, dead code
6. Impact Engine    evaluates ripple effects on demand
7. Cache            stores results in-memory for the session
```

---

## 12. Out of Scope for V1 (Future Enhancements)

These are explicitly deferred and should not be implemented in V1:

- **Persistence layer** — graph and results are in-memory only
- **Project-level graph** — only single-file scope today
- **Documentation generation** from semantic graph
- **Refactoring safety validator**
- **Async misuse detection** — requires `AwaitExpression` (Module 2 TODO)
- **Exception flow analysis** — requires `ThrowStatement`, `CatchClause` (Module 2 TODO)
- **Lambda analysis** — requires `LambdaDeclared` (Module 2 TODO)
- **Cross-language project reasoning**

---

## 13. How to Run

### Prerequisites

- Rust installed via rustup
- Python 3.10+ with the module1_adapter virtual environment
- pygls 1.3.1 (not 2.x — the API changed)

```bash
# Install correct pygls version
cd module1_adapter
.venv\Scripts\pip install "pygls==1.3.1"
```

### Step 1 — Build Module 3

From the workspace root:

```bash
cd C:\Users\vaish\polycode
cargo build -p module3_analysis
```

### Step 2 — Start Module 1 gRPC Server (Terminal 1)

```bash
cd C:\Users\vaish\polycode
module1_adapter\.venv\Scripts\Activate
python start_grpc.py
```

Wait until you see:
```
[gRPC] Ready — 83 events waiting.
[gRPC] Run Module 3 now in Terminal 2.
```

### Step 3 — Run Module 3 (Terminal 2)

**Basic analysis:**
```bash
cargo run -p module3_analysis -- --file module1_adapter/examples/sample.py
```

**With impact analysis:**
```bash
cargo run -p module3_analysis -- --file module1_adapter/examples/sample.py --impact-target login
```

**Against a different file:**
```bash
cargo run -p module3_analysis -- --file module1_adapter/examples/ecommerce.py --impact-target process_order
```

### CLI Arguments

| Argument | Default | Description |
|---|---|---|
| `--server` | `http://127.0.0.1:50051` | Module 1 gRPC server address |
| `--file` | (required) | Source file to analyse |
| `--language` | `python` | Programming language |
| `--impact-target` | (optional) | Function name for impact analysis |

### Run Unit Tests Only (no Module 1 needed)

```bash
cargo test -p module3_analysis
```

All 42 tests run against hand-built in-memory graphs — no gRPC server required.

### Known Issues

- `start_grpc.py` auto-closes the stream after ~6 seconds. If Module 3 takes longer to connect, restart Terminal 1 first.
- The `<module>` unresolved call warning is harmless — it comes from Module 2 seeing a top-level call with no declared caller.
- pygls 2.x breaks Module 1. Always use `pygls==1.3.1`.

---

## 14. Design Principles

- **Separation of concerns** — observation (M1), modeling (M2), reasoning (M3) are strictly isolated
- **API-only coupling** — Module 3 never touches IRGraph internals, only `GraphQuery`
- **Incremental computation** — invalidate and recompute only what changed
- **Language-agnostic** — IR is already normalized by Module 2; Module 3 has zero language-specific logic
- **Honest scoping** — V1 only reasons over what Module 2 has fully implemented