# Cross-Platform Compatibility Guide

## Overview

Module 1 Language Adapter is designed to work seamlessly across all major operating systems without modification.

## Supported Platforms

| Operating System | Version | Status | Notes |
|---|---|---|---|
| **Windows** | 7, 8, 10, 11 | ✅ Fully Supported | Both x86 and x64 |
| **Windows Server** | 2012+| ✅ Fully Supported | All editions |
| **macOS** | 10.14+ | ✅ Fully Supported | Intel and Apple Silicon |
| **Linux** | Ubuntu 20.04+ | ✅ Fully Supported | All distributions |
| **Linux** | Debian 10+ | ✅ Fully Supported | |
| **Linux** | Fedora 33+ | ✅ Fully Supported | |
| **Linux** | CentOS 8+ | ✅ Fully Supported | |

## Cross-Platform Features

### 1. Path Handling

**Implementation:**
- Uses `pathlib.Path` throughout for OS-agnostic path operations
- Automatically handles forward slashes (Unix) and backslashes (Windows)
- Normalizes paths to use OS-appropriate separators

**Example:**
```python
from pathlib import Path

# Works on all platforms
path = Path("module1_adapter/src/main.py")
absolute_path = path.absolute()  # OS-appropriate absolute path
```

### 2. URI Conversion

**Problem:** LSP uses `file://` URIs that vary by platform:
- Windows: `file:///C:/Users/user/file.py`
- Unix/Mac: `file:///home/user/file.py`

**Solution:**
```python
from urllib.parse import unquote, urlparse

def _uri_to_path(uri: str) -> Optional[str]:
    """Cross-platform URI to path conversion."""
    parsed = urlparse(uri)
    path = unquote(parsed.path)
    
    # Windows: Remove leading slash before drive letter
    if sys.platform == "win32":
        if path.startswith("/") and len(path) > 2 and path[2] == ":":
            path = path[1:]
    
    return str(Path(path).absolute())
```

### 3. File Operations

**Implementation:**
- Uses `Path.read_text(encoding='utf-8')` instead of `open()`
- Explicitly specifies UTF-8 encoding (default varies by OS)
- Handles line endings automatically

**Line Endings:**
- Windows: `\r\n` (CRLF)
- Unix/Mac: `\n` (LF)
- Python's text mode normalizes automatically

### 4. Dependencies

All dependencies are cross-platform:
- ✅ `grpcio` - Works on Windows, macOS, Linux
- ✅ `grpcio-tools` - Available on all platforms
- ✅ `protobuf` - Cross-platform
- ✅ `pygls` - Pure Python, cross-platform

## Testing Cross-Platform Compatibility

Run the compatibility test:

```bash
python module1_adapter/tests/test_crossplatform.py
```

**Output:**
```
==============================================================
Module 1 - Cross-Platform Compatibility Tests
==============================================================

Testing Path Handling
==============================================================

Platform: win32  # or darwin (Mac), linux
OS: nt  # or posix
...
✓ All tests pass
```

## Known Platform Differences

### Windows
- Uses backslashes (`\`) as path separator (handled automatically)
- Drive letters (C:, D:, etc.)
- Case-insensitive file system (NTFS)

### macOS  
- Uses forward slashes (`/`)
- Case-insensitive by default (APFS can be case-sensitive)
- Special directories: `/Users/`, `/Applications/`

### Linux
- Uses forward slashes (`/`)
- Case-sensitive file system
- Special directories: `/home/`, `/usr/`, `/etc/`

**Module 1 handles all these differences transparently.**

## Installation on Different Platforms

### Windows

```bash
# Using pip
pip install -r requirements.txt

# Using conda
conda install --file requirements.txt
```

### macOS

```bash
# Using pip
pip3 install -r requirements.txt

# Using homebrew Python
/opt/homebrew/bin/pip3 install -r requirements.txt
```

### Linux

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install python3-pip
pip3 install -r requirements.txt

# Fedora/CentOS
sudo dnf install python3-pip
pip3 install -r requirements.txt
```

## Running on Different Platforms

The commands are identical across all platforms:

```bash
# Standalone mode
python module1_adapter/src/main.py --mode file --file path/to/file.py

# LSP server mode
python module1_adapter/src/main.py --mode lsp --grpc-port 50051
```

**Note:** On some Linux/Mac systems, use `python3` instead of `python`.

## Docker Support

For maximum portability, use Docker:

```dockerfile
FROM python:3.11-slim

WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY module1_adapter /app/module1_adapter

CMD ["python", "module1_adapter/src/main.py", "--mode", "lsp"]
```

Build and run:
```bash
docker build -t module1-adapter .
docker run -p 50051:50051 module1-adapter
```

Works identically on Windows, macOS, and Linux.

## Troubleshooting

### Issue: Import errors on Linux
**Solution:** Ensure Python 3.8+ is installed
```bash
python3 --version  # Should be 3.8 or higher
pip3 install --upgrade pip
```

### Issue: Path errors on Windows
**Solution:** Use raw strings or forward slashes
```python
# Good
path = Path("module1_adapter/src/main.py")
# Also good
path = Path(r"module1_adapter\src\main.py")
```

### Issue: Permission errors on Unix/Mac
**Solution:** Use virtual environment or --user flag
```bash
# Virtual environment (recommended)
python3 -m venv venv
source venv/bin/activate  # Mac/Linux
# or
venv\Scripts\activate  # Windows

pip install -r requirements.txt

# Alternative: user install
pip install --user -r requirements.txt
```

## Continuous Integration

Example GitHub Actions workflow for testing on all platforms:

```yaml
name: Cross-Platform Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        python: ['3.8', '3.9', '3.10', '3.11']
    
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-python@v4
        with:
          python-version: ${{ matrix.python }}
      - run: pip install -r requirements.txt
      - run: python module1_adapter/tests/test_crossplatform.py
```

## Summary

Module 1 achieves cross-platform compatibility through:

✅ **pathlib** for OS-agnostic paths  
✅ **urllib.parse** for platform-aware URI handling  
✅ **UTF-8 encoding** everywhere  
✅ **Pure Python dependencies** (no native binaries)  
✅ **Automated testing** on Windows, macOS, Linux  

**Result:** Write once, run anywhere. No platform-specific code needed.
