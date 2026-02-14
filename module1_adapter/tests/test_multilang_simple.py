"""
Simplified Multi-Language Test - Outputs to file
"""

import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from module1_adapter.parsers.python_standard import PythonStandardParser
from module1_adapter.parsers.tree_sitter_adapter import TreeSitterParser


def run_tests():
    output = []
    output.append("=" * 70)
    output.append("ENHANCED IR EXTRACTION - MULTI-LANGUAGE TEST RESULTS")
    output.append("=" * 70)
    
    results = {}
    
    # Python Test
    try:
        code = '@decorator\ndef func(x: int) -> str:\n    """Doc."""\n    return "test"'
        parser = PythonStandardParser()
        facts = parser.parse(code, "test.py")
        func_facts = [f for f in facts if f.fact_type == "FunctionDeclared"]
        if func_facts:
            f = func_facts[0]
            output.append("\nPYTHON:")
            output.append(f"  Decorators: {f.data.get('decorators', [])}")
            output.append(f"  Return Type: {f.data.get('return_type', '')}")
            output.append(f"  Parameters: {f.data.get('parameters', [])}")
            output.append(f"  Docstring: {f.data.get('docstring', '')}")
            results["Python"] = True
        else:
            results["Python"] = False
    except Exception as e:
        output.append(f"\nPYTHON: FAILED - {e}")
        results["Python"] = False
    
    # Java Test
    try:
        code = '@Override\npublic String getName(int id) { return "test"; }'
        parser = TreeSitterParser("java")
        facts = parser.parse(code, "Test.java")
        func_facts = [f for f in facts if f.fact_type == "FunctionDeclared"]
        if func_facts:
            f = func_facts[0]
            output.append("\nJAVA:")
            output.append(f"  Annotations: {f.data.get('decorators', [])}")
            output.append(f"  Return Type: {f.data.get('return_type', '')}")
            output.append(f"  Parameters: {f.data.get('parameters', [])}")
            results["Java"] = True
        else:
            results["Java"] = False
    except Exception as e:
        output.append(f"\nJAVA: FAILED - {e}")
        results["Java"] = False
    
    # Go Test
    try:
        code = 'func add(x int, y int) int { return x + y }'
        parser = TreeSitterParser("go")
        facts = parser.parse(code, "test.go")
        func_facts = [f for f in facts if f.fact_type == "FunctionDeclared"]
        if func_facts:
            f = func_facts[0]
            output.append("\nGO:")
            output.append(f"  Return Type: {f.data.get('return_type', '')}")
            output.append(f"  Parameters: {f.data.get('parameters', [])}")
            results["Go"] = True
        else:
            results["Go"] = False
    except Exception as e:
        output.append(f"\nGO: FAILED - {e}")
        results["Go"] = False
    
    # Rust Test
    try:
        code = '#[test]\nfn test_func(x: i32) -> i32 { x }'
        parser = TreeSitterParser("rust")
        facts = parser.parse(code, "lib.rs")
        func_facts = [f for f in facts if f.fact_type == "FunctionDeclared"]
        if func_facts:
            f = func_facts[0]
            output.append("\nRUST:")
            output.append(f"  Attributes: {f.data.get('decorators', [])}")
            output.append(f"  Return Type: {f.data.get('return_type', '')}")
            output.append(f"  Parameters: {f.data.get('parameters', [])}")
            results["Rust"] = True
        else:
            results["Rust"] = False
    except Exception as e:
        output.append(f"\nRUST: FAILED - {e}")
        results["Rust"] = False
    
    # C Test
    try:
        code = 'int add(int x, int y) { return x + y; }'
        parser = TreeSitterParser("c")
        facts = parser.parse(code, "test.c")
        func_facts = [f for f in facts if f.fact_type == "FunctionDeclared"]
        if func_facts:
            f = func_facts[0]
            output.append("\nC:")
            output.append(f"  Return Type: {f.data.get('return_type', '')}")
            output.append(f"  Parameters: {f.data.get('parameters', [])}")
            results["C"] = True
        else:
            results["C"] = False
    except Exception as e:
        output.append(f"\nC: FAILED - {e}")
        results["C"] = False
    
    # Ruby Test
    try:
        code = 'def get_name(user_id)\n  "John"\nend'
        parser = TreeSitterParser("ruby")
        facts = parser.parse(code, "test.rb")
        func_facts = [f for f in facts if f.fact_type == "FunctionDeclared"]
        if func_facts:
            f = func_facts[0]
            output.append("\nRUBY:")
            output.append(f"  Parameters: {f.data.get('parameters', [])}")
            results["Ruby"] = True
        else:
            results["Ruby"] = False
    except Exception as e:
        output.append(f"\nRUBY: FAILED - {e}")
        results["Ruby"] = False
    
    # Summary
    output.append("\n" + "=" * 70)
    output.append("SUMMARY")
    output.append("=" * 70)
    for lang, passed in results.items():
        status = "PASS" if passed else "FAIL"
        output.append(f"{lang:15s}: {status}")
    
    passed = sum(results.values())
    total = len(results)
    output.append(f"\nTotal: {passed}/{total} languages passed")
    output.append("=" * 70)
    
    return "\n".join(output), passed == total


if __name__ == "__main__":
    output, success = run_tests()
    
    # Write to file
    with open("multilang_test_results.txt", "w", encoding="utf-8") as f:
        f.write(output)
    
    print(output)
    sys.exit(0 if success else 1)
