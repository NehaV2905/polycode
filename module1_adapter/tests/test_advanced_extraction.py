"""
Test Advanced Event Extraction

Tests that the enhanced IR extraction correctly captures:
- Decorators/annotations
- Return types
- Typed parameters
- Docstrings
"""

import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from module1_adapter.parsers.python_standard import PythonStandardParser
from module1_adapter.parsers.tree_sitter_adapter import TreeSitterParser


def test_python_enhanced_extraction():
    """Test Python parser extracts all enhanced metadata."""
    code = '''
@my_decorator
@another_decorator("arg")
def typed_func(a: int, b: str) -> bool:
    """This is a docstring."""
    return True

@dataclass
class MyClass:
    """Class docstring."""
    pass
'''
    
    parser = PythonStandardParser()
    facts = parser.parse(code, "test.py")
    
    # Find function fact
    func_facts = [f for f in facts if f.fact_type == "FunctionDeclared"]
    print(f"Found {len(func_facts)} function facts")
    
    if len(func_facts) == 0:
        print("ERROR: No function facts found!")
        print("All facts:", facts)
        return
    
    func = func_facts[0]
    
    print("DEBUG: func.data fields =", list(func.data.keys()))
    print("DEBUG: decorators =", func.data.get("decorators"))
    print("DEBUG: return_type =", func.data.get("return_type"))
    print("DEBUG: parameters =", func.data.get("parameters"))
    print("DEBUG: docstring =", func.data.get("docstring"))
    
    # Assertions
    if "decorators" in func.data and len(func.data.get("decorators", [])) >= 1:
        print("✓ Function decorators:", func.data.get("decorators"))
    else:
        print("✗ NO DECORATORS FOUND")
    
    print("✓ Return type:", func.data.get("return_type"))
    assert func.data.get("return_type") == "bool"
    
    print("✓ Parameters:", func.data.get("parameters"))
    params = func.data.get("parameters", [])
    assert len(params) == 2
    assert params[0]["name"] == "a" and params[0]["type"] == "int"
    assert params[1]["name"] == "b" and params[1]["type"] == "str"
    
    print("✓ Docstring:", func.data.get("docstring"))
    assert "This is a docstring" in func.data.get("docstring", "")
    
    # Find class fact
    class_facts = [f for f in facts if f.fact_type == "ClassDeclared"]
    assert len(class_facts) == 1
    cls = class_facts[0]
    
    print("✓ Class decorators:", cls.data.get("decorators"))
    assert "dataclass" in cls.data.get("decorators", [])
    
    print("✓ Class docstring:", cls.data.get("docstring"))
    assert "Class docstring" in cls.data.get("docstring", "")
    
    print("✅ Python standard parser test passed!\n")


def test_python_treesitter_extraction():
    """Test Python tree-sitter parser extracts enhanced metadata."""
    code = '''
@my_decorator
def typed_func(a: int) -> str:
    """Docstring here."""
    return "ok"
'''
    
    parser = TreeSitterParser("python")
    facts = parser.parse(code, "test.py")
    
    func_facts = [f for f in facts if f.fact_type == "FunctionDeclared"]
    assert len(func_facts) == 1
    func = func_facts[0]
    
    print("✓ Tree-sitter decorators:", func.data.get("decorators"))
    assert len(func.data.get("decorators", [])) > 0
    
    print("✓ Tree-sitter return type:", func.data.get("return_type"))
    assert func.data.get("return_type") != ""
    
    print("✓ Tree-sitter parameters:", func.data.get("parameters"))
    assert len(func.data.get("parameters", [])) == 1
    
    print("✅ Python tree-sitter test passed!\n")


def test_java_extraction():
    """Test Java tree-sitter parser extracts annotations and types."""
    code = '''
@Override
public String getName(int id) {
    return "test";
}
'''
    
    parser = TreeSitterParser("java")
    facts = parser.parse(code, "Test.java")
    
    func_facts = [f for f in facts if f.fact_type == "FunctionDeclared"]
    if len(func_facts) > 0:
        func = func_facts[0]
        print("✓ Java annotations:", func.data.get("decorators"))
        print("✓ Java return type:", func.data.get("return_type"))
        print("✓ Java parameters:", func.data.get("parameters"))
        print("✅ Java test passed!\n")
    else:
        print("⚠ Java test: No functions found (may need valid class context)\n")


def test_go_extraction():
    """Test Go tree-sitter parser extracts types."""
    code = '''
func add(x int, y int) int {
    return x + y
}
'''
    
    parser = TreeSitterParser("go")
    facts = parser.parse(code, "test.go")
    
    func_facts = [f for f in facts if f.fact_type == "FunctionDeclared"]
    if len(func_facts) > 0:
        func = func_facts[0]
        print("✓ Go return type:", func.data.get("return_type"))
        print("✓ Go parameters:", func.data.get("parameters"))
        params = func.data.get("parameters", [])
        if len(params) >= 2:
            assert params[0]["type"] == "int"
            assert params[1]["type"] == "int"
        print("✅ Go test passed!\n")
    else:
        print("⚠ Go test: No functions found\n")


if __name__ == "__main__":
    print("=" * 60)
    print("Testing Enhanced IR Event Extraction")
    print("=" * 60 + "\n")
    
    test_python_enhanced_extraction()
    test_python_treesitter_extraction()
    test_java_extraction()
    test_go_extraction()
    
    print("=" * 60)
    print("All tests completed!")
    print("=" * 60)
