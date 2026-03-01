// Shared types matching the api_server response format

export interface NodeMetadata {
  file_path: string;
  line_number: number;
}

export type NodeType =
  | { Module: { file_path: string; language: string } }
  | { Function: { name: string; param_count: number; is_async: boolean; parent_scope: string | null } }
  | { Class: { name: string; base_classes: string[] } }
  | { Interface: { name: string; base_interfaces: string[] } }
  | { Enum: { name: string; member_count: number } }
  | { Variable: { name: string; scope: string } }
  | { Lambda: { param_count: number } }
  | { ControlFlow: { control_type: string } };

export interface IRNode {
  id: string;
  node_type: NodeType;
  metadata: NodeMetadata;
}

export interface IREdge {
  id: string;
  from: string;
  to: string;
  edge_type: unknown;
  line_number: number;
}

export interface IRGraph {
  nodes: IRNode[];
  edges: IREdge[];
}

export interface Suggestion {
  id: number;
  file: string;
  line: number;
  function: string;
  suggestion: string;
}

export interface AnalysisStats {
  source: string;
  files_parsed: number;
  total_findings: number;
  cap: number;
}

export interface AnalysisResult {
  ir: IRGraph;
  suggestions: Suggestion[];
  stats: AnalysisStats;
}
