import CytoscapeComponent from "react-cytoscapejs";
import { dummyIR } from "../data/dummyIR";

type IRNode = typeof dummyIR.nodes[0];

const getLabel = (n: IRNode): string => {
  if ("Module"   in n.node_type) return n.node_type.Module?.file_path  ?? n.id;
  if ("Class"    in n.node_type) return n.node_type.Class?.name        ?? n.id;
  if ("Function" in n.node_type) return n.node_type.Function?.name     ?? n.id;
  return n.id;
};

const getKind = (n: IRNode): string => {
  if ("Module"   in n.node_type) return "Module";
  if ("Class"    in n.node_type) return "Class";
  if ("Function" in n.node_type) return "Function";
  return "Unknown";
};

export default function DependencyGraph() {
  const nodeElements = dummyIR.nodes.map(n => ({
    data: { id: n.id, label: getLabel(n), kind: getKind(n) }
  }));

  const edgeElements = dummyIR.edges.map((e, i) => ({
    data: {
      id: `edge-${i}`,
      source: e.from,
      target: e.to,
      kind: typeof e.edge_type === "object" && "Calls" in e.edge_type ? "Calls" : "HasMember"
    }
  }));

  const stylesheet = [
    {
      selector: "node",
      style: {
        label: "data(label)",
        color: "#D5B893",
        "font-size": "11px",
        "font-family": "Segoe UI, system-ui, sans-serif",
        "text-valign": "top",
        "text-halign": "center",
        "text-margin-y": -6,
        "background-color": "#617891",
        "border-width": 2,
        "border-color": "#D5B893",
        width: 18,
        height: 18,
      }
    },
    {
      selector: 'node[kind = "Module"]',
      style: {
        "background-color": "#6F4D38",
        "border-color": "#D5B893",
        width: 28, height: 28,
        "font-size": "13px",
        color: "#D5B893",
      }
    },
    {
      selector: 'node[kind = "Class"]',
      style: {
        "background-color": "#25344F",
        "border-color": "#617891",
        width: 22, height: 22,
        color: "#a8bfd4",
      }
    },
    {
      selector: 'edge[kind = "Calls"]',
      style: {
        "line-color": "#D5B893",
        "target-arrow-color": "#D5B893",
        "target-arrow-shape": "triangle",
        "curve-style": "bezier",
        width: 1.5,
        opacity: 0.8,
      }
    },
    {
      selector: 'edge[kind = "HasMember"]',
      style: {
        "line-color": "#617891",
        "line-style": "dashed",
        "target-arrow-shape": "none",
        "curve-style": "bezier",
        width: 1,
        opacity: 0.45,
      }
    }
  ];

  return (
    <div className="graph-container">
      <CytoscapeComponent
        elements={[...nodeElements, ...edgeElements]}
        layout={{ name: "breadthfirst", directed: true, spacingFactor: 1.4 } as any}
        stylesheet={stylesheet as any}
        style={{ width: "100%", height: "100%" }}
      />
    </div>
  );
}