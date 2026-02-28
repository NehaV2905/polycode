export const dummyIR = {
  nodes: [
    {
      id: "a1b2c3d4",
      node_type: { Module: { file_path: "sample.py", language: "python" } },
      metadata: { line_number: 1, file_path: "sample.py" }
    },
    {
      id: "e5f6a7b8",
      node_type: { Class: { name: "UserManager", base_classes: [] } },
      metadata: { line_number: 13, file_path: "sample.py" }
    },
    {
      id: "c9d0e1f2",
      node_type: { Function: { name: "__init__", param_count: 2, is_async: false, parent_scope: "UserManager" } },
      metadata: { line_number: 16, file_path: "sample.py" }
    },
    {
      id: "a3b4c5d6",
      node_type: { Function: { name: "connect", param_count: 1, is_async: false, parent_scope: "UserManager" } },
      metadata: { line_number: 20, file_path: "sample.py" }
    },
    {
      id: "e7f8a9b0",
      node_type: { Function: { name: "create_user", param_count: 3, is_async: false, parent_scope: "UserManager" } },
      metadata: { line_number: 28, file_path: "sample.py" }
    },
    {
      id: "c1d2e3f4",
      node_type: { Function: { name: "_insert_user", param_count: 3, is_async: false, parent_scope: "UserManager" } },
      metadata: { line_number: 38, file_path: "sample.py" }
    },
    {
      id: "a5b6c7d8",
      node_type: { Function: { name: "hash_password", param_count: 1, is_async: false, parent_scope: null } },
      metadata: { line_number: 44, file_path: "sample.py" }
    },
    {
      id: "e9f0a1b2",
      node_type: { Function: { name: "login", param_count: 2, is_async: false, parent_scope: null } },
      metadata: { line_number: 50, file_path: "sample.py" }
    },
    {
      id: "c3d4e5f6",
      node_type: { Function: { name: "check_credentials", param_count: 2, is_async: false, parent_scope: null } },
      metadata: { line_number: 67, file_path: "sample.py" }
    },
    {
      id: "a7b8c9d0",
      node_type: { Function: { name: "process_users", param_count: 1, is_async: false, parent_scope: null } },
      metadata: { line_number: 73, file_path: "sample.py" }
    },
    {
      id: "e1f2a3b4",
      node_type: { Function: { name: "validate_username", param_count: 1, is_async: false, parent_scope: null } },
      metadata: { line_number: 90, file_path: "sample.py" }
    },
    {
      id: "c5d6e7f8",
      node_type: { Function: { name: "main", param_count: 0, is_async: false, parent_scope: null } },
      metadata: { line_number: 97, file_path: "sample.py" }
    }
  ],
  edges: [
    { from: "e7f8a9b0", to: "a5b6c7d8", edge_type: { Calls: { arg_count: 1 } }, line_number: 33 },
    { from: "e7f8a9b0", to: "c1d2e3f4", edge_type: { Calls: { arg_count: 2 } }, line_number: 34 },
    { from: "e9f0a1b2", to: "a3b4c5d6", edge_type: { Calls: { arg_count: 0 } }, line_number: 53 },
    { from: "e9f0a1b2", to: "a5b6c7d8", edge_type: { Calls: { arg_count: 1 } }, line_number: 56 },
    { from: "e9f0a1b2", to: "c3d4e5f6", edge_type: { Calls: { arg_count: 2 } }, line_number: 57 },
    { from: "a7b8c9d0", to: "e1f2a3b4", edge_type: { Calls: { arg_count: 1 } }, line_number: 83 },
    { from: "c5d6e7f8", to: "a7b8c9d0", edge_type: { Calls: { arg_count: 1 } }, line_number: 102 },
    { from: "c5d6e7f8", to: "e9f0a1b2", edge_type: { Calls: { arg_count: 2 } }, line_number: 105 },
    { from: "e5f6a7b8", to: "c9d0e1f2", edge_type: "HasMember", line_number: 16 },
    { from: "e5f6a7b8", to: "a3b4c5d6", edge_type: "HasMember", line_number: 20 },
    { from: "e5f6a7b8", to: "e7f8a9b0", edge_type: "HasMember", line_number: 28 },
    { from: "e5f6a7b8", to: "c1d2e3f4", edge_type: "HasMember", line_number: 38 },
  ]
};