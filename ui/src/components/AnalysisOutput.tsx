import type { IRGraph, IRNode } from "../types";

const getModuleData = (n: IRNode) => "Module"   in n.node_type ? (n.node_type as any).Module   : null;
const getClassData  = (n: IRNode) => "Class"    in n.node_type ? (n.node_type as any).Class    : null;
const getFnData     = (n: IRNode) => "Function" in n.node_type ? (n.node_type as any).Function : null;

export default function AnalysisOutput({ ir }: { ir: IRGraph }) {
  const modules   = ir.nodes.filter(n => "Module"   in n.node_type);
  const classes   = ir.nodes.filter(n => "Class"    in n.node_type);
  const functions = ir.nodes.filter(n => "Function" in n.node_type);

  const callEdges   = ir.edges.filter(e => typeof e.edge_type === "object" && e.edge_type !== null && "Calls" in (e.edge_type as object));
  const memberEdges = ir.edges.filter(e => e.edge_type === "HasMember");

  const calledIds       = new Set(callEdges.map(e => e.to));
  const unusedFunctions = functions.filter(fn => !calledIds.has(fn.id));

  return (
    <section className="analysis-output">
      <h2>Code Summary</h2>

      <h3>Modules</h3>
      <ul>
        {modules.map(n => {
          const m = getModuleData(n);
          if (!m) return null;
          return (
            <li key={n.id}>
              {m.file_path}
              <span className="module">{m.language} · line {n.metadata.line_number}</span>
            </li>
          );
        })}
      </ul>

      <h3>Classes</h3>
      <ul>
        {classes.map(n => {
          const c = getClassData(n);
          if (!c) return null;
          return (
            <li key={n.id}>
              {c.name}
              <span className="module">line {n.metadata.line_number}</span>
            </li>
          );
        })}
      </ul>

      <h3>Functions</h3>
      <ul>
        {functions.map(n => {
          const fn = getFnData(n);
          if (!fn) return null;
          return (
            <li key={n.id}>
              {fn.name}
              <span className="module">
                {fn.parent_scope ? `${fn.parent_scope} · ` : ""}
                {fn.param_count} params · line {n.metadata.line_number}
              </span>
            </li>
          );
        })}
      </ul>

      <h3>Unused Functions</h3>
      {unusedFunctions.length === 0 ? (
        <p className="no-unused">None found</p>
      ) : (
        <ul>
          {unusedFunctions.map(n => {
            const fn = getFnData(n);
            if (!fn) return null;
            return (
              <li key={n.id}>
                {fn.name}
                <span className="module">line {n.metadata.line_number}</span>
              </li>
            );
          })}
        </ul>
      )}

      <h3>Graph Summary</h3>
      <p className="graph-summary">
        Nodes: <strong>{ir.nodes.length}</strong> |{" "}
        Edges: <strong>{ir.edges.length}</strong> |{" "}
        Calls: <strong>{callEdges.length}</strong> |{" "}
        Members: <strong>{memberEdges.length}</strong>
      </p>
    </section>
  );
}