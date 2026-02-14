"""
Parsers Package

Exports parsers and provides a factory method to get the correct parser.
"""

import os
from .base import BaseParser
from .python_standard import PythonStandardParser
from .tree_sitter_adapter import TreeSitterParser

# Mapping of file extensions to language names (for Tree-sitter)
EXTENSION_MAP = {
    ".py": "python",
    ".go": "go",
    ".java": "java",
    ".c": "c",
    ".h": "c",
    ".rs": "rust",
    ".rb": "ruby",
}

def get_parser(language: str = None, file_path: str = None) -> BaseParser:
    """
    Factory to get the correct parser instance.
    
    Args:
        language: Explicit language ID (e.g. 'python', 'go')
        file_path: File path to infer language from extension
    
    Returns:
        BaseParser instance
    """
    
    # 1. Infer language if not provided
    if not language and file_path:
        _, ext = os.path.splitext(file_path)
        language = EXTENSION_MAP.get(ext.lower(), "python") # Default to Python
    
    language = language.lower() if language else "python"
    
    # 2. Select parser
    if language == "python":
        # We can choose between Standard and Tree-sitter for Python
        # Using Tree-sitter as requested in the new plan, 
        # but defaulting to Standard if Tree-sitter fails to load?
        # For now, let's respect the "Unifying on Tree-sitter" goal
        # BUT, to keep it safe, let's stick to standard if explicit 'python'
        # UNLESS we want to fully switch.
        # Plan said: "Python... Unifying on Tree-sitter".
        # Let's try Tree-sitter, fallback to standard?
        # For simplicity, let's return TreeSitter for everything if possible.
        try:
           return TreeSitterParser("python")
        except Exception as e:
           print(f"[Info] Tree-sitter python init failed ({e}), falling back to ast module.")
           return PythonStandardParser()
        
    
    # 3. For other languages, use TreeSitter
    return TreeSitterParser(language)
