import { useState } from "react";
import { dummyIR } from "../data/dummyIR";

type IRNode = typeof dummyIR.nodes[0];

const getModuleData = (n: IRNode) => "Module"   in n.node_type ? n.node_type.Module   : null;
const getClassData  = (n: IRNode) => "Class"    in n.node_type ? n.node_type.Class    : null;
const getFnData     = (n: IRNode) => "Function" in n.node_type ? n.node_type.Function : null;

function Section({ title, count, children }: { title: string; count: number; children: React.ReactNode }) {
  const [open, setOpen] = useState(true);
  return (
    <div className="collapsible-section">
      <button className="collapsible-header" onClick={() => setOpen(o => !o)}>
        <span className="collapsible-title">{title}</span>
        <span className="collapsible-meta">
          <span className="collapsible-count">{count}</span>
          <span className="collapsible-chevron">{open ? "▲" : "▼"}</span>
        </span>
      </button>
      {open && <div className="collapsible-body">{children}</div>}
    </div>
  );
}

export default function AnalysisOutput() {
  const modules   = dummyIR.nodes.filter(n => "Module"   in n.node_type);
  const classes   = dummyIR.nodes.filter(n => "Class"    in n.node_type);
  const functions = dummyIR.nodes.filter(n => "Function" in n.node_type);

  const callEdges   = dummyIR.edges.filter(e => typeof e.edge_type === "object" && e.edge_type !== null && "Calls" in (e.edge_type as object));
  const memberEdges = dummyIR.edges.filter(e => e.edge_type === "HasMember");

  const calledIds       = new Set(callEdges.map(e => e.to));
  const unusedFunctions = functions.filter(fn => !calledIds.has(fn.id));

  return (
    <section className="analysis-output">
      <h2>Code Summary</h2>

      <Section title="Modules" count={modules.length}>
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
      </Section>

      <Section title="Classes" count={classes.length}>
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
      </Section>

      <Section title="Functions" count={functions.length}>
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
      </Section>

      <Section title="Unused Functions" count={unusedFunctions.length}>
        {unusedFunctions.length === 0 ? (
          <p className="no-unused">None 🎉</p>
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
      </Section>

      <Section title="Graph Summary" count={dummyIR.nodes.length}>
        <p className="graph-summary">
          Nodes: <strong>{dummyIR.nodes.length}</strong> |{" "}
          Edges: <strong>{dummyIR.edges.length}</strong> |{" "}
          Calls: <strong>{callEdges.length}</strong> |{" "}
          Members: <strong>{memberEdges.length}</strong>
        </p>
      </Section>
    </section>
  );
}