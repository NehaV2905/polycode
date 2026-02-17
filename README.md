Module 3: Analysis Engine (Language-Agnostic Intelligence)
Overview

Module 3 is the language-agnostic reasoning engine of the PolyCode system.

It does not parse source code and does not understand syntax.
Instead, it consumes the Intermediate Representation (IR) graph produced by Module 2 and performs graph-based static analysis.

This module operates purely on structural relationships between program entities.

Architecture
Module 1 (Language Adapters)
         ↓ (gRPC IREvents)
Module 2 (Rust)
    - GraphBuilder
    - IRGraph (petgraph)
    - Query API
         ↓ (exported IR JSON)
Module 3 (Python)
    - Graph Loader
    - Analysis Engine
    - Structured Results


Module 3 receives a serialized IR graph from Module 2 and performs reasoning using graph algorithms.

Input

Module 3 expects a JSON file exported from Module 2:

ir.json


Example format:

{
  "nodes": [
    {
      "id": "uuid-1",
      "node_type": "Function",
      "display_name": "login",
      "metadata": {
        "file_path": "auth.rs",
        "line": 12
      }
    }
  ],
  "edges": [
    {
      "from": "uuid-1",
      "to": "uuid-2",
      "edge_type": "Calls"
    }
  ]
}


The analysis engine only interprets relationships such as:

Calls

Imports

InheritsFrom

HasMember

No language-specific parsing occurs in this module.

Responsibilities

Module 3 performs:

Call graph reasoning

Dependency analysis

Dead code detection

Change impact analysis

Inheritance queries

It treats the IR as a directed graph.

Core Analyses
1. find_callers(function_id)

Returns all functions that call the given function.

Graph logic:

Traverse incoming edges of type Calls

2. find_callees(function_id)

Returns all functions called by the given function.

Graph logic:

Traverse outgoing edges of type Calls

3. find_unused_functions()

Returns functions that are never called.

Graph logic:

All function nodes

Minus nodes that appear as target of Calls edges

4. find_dependencies(file_path)

Returns modules imported by a file.

Graph logic:

Follow Imports edges from nodes in the file

5. find_dependents(module_id)

Returns files that import a module.

Graph logic:

Reverse traversal of Imports edges

6. find_subclasses(class_id)

Returns all classes inheriting from a given class.

Graph logic:

Reverse traversal of InheritsFrom edges

Design Principles
1. Language Agnostic

This module does not know:

Rust

Python

Java

C++

It only understands graph structure.

2. No Storage Responsibility

Module 3:

Does not build the graph

Does not modify the graph

Does not resolve symbols

Does not manage scope

Those are responsibilities of Module 2.

3. Pure Reasoning Layer

Module 3 performs:

Graph → Algorithms → Structured Results


No UI formatting.
No source rendering.
Only structured output.

Output Format

All analyses return structured Python dictionaries.

Example:

{
  "unused_functions": ["uuid-45", "uuid-78"]
}


This makes the engine suitable for:

CLI tools

IDE integrations

Dashboards

AI reasoning systems

Algorithms Used

Module 3 relies on classical graph theory:

Set difference

Directed graph traversal

Reverse edge traversal

Basic dependency graph analysis

No compiler-specific logic is implemented here.

Example Execution Flow

Module 2 builds IR graph in Rust.

Module 2 exports graph as ir.json.

Module 3 loads ir.json.

Module 3 constructs adjacency representation.

Analysis functions are executed.

Structured results are returned.
