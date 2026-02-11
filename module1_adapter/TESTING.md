# Testing Guide for Module 1

## Quick Test (2 minutes)

### Step 1: Install
```bash
pip install -r requirements.txt
```

### Step 2: Run Multi-Language Test
```bash
# Python (110 events)
python src\main.py --mode file --file examples\multi_lang\comprehensive_test.py

# Java (45 events)
python src\main.py --mode file --file examples\multi_lang\ComplexJava.java

# Rust (34 events)
python src\main.py --mode file --file examples\multi_lang\complex_rust.rs

# C (33 events)
python src\main.py --mode file --file examples\multi_lang\complex_c.c
```

### Step 3: Verify Output
Look for:
- ✅ "Extracted X IR facts" message
- ✅ Events listed with line numbers and types
- ✅ No errors or warnings
- ✅ Event count matches expectations

## Supported Languages

| Language | Parser | Test File | Expected Events |
|----------|--------|-----------|-----------------|
| **Python** | tree-sitter | `comprehensive_test.py` | ~110 |
| **Java** | tree-sitter | `ComplexJava.java` | ~45 |
| **Rust** | tree-sitter | `complex_rust.rs` | ~34 |
| **C** | tree-sitter | `complex_c.c` | ~33 |
| **Ruby** | tree-sitter | `complex_ruby.rb` | ~23 |
| **Go** | tree-sitter | `sample.go` | ~11 |

## Comprehensive Event Testing

### Test 1: Python - All Events (~110 events)
```bash
python src\main.py --mode file --file examples\multi_lang\comprehensive_test.py
```

**Should extract:**
- ✅ **Imports**: `os`, `sys`, `typing`, `dataclasses`
- ✅ **Classes**: `Config`, `UserManager`
- ✅ **Functions**: `hash_password`, `check_credentials`, `get_stored_password`, `process_users`, `main`
- ✅ **Async Functions**: `fetch_user_data`, `_async_request`
- ✅ **Control Structures**: if, for, while, try
- ✅ **Exceptions**: raise `ValueError`, except `KeyError`, `ValueError`
- ✅ **Lambdas**: filter, sorted lambdas
- ✅ **Await**: async calls in async functions
- ✅ **Member Access**: `self.config`, `user["name"]`, method calls
- ✅ **Function Calls**: `hash_password()`, `print()`, etc.

### Test 2: Java - OOP & Generics (~45 events)
```bash
python src\main.py --mode file --file examples\multi_lang\ComplexJava.java
```

**Should extract:**
- ✅ **Imports**: `java.util.ArrayList`, `java.util.List`
- ✅ **Class**: `ComplexJava<T>` with generics
- ✅ **Methods**: Constructor, `getValue`, `convert`, `createList`, `process` (overloaded), `main`
- ✅ **Control**: for-each loop
- ✅ **Member Access**: `this.value`, `intBox.getValue()`
- ✅ **Calls**: Constructor calls, static methods, `System.out.println()`

### Test 3: Rust - Advanced Features (~34 events)
```bash
python src\main.py --mode file --file examples\multi_lang\complex_rust.rs
```

**Should extract:**
- ✅ **Use**: `std::collections::HashMap`
- ✅ **Structs**: `Config`, `Calculator`
- ✅ **Traits**: `Compute`
- ✅ **Enums**: `Operation`
- ✅ **Impl blocks**: implementations
- ✅ **Functions**: `process_data`, `main`
- ✅ **Closures**: map/filter closures
- ✅ **Control**: if, for, match expressions

### Test 4: C - Structs & Pointers (~33 events)
```bash
python src\main.py --mode file --file examples\multi_lang\complex_c.c
```

**Should extract:**
- ✅ **Includes**: `stdio.h`, `stdlib.h`, `string.h`
- ✅ **Structs**: `Point`, `Config`
- ✅ **Enums**: enum values
- ✅ **Functions**: `create_point`, `calculate_distance`, etc.
- ✅ **Control**: if, for, while, switch
- ✅ **Member Access**: struct field access
- ✅ **Macros**: preprocessor directives

### Test 5: Ruby - Dynamic Features (~23 events)
```bash
python src\main.py --mode file --file examples\multi_lang\complex_ruby.rb
```

**Should extract:**
- ✅ **Classes**: `Calculator` with class methods
- ✅ **Methods**: instance and singleton methods
- ✅ **Control**: if, unless, while
- ✅ **Blocks**: each, map blocks
- ✅ **Lambdas**: proc and lambda expressions
- ✅ **Exceptions**: raise and rescue

### Test 6: Go - Concurrency (~11 events)
```bash
python src\main.py --mode file --file examples\multi_lang\sample.go
```

**Should extract:**
- ✅ **Imports**: package imports
- ✅ **Types**: struct declarations
- ✅ **Functions**: functions and methods
- ✅ **Interfaces**: interface types (if present)

## Event Verification Checklist

| Feature | Syntax Example | Expected Event | All Languages |
|---------|----------------|----------------|---------------|
| **Functions** | `def login():` (Python), `void login()` (C/Java) | `FunctionDeclared` | ✅ |
| **Async** | `async def fetch():` (Python), `async fn fetch()` (Rust) | `AsyncFunctionDeclared` | Python, Rust |
| **Calls** | `print("hello")` | `FunctionCall` | ✅ |
| **Returns** | `return value` | `ReturnStatement` | ✅ |
| **Imports** | `import os`, `use std`, `#include <stdio.h>` | `ImportStatement` | ✅ |
| **Classes** | `class User:`, `struct User`, `type User struct` | `ClassDeclared` | ✅ |
| **Interfaces** | `interface Reader`, `trait Display` | `InterfaceDeclared` | Java, Go, Rust |
| **Enums** | `enum Color`, `enum class Status` | `EnumDeclared` | Java, Rust, C |
| **Control** | `if`, `for`, `while`, `try`, `switch` | `ControlStructure` | ✅ |
| **Exceptions** | `raise ValueError()`, `throw new Exception()` | `ThrowStatement` | Python, Java, Ruby |
| **Catch** | `except ValueError:`, `catch (Exception e)` | `CatchClause` | Python, Java, Ruby |
| **Lambdas** | `lambda x: x*2`, `\|x\| x*2`, `x -> x*2` | `LambdaDeclared` | ✅ |
| **Await** | `await fetch()`, `fetch().await` | `AwaitExpression` | Python, Rust |
| **Member** | `obj.field`, `obj->field`, `obj.method()` | `MemberAccess` | ✅ |

## Cross-Platform Test

```bash
python tests\unit\test_langs.py
```

**Works on:** Windows, macOS, Linux

## Performance Test

Test on large files:
```bash
# Should complete in < 1 second for most files
python src\main.py --mode file --file path\to\large_file.py
```

## Troubleshooting

### Error: "tree-sitter not found"
```bash
pip install tree-sitter tree-sitter-languages
```

### Error: "No module named 'grpc'"
```bash
pip install grpcio grpcio-tools
```

### Error: "Language not supported"
Supported extensions: `.py`, `.java`, `.go`, `.c`, `.rs`, `.rb`

### No events extracted
- Check file has valid syntax for its language
- Verify file extension is recognized
- Try with one of the example files first

### Different event count than expected
Event counts may vary slightly based on:
- Code style (inline vs multi-line)
- Comments and docstrings
- Language-specific syntax variations

## Manual Verification

**Test file (test.py):**
```python
import sys

def hello(name):
    print(f"Hello {name}")
    return True
```

**Run:**
```bash
python src\main.py --mode file --file test.py
```

**Expected events:**
1. `ImportStatement` - `sys`
2. `FunctionDeclared` - `hello` with 1 parameter
3. `FunctionCall` - `print`
4. `ReturnStatement` - with value

## Unit Tests

Run specific language tests:
```bash
# Java event breakdown
python tests\unit\test_java_events.py

# Multi-language smoke test
python tests\unit\test_langs.py
```

## Success Criteria

✅ All 6 languages parse without errors  
✅ Event counts match expectations (±10%)  
✅ Line numbers are accurate  
✅ No crashes or exceptions  
✅ Works on your OS (Windows/Mac/Linux)  
✅ All event types extracted correctly

## Advanced Testing

### Test Language-Specific Features

**Python**: Decorators, f-strings, comprehensions
**Java**: Generics, annotations, method overloading
**Rust**: Lifetimes, pattern matching, macros
**C**: Function pointers, preprocessor, unions
**Ruby**: Metaprogramming, symbols, blocks
**Go**: Goroutines, channels, defer

### Custom Test File

Create your own test file with diverse constructs and verify all events are captured.

## Next: Build Something

Once tests pass, Module 1 is ready! Try:
1. Parse your entire codebase
2. Build a cross-language call graph analyzer
3. Create a complexity tracker
4. Implement security scanners
5. Analyze async/await patterns
6. Track exception handling coverage

All using the **comprehensive IR events** Module 1 provides!
