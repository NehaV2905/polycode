"""
Comprehensive Multi-Language Test Suite
Tests enhanced IR extraction for Python, Java, Go, Rust, C, and Ruby
"""

import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from module1_adapter.parsers.python_standard import PythonStandardParser
from module1_adapter.parsers.tree_sitter_adapter import TreeSitterParser


def print_section(title):
    print("\n" + "=" * 70)
    print(f"  {title}")
    print("=" * 70)


def print_result(label, value):
    print(f"  ✓ {label:20s}: {value}")


def test_python():
    """Test Python enhanced extraction"""
    print_section("PYTHON TEST")
    
    code = '''
@app.route("/api")
@require_auth
def get_user(user_id: int, name: str = "default") -> dict:
    """
    Retrieves user information.
    
    Args:
        user_id: The user identifier
        name: Optional user name
    """
    return {"id": user_id, "name": name}

@dataclass
class User:
    """User data model."""
    name: str
    age: int
'''
    
    parser = PythonStandardParser()
    facts = parser.parse(code, "test.py")
    
    func_facts = [f for f in facts if f.fact_type == "FunctionDeclared"]
    class_facts = [f for f in facts if f.fact_type == "ClassDeclared"]
    
    if func_facts:
        func = func_facts[0]
        print("\n  Function: get_user")
        print_result("Decorators", func.data.get("decorators", []))
        print_result("Return Type", func.data.get("return_type", ""))
        print_result("Parameters", [(p["name"], p["type"]) for p in func.data.get("parameters", [])])
        print_result("Docstring", func.data.get("docstring", "")[:50] + "...")
        
    if class_facts:
        cls = class_facts[0]
        print("\n  Class: User")
        print_result("Decorators", cls.data.get("decorators", []))
        print_result("Docstring", cls.data.get("docstring", ""))
    
    return len(func_facts) > 0 and len(class_facts) > 0


def test_java():
    """Test Java enhanced extraction"""
    print_section("JAVA TEST")
    
    code = '''
public class UserService {
    @Override
    @Transactional
    public String getUserName(int userId, String defaultName) {
        return "John";
    }
    
    @RestController
    public class ApiController {
        // Controller logic
    }
}
'''
    
    parser = TreeSitterParser("java")
    facts = parser.parse(code, "UserService.java")
    
    func_facts = [f for f in facts if f.fact_type == "FunctionDeclared"]
    
    if func_facts:
        func = func_facts[0]
        print("\n  Method: getUserName")
        print_result("Annotations", func.data.get("decorators", []))
        print_result("Return Type", func.data.get("return_type", ""))
        params = func.data.get("parameters", [])
        print_result("Parameters", [(p["name"], p["type"]) for p in params])
    
    return len(func_facts) > 0


def test_go():
    """Test Go enhanced extraction"""
    print_section("GO TEST")
    
    code = '''
package main

// Add returns the sum of two integers
func Add(x int, y int) int {
    return x + y
}

// ProcessUser handles user data with multiple return values
func ProcessUser(name string, age int) (string, error) {
    return name, nil
}
'''
    
    parser = TreeSitterParser("go")
    facts = parser.parse(code, "main.go")
    
    func_facts = [f for f in facts if f.fact_type == "FunctionDeclared"]
    
    for i, func in enumerate(func_facts[:2]):
        print(f"\n  Function: {func.data.get('name')}")
        print_result("Return Type", func.data.get("return_type", ""))
        params = func.data.get("parameters", [])
        print_result("Parameters", [(p["name"], p["type"]) for p in params])
        print_result("Docstring", func.data.get("docstring", ""))
    
    return len(func_facts) >= 2


def test_rust():
    """Test Rust enhanced extraction"""
    print_section("RUST TEST")
    
    code = '''
#[derive(Debug, Clone)]
#[test]
fn calculate_sum(x: i32, y: i32) -> i32 {
    x + y
}

/// Processes user data
#[inline]
pub fn process_user(name: &str, age: u32) -> Result<String, Error> {
    Ok(name.to_string())
}
'''
    
    parser = TreeSitterParser("rust")
    facts = parser.parse(code, "lib.rs")
    
    func_facts = [f for f in facts if f.fact_type == "FunctionDeclared"]
    
    for func in func_facts[:2]:
        print(f"\n  Function: {func.data.get('name')}")
        print_result("Attributes", func.data.get("decorators", []))
        print_result("Return Type", func.data.get("return_type", ""))
        params = func.data.get("parameters", [])
        print_result("Parameters", [(p["name"], p["type"]) for p in params])
    
    return len(func_facts) >= 1


def test_c():
    """Test C enhanced extraction"""
    print_section("C TEST")
    
    code = '''
/* Calculate sum of two integers */
int add(int x, int y) {
    return x + y;
}

// Process user data
char* get_username(int user_id, const char* default_name) {
    return "John";
}
'''
    
    parser = TreeSitterParser("c")
    facts = parser.parse(code, "utils.c")
    
    func_facts = [f for f in facts if f.fact_type == "FunctionDeclared"]
    
    for func in func_facts[:2]:
        print(f"\n  Function: {func.data.get('name')}")
        print_result("Return Type", func.data.get("return_type", ""))
        params = func.data.get("parameters", [])
        print_result("Parameters", [(p["name"], p["type"]) for p in params])
        print_result("Docstring", func.data.get("docstring", ""))
    
    return len(func_facts) >= 2


def test_ruby():
    """Test Ruby enhanced extraction"""
    print_section("RUBY TEST")
    
    code = '''
class UserService
  # Retrieves user name by ID
  def get_user_name(user_id, default_name)
    "John"
  end
  
  # Process user data
  def process_user(name, age)
    { name: name, age: age }
  end
end
'''
    
    parser = TreeSitterParser("ruby")
    facts = parser.parse(code, "user_service.rb")
    
    func_facts = [f for f in facts if f.fact_type == "FunctionDeclared"]
    
    for func in func_facts[:2]:
        print(f"\n  Method: {func.data.get('name')}")
        params = func.data.get("parameters", [])
        print_result("Parameters", [p["name"] for p in params])
        print_result("Docstring", func.data.get("docstring", ""))
    
    return len(func_facts) >= 2


def main():
    print("\n" + "█" * 70)
    print("  ENHANCED IR EXTRACTION - MULTI-LANGUAGE TEST SUITE")
    print("█" * 70)
    
    results = {}
    
    try:
        results["Python"] = test_python()
    except Exception as e:
        print(f"\n  ✗ Python test failed: {e}")
        results["Python"] = False
    
    try:
        results["Java"] = test_java()
    except Exception as e:
        print(f"\n  ✗ Java test failed: {e}")
        results["Java"] = False
    
    try:
        results["Go"] = test_go()
    except Exception as e:
        print(f"\n  ✗ Go test failed: {e}")
        results["Go"] = False
    
    try:
        results["Rust"] = test_rust()
    except Exception as e:
        print(f"\n  ✗ Rust test failed: {e}")
        results["Rust"] = False
    
    try:
        results["C"] = test_c()
    except Exception as e:
        print(f"\n  ✗ C test failed: {e}")
        results["C"] = False
    
    try:
        results["Ruby"] = test_ruby()
    except Exception as e:
        print(f"\n  ✗ Ruby test failed: {e}")
        results["Ruby"] = False
    
    # Summary
    print_section("TEST SUMMARY")
    for lang, passed in results.items():
        status = "✅ PASSED" if passed else "❌ FAILED"
        print(f"  {lang:15s}: {status}")
    
    passed_count = sum(results.values())
    total_count = len(results)
    
    print(f"\n  Total: {passed_count}/{total_count} languages passed")
    print("\n" + "█" * 70 + "\n")
    
    return passed_count == total_count


if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
