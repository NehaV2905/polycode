"""Simple debug test to see what's being extracted"""
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from parsers.python_standard import PythonStandardParser

code = '''
@my_decorator
def typed_func(a: int) -> bool:
    """Docstring."""
    return True
'''

parser = PythonStandardParser()
facts = parser.parse(code, "test.py")

with open("test_output.txt", "w", encoding="utf-8") as f:
    f.write(f"Total facts: {len(facts)}\n\n")
    for fact in facts:
        f.write(f"Fact type: {fact.fact_type}\n")
        f.write(f"Data: {fact.data}\n")
        f.write(f"Data keys: {list(fact.data.keys())}\n")
        f.write("-" * 50 + "\n")

print("Output written to test_output.txt")
