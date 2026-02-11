# Testing Guide for Module 1

## Quick Test (2 minutes)

### Step 1: Install
```bash
pip install -r requirements.txt
```

### Step 2: Run Basic Test
```bash
python module1_adapter\src\main.py --mode file --file module1_adapter\examples\sample.py
```

### Step 3: Verify Output
Look for:
- ✅ "Extracted X IR facts" message
- ✅ Events listed with line numbers
- ✅ No errors

## Detailed Testing

### Test 1: Basic Events
```bash
python module1_adapter\src\main.py --mode file --file module1_adapter\examples\sample.py
```

**Should extract (~30 events):**
- Functions: `login`, `hash_password`, `check_credentials`, etc.
- Classes: `UserManager`
- Imports: `os`, `sys`, `typing`
- Control structures: if, for, try
- Function calls: `print()`, `hash_password()`, etc.

### Test 2: Extended Events
```bash
python module1_adapter\src\main.py --mode file --file module1_adapter\examples\extended_sample.py
```

**Should extract (~50 events):**
- Async functions: `fetch_data`, `_async_request`
- Await expressions in async functions
- Exception handling: raise, except clauses
- Lambdas in filter/map/sort
- Member access: `self.config.get()`, `self.data[0]`

### Test 3: Your Own Code
```bash
python module1_adapter\src\main.py --mode file --file path\to\your\file.py
```

## Verification Checklist

| Feature | Test | Expected |
|---------|------|----------|
| **Functions** | `def login():` | `FunctionDeclared` event |
| **Async** | `async def fetch():` | `AsyncFunctionDeclared` event |
| **Calls** | `print("hello")` | `FunctionCall` event |
| **Lambdas** | `lambda x: x*2` | `LambdaDeclared` event |
| **Exceptions** | `raise ValueError()` | `ThrowStatement` event |
| **Exceptions** | `except ValueError:` | `CatchClause` event |
| **Await** | `await fetch()` | `AwaitExpression` event |
| **Member** | `obj.field` | `MemberAccess` event |
| **Classes** | `class User:` | `ClassDeclared` event |
| **Imports** | `import os` | `ImportStatement` event |

## Cross-Platform Test

```bash
python module1_adapter\tests\test_crossplatform.py
```

**Expected output:**
```
============================================================
Module 1 - Cross-Platform Compatibility Tests
============================================================

Platform: win32  # or darwin, linux
✓ Path handling works
✓ URI conversion works
✓ File operations work
✓ All Tests Complete
```

## Troubleshooting

### Error: "No module named 'grpc'"
```bash
pip install grpcio grpcio-tools
```

### Error: "File not found"
Use absolute path or check current directory:
```bash
cd f:\CapstoneProject
python module1_adapter\src\main.py --mode file --file module1_adapter\examples\sample.py
```

### Error: "Syntax error in file.py"
The source file has Python syntax errors. Fix the code first.

### No output / Silent failure
Check that the file exists and is readable:
```bash
# Windows
dir module1_adapter\examples\sample.py

# Mac/Linux
ls module1_adapter/examples/sample.py
```

## Performance Test

Test on a large file (optional):
```bash
# Find a large Python file in your system
python module1_adapter\src\main.py --mode file --file path\to\large_file.py
```

Should complete in under 1 second for most files.

## Manual Verification

Pick a simple test file and manually verify:

**Test file (test.py):**
```python
def hello():
    print("world")
    return True
```

**Run:**
```bash
python module1_adapter\src\main.py --mode file --file test.py
```

**Expected events:**
1. Line 1: `FunctionDeclared` (name="hello", param_count=0)
2. Line 2: `FunctionCall` (callee="print")  
3. Line 3: `ReturnStatement` (has_value=True)

## Success Criteria

✅ All example files parse without errors  
✅ Event counts match expectations  
✅ Line numbers are accurate  
✅ No missing imports or crashes  
✅ Works on your OS (Windows/Mac/Linux)

## Next: Build Something

Once tests pass, Module 1 is ready! Try:
1. Parse your entire codebase
2. Build a call graph analyzer
3. Create a complexity tracker
4. Implement security scanners

All using the IR events Module 1 provides.
