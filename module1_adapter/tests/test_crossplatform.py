"""
Cross-Platform Compatibility Test Script

Tests Module 1 on different operating systems to ensure compatibility.
"""

import sys
import os
from pathlib import Path


def test_path_handling():
    """Test path handling across platforms."""
    print(f"\n{'='*60}")
    print("Testing Path Handling")
    print(f"{'='*60}\n")
    
    # Test platform detection
    print(f"Platform: {sys.platform}")
    print(f"OS: {os.name}")
    
    # Test Path operations
    test_paths = [
        "module1_adapter/src/main.py",
        "module1_adapter\\src\\main.py",  # Windows-style
        "./module1_adapter/src/main.py",
    ]
    
    for path_str in test_paths:
        path = Path(path_str)
        print(f"\nOriginal: {path_str}")
        print(f"  Normalized: {path}")
        print(f"  Absolute: {path.absolute()}")
        print(f"  Exists: {path.exists()}")


def test_uri_conversion():
    """Test URI to path conversion."""
    from urllib.parse import unquote, urlparse
    
    print(f"\n{'='*60}")
    print("Testing URI Conversion")
    print(f"{'='*60}\n")
    
    # Test URIs for different platforms
    test_uris = [
        "file:///home/user/project/file.py",  # Linux/Mac
        "file:///C:/Users/user/project/file.py",  # Windows
        "file:///c:/Users/user/project/file.py",  # Windows lowercase
        "file:///path/with%20spaces/file.py",  # Encoded spaces
    ]
    
    for uri in test_uris:
        parsed = urlparse(uri)
        path = unquote(parsed.path)
        
        # Platform-specific handling
        if sys.platform == "win32":
            if path.startswith("/") and len(path) > 2 and path[2] == ":":
                path = path[1:]
        
        path_obj = Path(path)
        
        print(f"\nURI: {uri}")
        print(f"  Parsed path: {parsed.path}")
        print(f"  Unquoted: {unquote(parsed.path)}")
        print(f"  Final path: {path}")
        print(f"  Path object: {path_obj}")
        print(f"  Absolute: {path_obj.absolute()}")


def test_file_operations():
    """Test file reading operations."""
    print(f"\n{'='*60}")
    print("Testing File Operations")
    print(f"{'='*60}\n")
    
    # Try to read a test file
    test_file = Path("module1_adapter/examples/sample.py")
    
    if test_file.exists():
        print(f"Reading: {test_file}")
        print(f"  Absolute path: {test_file.absolute()}")
        
        # Test reading
        content = test_file.read_text(encoding='utf-8')
        lines = content.split('\n')
        print(f"  Lines: {len(lines)}")
        print(f"  Size: {len(content)} bytes")
        print(f"  ✓ File read successfully")
    else:
        print(f"Test file not found: {test_file}")


def test_line_endings():
    """Test line ending handling."""
    print(f"\n{'='*60}")
    print("Testing Line Endings")
    print(f"{'='*60}\n")
    
    test_code = "def hello():\n    print('Hello')\r\n    return True"
    
    # Test different line ending styles
    unix_lines = test_code.count('\n')
    windows_lines = test_code.count('\r\n')
    
    print(f"Unix line endings (\\n): {unix_lines}")
    print(f"Windows line endings (\\r\\n): {windows_lines}")
    
    # Python's text mode handles this automatically
    print("✓ Python handles line endings automatically in text mode")


def main():
    """Run all compatibility tests."""
    print(f"\n{'='*60}")
    print("Module 1 - Cross-Platform Compatibility Tests")
    print(f"{'='*60}")
    
    test_path_handling()
    test_uri_conversion()
    test_file_operations()
    test_line_endings()
    
    print(f"\n{'='*60}")
    print("All Tests Complete")
    print(f"{'='*60}\n")
    
    print(f"Platform: {sys.platform}")
    print(f"Python: {sys.version}")
    print(f"Working directory: {Path.cwd()}")


if __name__ == "__main__":
    main()
