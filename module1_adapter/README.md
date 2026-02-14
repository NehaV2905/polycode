# Module 1 - Language Adapter for IR Event Extraction

## Overview

Module 1 is a **multi-language adapter** that extracts comprehensive semantic facts from source code in 6 languages: **Python, C, Go, Java, Rust, and Ruby**. It provides a unified, language-agnostic Intermediate Representation (IR) stream via gRPC.

## ✨ Key Features (V2)

- ✅ **6 Languages Supported**: Python, C, Go, Java, Rust, Ruby
- ✅ **15+ Event Types**: Functions, classes, imports, control structures, exceptions, lambdas, async/await, member access
- ✅ **Professional Package Structure**: Clean, modular setup following Python best practices
- ✅ **Bulletproof OS Compatibility**: First-class support for Windows, macOS, and Linux
- ✅ **LSP & Standalone Modes**: Real-time monitoring via LSP or batch processing via CLI

## 🚀 Quick Start

### Installation

```bash
# Clone the repository and navigate to module1_adapter
pip install -r requirements.txt
```

### Standalone Usage (CLI)

Process any supported file and see extracted events:

```bash
# Windows
python src\main.py --mode file --file examples\multi_lang\comprehensive_test.py

# macOS / Linux
python src/main.py --mode file --file examples/multi_lang/comprehensive_test.py
```

### LSP Server Mode

Run as an LSP server for real-time monitoring by downstream modules:

```bash
python src/main.py --mode lsp --grpc-port 50051
```

## 🏗️ Project Structure

The project is organized as a professional Python package:

```
module1_adapter/
├── pyproject.toml           # Packaging metadata
├── requirements.txt         # Explicit dependencies
├── src/
│   ├── main.py              # CLI Entry point (Shim)
│   └── module1_adapter/     # Core package
│       ├── core/            # Orchestration & gRPC transport
│       ├── parsers/         # Tree-sitter language handlers
│       └── generated/       # gRPC/Protobuf generated code
├── proto/                   # IR Event definitions (.proto)
├── examples/                # Sample files for 6 languages
└── tests/                   # Verification suite
```

## 📂 Supported Events

Module 1 extracts ~50-110 events per file depending on the language and code complexity:

| Feature | Syntax Example | Event Type |
|---------|----------------|------------|
| **Functions** | `def login():`, `void login()` | `FunctionDeclared` |
| **Async** | `async def fetch()`, `async fn fetch()` | `AsyncFunctionDeclared` |
| **Calls** | `print("hello")` | `FunctionCall` |
| **Returns** | `return value` | `ReturnStatement` |
| **Imports** | `import os`, `use std`, `#include` | `ImportStatement` |
| **Classes** | `class User:`, `struct User` | `ClassDeclared` |
| **Interfaces** | `interface Reader`, `trait Display` | `InterfaceDeclared` |
| **Enums** | `enum Color` | `EnumDeclared` |
| **Control** | `if`, `for`, `while`, `try`, `switch` | `ControlStructure` |
| **Exceptions** | `raise Error()`, `throw new Error()` | `ThrowStatement` |
| **Catch** | `except Error:`, `catch (Error e)` | `CatchClause` |
| **Lambdas** | `lambda x: x*2`, `|x| x*2` | `LambdaDeclared` |
| **Await** | `await fetch()`, `fetch().await` | `AwaitExpression` |
| **Member** | `obj.field`, `obj.method()` | `MemberAccess` |

## 🧪 Testing

Comprehensive verification across all languages:

```bash
# Run multi-language smoke test
python tests/test_all_languages.py

# Test advanced extraction (Types, Parameters, Docstrings)
python tests/test_advanced_extraction.py
```

## 🌐 Platform Compatibility

✅ **Windows** (Auto-handles URI formats and CRLF)  
✅ **macOS** (Optimized Tree-sitter loading)  
✅ **Linux** (Standard path compliance)  

All path operations use `pathlib` to ensure consistent behavior across all environments.

## 📜 Philosophy

> "I observe and report. I make no conclusions."

The adapter is intentionally simple. All intelligence lives in downstream modules that consume the high-fidelity IR event stream.

## 📖 Testing Guide

To test the IR extraction for each language, use the `src/main.py` entry point with the high-fidelity sample files:

### 🐍 Python
```powershell
python src/main.py --mode file --file examples/multi_lang/comprehensive_test.py
```

### ☕ Java
```powershell
python src/main.py --mode file --file examples/multi_lang/ComplexJava.java
```

### 🦀 Rust
```powershell
python src/main.py --mode file --file examples/multi_lang/complex_rust.rs
```

### 💎 Ruby
```powershell
python src/main.py --mode file --file examples/multi_lang/complex_ruby.rb
```

### 🔵 Go
```powershell
python src/main.py --mode file --file examples/multi_lang/sample.go
```

### ⚡ C
```powershell
python src/main.py --mode file --file examples/multi_lang/complex_c.c
```

> [!TIP]
> **Version Check**: Ensure you have installed the updated dependencies first using `pip install -r requirements.txt`.