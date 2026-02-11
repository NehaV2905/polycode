# Module 1 - Language Adapter

## Overview

Module 1 is a language-agnostic adapter that listens to source code changes and extracts semantic facts as IR events. It operates on the principle of **observation without analysis**.

## Platform Compatibility

✅ **Fully Cross-Platform**

Module 1 works identically across all major operating systems:
- **Windows** (7, 8, 10, 11, Server)
- **macOS** (10.14+)
- **Linux** (Ubuntu, Debian, Fedora, CentOS, etc.)

**Key Features:**
- URI to path conversion handles platform-specific formats
- File operations use `pathlib` for OS-agnostic path handling
- Line endings handled automatically (CRLF on Windows, LF on Unix)
- No platform-specific dependencies

**Tested On:**
- Windows 10/11 (x64)
- macOS Monterey+ (Apple Silicon & Intel)
- Ubuntu 20.04+ LTS
- Python 3.8+

## What It Does

1. **Listens**: Monitors file changes via LSP (Language Server Protocol)
2. **Extracts**: Walks the AST to identify code constructs (functions, calls, control structures)
3. **Reports**: Emits IR events via gRPC

## What It Does NOT Do

- ❌ Analysis or inference
- ❌ Type checking
- ❌ Optimization suggestions
- ❌ Code transformation

It's deliberately "dumb" - just observing and reporting facts.

## Architecture

```
Source Code (Python/Java/Go)
         ↓
    LSP Server
         ↓
    AST Walker (parser_logic.py)
         ↓
    IR Events (Protocol Buffers)
         ↓
    gRPC Stream (transport.py)
         ↓
    Downstream Modules
```

## Components

### 1. Protocol Buffer Schema (`proto/ir_events.proto`)
Defines language-agnostic IR events:

**Core Events:**
- `FunctionDeclared` - Regular function declarations
- `AsyncFunctionDeclared` - Async function declarations
- `FunctionCall` - Function invocations
- `ReturnStatement` - Return statements
- `ImportStatement` - Import/include directives
- `ControlStructure` (if/while/for/try) - Control flow
- `ClassDeclared` - Class definitions
- `VariableAssignment` - Variable assignments

**Extended Events (Exception Handling & Async):**
- `ThrowStatement` - Exception throwing (raise, throw)
- `CatchClause` - Exception handlers (except, catch)
- `AwaitExpression` - Async await calls

**Extended Events (Advanced Constructs):**
- `LambdaDeclared` - Anonymous/lambda functions
- `MemberAccess` - Property/method access (obj.field)
- `InterfaceDeclared` - Interface/trait/protocol declarations
- `EnumDeclared` - Enumeration declarations

### 2. Parser Logic (`src/parser_logic.py`)
Python AST walker that extracts facts:
- Uses Python's built-in `ast` module
- Walks the tree asking simple questions: "Is this a function? A call? A loop?"
- Returns `IRFact` objects with metadata

### 3. Transport Layer (`src/transport.py`)
gRPC streaming implementation:
- Converts `IRFact` objects to protobuf messages
- Streams events to consumers
- Handles connection lifecycle

### 4. Main Orchestrator (`src/main.py`)
Ties everything together:
- Runs LSP server to monitor file changes
- Triggers parser on document events
- Publishes events via transport

## Usage

### Standalone File Mode (Testing)
Process a single Python file and print IR events:

```bash
cd f:\CapstoneProject
python module1_adapter\src\main.py --mode file --file module1_adapter\examples\sample.py
```

### LSP Server Mode (Production)
Run as an LSP server for real-time monitoring:

```bash
python module1_adapter\src\main.py --mode lsp --grpc-port 50051
```

Then connect your editor/IDE with LSP support.

### gRPC Server Mode (Testing)
Test the gRPC transport independently:

```bash
python module1_adapter\src\transport.py
```

## Example Output

For this code:
```python
def login(user, password):
    hashed = hash_password(password)
    return check_user(user, hashed)
```

The adapter emits:
```
FunctionDeclared(name="login", param_count=2, line=1)
FunctionCall(caller="login", callee="hash_password", line=2)
FunctionCall(caller="login", callee="check_user", line=3)
ReturnStatement(function="login", has_value=True, line=3)
```

## Testing

Run the standalone mode on the example file:
```bash
python module1_adapter\src\main.py --mode file --file module1_adapter\examples\sample.py
```

Expected: ~30+ IR events extracted from the sample code.

## Next Steps

- [ ] Add support for Java adapter
- [ ] Add support for Go adapter
- [ ] Add support for C adapter
- [ ] Add support for Rust adapter
- [ ] Implement automated tests
- [ ] Add LSP client integration examples

## Philosophy

> "I observe and report. I make no conclusions."

The adapter is intentionally simple. All intelligence lives in downstream modules that consume these IR events.
