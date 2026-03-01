import CytoscapeComponent from "react-cytoscapejs";
import type { IRGraph, IRNode } from "../types";

const SOURCE_EXTS = /\.(py|java|go|rs|rb|c|h|ts|js|cpp|cs)$/i;

const isSourceModule = (n: IRNode): boolean => {
  const nt = n.node_type as any;
  if (!("Module" in nt)) return true;
  return SOURCE_EXTS.test(nt.Module?.file_path ?? "");
};

const basename = (p: string) => p.split(/[/\\]/).pop() ?? p;

const getLabel = (n: IRNode): string => {
  const nt = n.node_type as any;
  if ("Module"   in nt) return basename(nt.Module?.file_path ?? n.id);
  if ("Class"    in nt) return nt.Class?.name    ?? n.id;
  if ("Function" in nt) return nt.Function?.name ?? n.id;
  return n.id;
};

const getKind = (n: IRNode): string => {
  if ("Module"   in n.node_type) return "Module";
  if ("Class"    in n.node_type) return "Class";
  if ("Function" in n.node_type) return "Function";
  return "Unknown";
};

export default function DependencyGraph({ ir }: { ir: IRGraph }) {
  const visibleNodes = ir.nodes.filter(isSourceModule);

  const nodeElements = visibleNodes.map(n => ({
    data: { id: n.id, label: getLabel(n), kind: getKind(n) }
  }));

  const allNodeIds = new Set(visibleNodes.map(n => n.id));
  const nodeById   = new Map(visibleNodes.map(n => [n.id, n]));

  const edgeElements = ir.edges
    .filter(e => allNodeIds.has(e.from) && allNodeIds.has(e.to))
    .map((e, i) => {
      const isCalls = typeof e.edge_type === "object" && e.edge_type !== null && "Calls" in (e.edge_type as object);
      const toNode  = nodeById.get(e.to);
      const label   = isCalls && toNode ? getLabel(toNode) : "";
      return {
        data: {
          id: `edge-${i}`,
          source: e.from,
          target: e.to,
          kind: isCalls ? "Calls" : "HasMember",
          label,
        }
      };
    });

  const stylesheet = [
    // ── Base ────────────────────────────────────────────────────────────────
    {
      selector: "node",
      style: {
        label: "",
        shape: "ellipse",
        "background-color": "#3d5166",
        "border-width": 1.5,
        "border-color": "#507090",
        width: 10,
        height: 10,
      }
    },

    // ── Module ───────────────────────────────────────────────────────────────
    {
      selector: 'node[kind = "Module"]',
      style: {
        shape: "ellipse",
        label: "data(label)",
        "font-size": "11px",
        "font-family": "ui-monospace, 'Cascadia Code', monospace",
        "font-weight": "600",
        "text-valign": "top",
        "text-halign": "center",
        "text-margin-y": -8,
        color: "#D5B893",
        "text-background-color": "#0e1520",
        "text-background-opacity": 0.88,
        "text-background-shape": "roundrectangle",
        "text-background-padding": "3px",
        "background-color": "#3d4f30",
        "border-color": "#D5B893",
        "border-width": 2,
        width: 22,
        height: 22,
      }
    },

    // ── Class ────────────────────────────────────────────────────────────────
    {
      selector: 'node[kind = "Class"]',
      style: {
        shape: "roundrectangle",
        label: "data(label)",
        "font-size": "10px",
        "font-family": "ui-monospace, 'Cascadia Code', monospace",
        "text-valign": "center",
        "text-halign": "center",
        color: "#0e1520",
        "text-background-color": "#7aaed4",
        "text-background-opacity": 1,
        "text-background-shape": "roundrectangle",
        "text-background-padding": "4px",
        "background-color": "#7aaed4",
        "border-width": 0,
        width: "label",
        height: "label",
        padding: "6px",
      }
    },

    // ── Function ─────────────────────────────────────────────────────────────
    {
      selector: 'node[kind = "Function"]',
      style: {
        shape: "ellipse",
        label: "",
        "background-color": "#a3536b",
        "border-width": 1,
        "border-color": "#c46a82",
        width: 9,
        height: 9,
      }
    },

    // ── Calls edges ──────────────────────────────────────────────────────────
    {
      selector: 'edge[kind = "Calls"]',
      style: {
        "line-color": "#c9a870",
        "target-arrow-color": "#c9a870",
        "target-arrow-shape": "triangle",
        "arrow-scale": 1.2,
        "curve-style": "bezier",
        width: 1.5,
        opacity: 0.8,
      }
    },

    // ── HasMember edges ──────────────────────────────────────────────────────
    {
      selector: 'edge[kind = "HasMember"]',
      style: {
        "line-color": "#a3536b",
        "line-style": "dashed",
        "line-dash-pattern": [6, 4],
        "target-arrow-shape": "none",
        "curve-style": "bezier",
        width: 2,
        opacity: 0.65,
      }
    },

    // ── Calls edge selected — show callee label ───────────────────────────────
    {
      selector: 'edge[kind = "Calls"]:selected',
      style: {
        label: "data(label)",
        "font-size": "10px",
        "font-family": "ui-monospace, 'Cascadia Code', monospace",
        color: "#c9a870",
        "text-background-color": "#0e1520",
        "text-background-opacity": 0.9,
        "text-background-shape": "roundrectangle",
        "text-background-padding": "3px",
        "text-rotation": "autorotate",
        width: 2.5,
        opacity: 1,
      }
    },

    // ── Hover / selected ─────────────────────────────────────────────────────
    {
      selector: "node:active",
      style: { "overlay-opacity": 0.08 }
    },
    {
      selector: "node:selected",
      style: {
        "border-color": "#ffffff",
        "border-width": 2.5,
        "border-style": "solid",
        label: "data(label)",
        color: "#ffffff",
        "font-size": "11px",
        "text-background-color": "#1e3050",
        "text-background-opacity": 0.95,
        "text-background-shape": "roundrectangle",
        "text-background-padding": "4px",
        "text-valign": "bottom",
        "text-margin-y": 6,
      }
    },
  ];

  const layout = {
    name: "cose",
    animate: false,
    randomize: true,
    nodeRepulsion:   () => 22000,
    idealEdgeLength: () => 120,
    edgeElasticity:  () => 45,
    gravity: 0.25,
    numIter: 1500,
    initialTemp: 200,
    coolingFactor: 0.97,
    minTemp: 1.0,
    fit: true,
    padding: 40,
    nodeOverlap: 50,
  };

  const moduleCount   = visibleNodes.filter(n => "Module"   in n.node_type).length;
  const classCount    = visibleNodes.filter(n => "Class"    in n.node_type).length;
  const fnCount       = visibleNodes.filter(n => "Function" in n.node_type).length;
  const memberCount   = edgeElements.filter(e => e.data.kind === "HasMember").length;
  const callsCount    = edgeElements.filter(e => e.data.kind === "Calls").length;

  return (
    <div className="graph-container">
      <div className="graph-legend">
        <span className="legend-module">● File ({moduleCount})</span>
        <span className="legend-class">■ Class ({classCount})</span>
        <span className="legend-fn" style={{ color: "#a3536b" }}>● Fn ({fnCount})</span>
        <span className="legend-member">- - Member ({memberCount})</span>
        <span className="legend-calls" style={{ color: "#c9a870" }}>→ Calls ({callsCount})</span>
        <span className="legend-hint">Scroll to zoom · drag to pan · click edge to see call target</span>
      </div>
      <CytoscapeComponent
        elements={[...nodeElements, ...edgeElements]}
        layout={layout as any}
        stylesheet={stylesheet as any}
        style={{ width: "100%", height: "calc(100% - 28px)" }}
        wheelSensitivity={0.25}
        minZoom={0.1}
        maxZoom={4}
      />
    </div>
  );
}
