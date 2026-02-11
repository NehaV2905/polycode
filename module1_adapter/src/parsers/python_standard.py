"""
Python Standard Parser
Uses the built-in `ast` module.
"""

import ast
from typing import List, Optional
from .base import BaseParser, IRFact

class PythonASTWalker(ast.NodeVisitor):
    """
    Walks a Python AST and extracts IR-relevant facts.
    """
    
    def __init__(self, source_file: str):
        self.source_file = source_file
        self.facts: List[IRFact] = []
        self.current_function: Optional[str] = None
        self.current_class: Optional[str] = None
        self.scope_stack: List[str] = []  # Track nested scopes
        
    def visit_FunctionDef(self, node: ast.FunctionDef):
        """Extract function declaration."""
        parent_scope = self.current_class or ""
        
        fact = IRFact(
            fact_type="FunctionDeclared",
            data={
                "name": node.name,
                "param_count": len(node.args.args),
                "parent_scope": parent_scope,
            },
            line_number=node.lineno
        )
        self.facts.append(fact)
        
        previous_function = self.current_function
        self.current_function = node.name
        self.scope_stack.append(node.name)
        self.generic_visit(node)
        self.scope_stack.pop()
        self.current_function = previous_function
    
    def visit_AsyncFunctionDef(self, node: ast.AsyncFunctionDef):
        """Extract async function declaration."""
        parent_scope = self.current_class or ""
        
        fact = IRFact(
            fact_type="AsyncFunctionDeclared",
            data={
                "name": node.name,
                "param_count": len(node.args.args),
                "parent_scope": parent_scope,
            },
            line_number=node.lineno
        )
        self.facts.append(fact)
        
        previous_function = self.current_function
        self.current_function = node.name
        self.scope_stack.append(node.name)
        self.generic_visit(node)
        self.scope_stack.pop()
        self.current_function = previous_function
    
    def visit_ClassDef(self, node: ast.ClassDef):
        """Extract class declaration."""
        base_classes = [self._get_name(base) for base in node.bases]
        
        fact = IRFact(
            fact_type="ClassDeclared",
            data={
                "name": node.name,
                "base_classes": base_classes,
            },
            line_number=node.lineno
        )
        self.facts.append(fact)
        
        previous_class = self.current_class
        self.current_class = node.name
        self.scope_stack.append(node.name)
        self.generic_visit(node)
        self.scope_stack.pop()
        self.current_class = previous_class
    
    def visit_Call(self, node: ast.Call):
        """Extract function call."""
        callee_name = self._get_name(node.func)
        
        fact = IRFact(
            fact_type="FunctionCall",
            data={
                "caller_function": self.current_function or "<module>",
                "callee_name": callee_name,
                "arg_count": len(node.args) + len(node.keywords),
            },
            line_number=node.lineno
        )
        self.facts.append(fact)
        self.generic_visit(node)
    
    def visit_Return(self, node: ast.Return):
        """Extract return statement."""
        fact = IRFact(
            fact_type="ReturnStatement",
            data={
                "function_name": self.current_function or "<module>",
                "has_value": node.value is not None,
            },
            line_number=node.lineno
        )
        self.facts.append(fact)
        self.generic_visit(node)
    
    def visit_Import(self, node: ast.Import):
        """Extract import statement."""
        for alias in node.names:
            fact = IRFact(
                fact_type="ImportStatement",
                data={
                    "module_name": alias.name,
                    "imported_names": [],
                    "is_wildcard": False,
                },
                line_number=node.lineno
            )
            self.facts.append(fact)
        self.generic_visit(node)
    
    def visit_ImportFrom(self, node: ast.ImportFrom):
        """Extract from...import statement."""
        module_name = node.module or ""
        imported_names = [alias.name for alias in node.names]
        is_wildcard = '*' in imported_names
        
        fact = IRFact(
            fact_type="ImportStatement",
            data={
                "module_name": module_name,
                "imported_names": imported_names,
                "is_wildcard": is_wildcard,
            },
            line_number=node.lineno
        )
        self.facts.append(fact)
        self.generic_visit(node)
    
    def visit_If(self, node: ast.If):
        """Extract if statement."""
        fact = IRFact(
            fact_type="ControlStructure",
            data={
                "type": "IF",
                "parent_function": self.current_function or "<module>",
                "has_else": len(node.orelse) > 0,
            },
            line_number=node.lineno
        )
        self.facts.append(fact)
        self.generic_visit(node)
    
    def visit_While(self, node: ast.While):
        """Extract while loop."""
        fact = IRFact(
            fact_type="ControlStructure",
            data={
                "type": "WHILE",
                "parent_function": self.current_function or "<module>",
                "has_else": len(node.orelse) > 0,
            },
            line_number=node.lineno
        )
        self.facts.append(fact)
        self.generic_visit(node)
    
    def visit_For(self, node: ast.For):
        """Extract for loop."""
        fact = IRFact(
            fact_type="ControlStructure",
            data={
                "type": "FOR",
                "parent_function": self.current_function or "<module>",
                "has_else": len(node.orelse) > 0,
            },
            line_number=node.lineno
        )
        self.facts.append(fact)
        self.generic_visit(node)
    
    def visit_Try(self, node: ast.Try):
        """Extract try statement."""
        fact = IRFact(
            fact_type="ControlStructure",
            data={
                "type": "TRY",
                "parent_function": self.current_function or "<module>",
                "has_else": len(node.orelse) > 0,
            },
            line_number=node.lineno
        )
        self.facts.append(fact)
        self.generic_visit(node)
    
    def visit_Assign(self, node: ast.Assign):
        """Extract variable assignment."""
        for target in node.targets:
            if isinstance(target, ast.Name):
                scope = self.current_function or self.current_class or "global"
                
                fact = IRFact(
                    fact_type="VariableAssignment",
                    data={
                        "variable_name": target.id,
                        "scope": scope,
                    },
                    line_number=node.lineno
                )
                self.facts.append(fact)
        self.generic_visit(node)
    
    def visit_Lambda(self, node: ast.Lambda):
        """Extract lambda/anonymous function declaration."""
        fact = IRFact(
            fact_type="LambdaDeclared",
            data={
                "param_count": len(node.args.args),
                "parent_function": self.current_function or "<module>",
            },
            line_number=node.lineno
        )
        self.facts.append(fact)
        self.generic_visit(node)
    
    def visit_Raise(self, node: ast.Raise):
        """Extract throw/raise statement."""
        exception_type = "<unknown>"
        has_message = False
        
        if node.exc:
            exception_type = self._get_name(node.exc)
            if isinstance(node.exc, ast.Call) and node.exc.args:
                has_message = True
        
        fact = IRFact(
            fact_type="ThrowStatement",
            data={
                "exception_type": exception_type,
                "parent_function": self.current_function or "<module>",
                "has_message": has_message,
            },
            line_number=node.lineno
        )
        self.facts.append(fact)
        self.generic_visit(node)
    
    def visit_ExceptHandler(self, node: ast.ExceptHandler):
        """Extract except/catch clause."""
        exception_types = []
        is_catch_all = False
        
        if node.type:
            if isinstance(node.type, ast.Tuple):
                exception_types = [self._get_name(exc) for exc in node.type.elts]
            else:
                exception_types = [self._get_name(node.type)]
        else:
            is_catch_all = True
        
        fact = IRFact(
            fact_type="CatchClause",
            data={
                "exception_types": exception_types,
                "parent_function": self.current_function or "<module>",
                "is_catch_all": is_catch_all,
            },
            line_number=node.lineno
        )
        self.facts.append(fact)
        self.generic_visit(node)
    
    def visit_Attribute(self, node: ast.Attribute):
        """Extract member/attribute access."""
        object_name = self._get_name(node.value)
        member_name = node.attr
        
        fact = IRFact(
            fact_type="MemberAccess",
            data={
                "object_name": object_name,
                "member_name": member_name,
                "parent_function": self.current_function or "<module>",
                "is_method_call": False,
            },
            line_number=node.lineno
        )
        self.facts.append(fact)
        self.generic_visit(node)
    
    def visit_Await(self, node: ast.Await):
        """Extract await expression."""
        awaited_function = "<unknown>"
        
        if isinstance(node.value, ast.Call):
            awaited_function = self._get_name(node.value.func)
        else:
            awaited_function = self._get_name(node.value)
        
        fact = IRFact(
            fact_type="AwaitExpression",
            data={
                "awaited_function": awaited_function,
                "parent_function": self.current_function or "<module>",
            },
            line_number=node.lineno
        )
        self.facts.append(fact)
        self.generic_visit(node)

    def _get_name(self, node) -> str:
        """Extract name from various node types."""
        if isinstance(node, ast.Name):
            return node.id
        elif isinstance(node, ast.Attribute):
            value_name = self._get_name(node.value)
            return f"{value_name}.{node.attr}"
        elif isinstance(node, ast.Call):
            return self._get_name(node.func)
        else:
            return "<unknown>"


class PythonStandardParser(BaseParser):
    """
    Implementation of BaseParser for Python using standard AST.
    """
    
    def parse(self, source_code: str, file_path: str) -> List[IRFact]:
        try:
            tree = ast.parse(source_code, filename=file_path)
            walker = PythonASTWalker(file_path)
            walker.visit(tree)
            return walker.facts
        except SyntaxError as e:
            print(f"Syntax error in {file_path}: {e}")
            return []
