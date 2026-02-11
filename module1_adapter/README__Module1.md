# Module 1 - Language Adapter

## Overview

Module 1 is a **multi-language adapter** that extracts comprehensive semantic facts from source code in 6 languages: **Python, C, Go, Java, Rust, and Ruby**. It operates on the principle of **observation without analysis**.

## ✨ What's New (V2)

**Comprehensive Event Extraction** - Now extracts ~50-110 events per file:
- ✅ **6 Languages Supported**: Python, C, Go, Java, Rust, Ruby
- ✅ **15+ Event Types**: Functions, classes, imports, control structures, exceptions, lambdas, async/await, member access
- ✅ **Multi-Language Parser**: Unified tree-sitter based parser for all languages
- ✅ **Cross-Platform**: Windows, macOS, Linux fully supported

## Platform Compatibility

✅ **Fully Cross-Platform**

Module 1 works identically across all major operating systems:
- **Windows** (10, 11)
- **macOS** (10.15+)
- **Linux** (Ubuntu, Debian, Fedora, etc.)

**Key Features:**
- URI to path conversion handles platform-specific formats
- File operations use `pathlib` for OS-agnostic path handling
- Line endings handled automatically (CRLF on Windows, LF on Unix)
- No platform-specific dependencies

See [CROSS_PLATFORM__Module1.md](CROSS_PLATFORM__Module1.md) for details.

## What It Does

1. **Listens**: Monitors file changes via LSP (Language Server Protocol)
2. **Extracts**: Walks the AST to identify code constructs (functions, classes, imports, control structures, etc.)
3. **Reports**: Emits IR events via gRPC

## What It Does NOT Do

- ❌ Analysis or inference
- ❌ Type checking
- ❌ Optimization suggestions
- ❌ Code transformation

It's deliberately "dumb" - just observing and reporting facts.

## Architecture

```
Source Code (Python/Java/Go/C/Rust/Ruby)
         ↓
    LSP Server
         ↓
    Tree-sitter Parser (parsers/tree_sitter_adapter.py)
         ↓
    IR Events (Protocol Buffers)
         ↓
    gRPC Stream (transport.py)
         ↓
    Downstream Modules
```

## Supported Languages & Events

| Language | Events Extracted | Test File |
|----------|------------------|-----------|
| **Python** | ~110 | `comprehensive_test.py` |
| **Java** | ~45 | `ComplexJava.java` |
| **Rust** | ~34 | `complex_rust.rs` |
| **C** | ~33 | `complex_c.c` |
| **Ruby** | ~23 | `complex_ruby.rb` |
| **Go** | ~11 | `sample.go` |

## Event Types

### Core Events
- `FunctionDeclared` - Regular function declarations
- `AsyncFunctionDeclared` - Async function declarations (Python, Rust, JavaScript)
- `FunctionCall` - Function invocations
- `ReturnStatement` - Return statements

### Type System
- `ClassDeclared` - Class/struct/module definitions
- `InterfaceDeclared` - Interface/trait/protocol declarations
- `EnumDeclared` - Enumeration declarations

### Control Flow
- `ControlStructure` - if, while, for, switch, try statements
- `ImportStatement` - Import/include/use/require directives

### Exception Handling
- `ThrowStatement` - Exception throwing (raise, throw)
- `CatchClause` - Exception handlers (except, catch, rescue)

### Advanced Constructs
- `LambdaDeclared` - Anonymous/lambda functions, closures, blocks
- `MemberAccess` - Property/method access (obj.field, obj.method())
- `AwaitExpression` - Async await calls

## Components

### 1. Protocol Buffer Schema (`proto/ir_events.proto`)
Defines 15+ language-agnostic IR event types.

### 2. Multi-Language Parser (`src/parsers/`)
- **`tree_sitter_adapter.py`** - Unified parser for all 6 languages using tree-sitter
- **`base.py`** - Base parser interface and IRFact data model
- **`python_standard.py`** - Legacy Python-only parser (deprecated)

### 3. Transport Layer (`src/transport.py`)
gRPC streaming implementation that converts IRFacts to protobuf messages.

### 4. Main Orchestrator (`src/main.py`)
Ties everything together, supports LSP and standalone modes.

## Usage

### Quick Start - Multi-Language Testing

Test Python (110 events):
```bash
python src\main.py --mode file --file examples\multi_lang\comprehensive_test.py
```

Test Java (45 events):
```bash
python src\main.py --mode file --file examples\multi_lang\ComplexJava.java
```

Test Rust (34 events):
```bash
python src\main.py --mode file --file examples\multi_lang\complex_rust.rs
```

Test C (33 events):
```bash
python src\main.py --mode file --file examples\multi_lang\complex_c.c
```

### Standalone File Mode
Process any supported file:
```bash
python src\main.py --mode file --file path\to\your\file.{py,java,go,c,rs,rb}
```

### LSP Server Mode (Production)
Run as an LSP server for real-time monitoring:
```bash
python src\main.py --mode lsp --grpc-port 50051
```

## Example Output

### Python Code:
```python
import os

class UserManager:
    def __init__(self):
        self.users = []
    
    def add_user(self, name):
        self.users.append(name)
        return True
```

### Extracted Events:
```
[1] ImportStatement     | module_name='os'
[3] ClassDeclared       | name='UserManager', base_classes=[]
[4] FunctionDeclared    | name='__init__', param_count=1
[5] MemberAccess        | object='self', member='users'
[7] FunctionDeclared    | name='add_user', param_count=2
[8] MemberAccess        | object='self', member='users'
[8] FunctionCall        | caller='add_user', callee='append'
[9] ReturnStatement     | function='add_user', has_value=True
```

## Installation

```bash
# Install dependencies
pip install -r requirements.txt

# Generate protobuf (if needed)
python -m grpc_tools.protoc -I proto --python_out=src --grpc_python_out=src proto/ir_events.proto
```

## Testing

See [TESTING.md](TESTING.md) for comprehensive testing guide.

**Quick Test:**
```bash
python src\main.py --mode file --file examples\multi_lang\comprehensive_test.py
```

Expected: **110 events** from Python comprehensive test file.

## Directory Structure

```
module1_adapter/
├─ proto/                   # Protocol Buffer definitions
├─ src/
│  ├─ main.py              # Main orchestrator
│  ├─ transport.py         # gRPC transport
│  └─ parsers/             # Multi-language parsers
│     ├─ base.py           # Base parser interface
│     └─ tree_sitter_adapter.py  # Unified tree-sitter parser
├─ examples/multi_lang/    # Test files for all languages
└─ tests/unit/             # Unit tests
```

## Philosophy

> "I observe and report. I make no conclusions."

The adapter is intentionally simple. All intelligence lives in downstream modules that consume these IR events.

## Next Steps

- ✅ ~~Add support for Java, Go, C, Rust, Ruby adapters~~ **DONE**
- ✅ ~~Comprehensive event extraction~~ **DONE**
- ✅ ~~Cross-platform compatibility~~ **DONE**
- [ ] Automated unit tests for all languages
- [ ] Performance benchmarking
- [ ] LSP client integration examples
