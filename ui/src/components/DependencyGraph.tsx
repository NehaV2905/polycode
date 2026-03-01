import { useEffect, useRef } from "react";
import CytoscapeComponent from "react-cytoscapejs";
import cytoscape from "cytoscape";
// @ts-ignore — no bundled types for fcose
import fcose from "cytoscape-fcose";
import type { IRGraph, IRNode } from "../types";

// Register fcose once
if (!(cytoscape as any)._fcoseRegistered) {
  cytoscape.use(fcose);
  (cytoscape as any)._fcoseRegistered = true;
}

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
  const moduleNodes = ir.nodes.filter(isSourceModule).filter(n => "Module" in n.node_type);
  const otherNodes  = ir.nodes.filter(n => !("Module" in n.node_type));

  // file_path → module node id
  const fileToModuleId = new Map<string, string>();
  for (const m of moduleNodes) {
    const fp = (m.node_type as any).Module?.file_path ?? "";
    fileToModuleId.set(fp, m.id);
  }

  // Module nodes → compound containers
  const moduleElements = moduleNodes.map(n => ({
    data: { id: n.id, label: getLabel(n), kind: "Module" }
  }));

  // Class / Function nodes → children of their source-file container
  const childElements = otherNodes.map(n => {
    const parentId = fileToModuleId.get(n.metadata.file_path);
    return {
      data: {
        id: n.id,
        label: getLabel(n),
        kind: getKind(n),
        ...(parentId ? { parent: parentId } : {}),
      }
    };
  });

  const allNodeIds = new Set([
    ...moduleNodes.map(n => n.id),
    ...otherNodes.map(n => n.id),
  ]);

  const nodeById = new Map([...moduleNodes, ...otherNodes].map(n => [n.id, n]));

  const edgeElements = ir.edges
    .filter(e => allNodeIds.has(e.from) && allNodeIds.has(e.to))
    // Only Calls edges — HasMember is implicit inside the compound boxes
    .filter(e => typeof e.edge_type === "object" && e.edge_type !== null && "Calls" in (e.edge_type as object))
    .map((e, i) => {
      const toNode = nodeById.get(e.to);
      return {
        data: {
          id: `edge-${i}`,
          source: e.from,
          target: e.to,
          kind: "Calls",
          label: toNode ? getLabel(toNode) : "",
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
        "border-width": 1,
        "border-color": "#507090",
        width: 10,
        height: 10,
      }
    },

    // ── Module: dashed container box ─────────────────────────────────────────
    {
      selector: 'node[kind = "Module"]',
      style: {
        shape: "roundrectangle",
        label: "data(label)",
        "font-size": "11px",
        "font-family": "ui-monospace, 'Cascadia Code', monospace",
        "font-weight": "600",
        "text-valign": "top",
        "text-halign": "center",
        "text-margin-y": 6,
        color: "#D5B893",
        "text-background-color": "#0e1520",
        "text-background-opacity": 0.85,
        "text-background-shape": "roundrectangle",
        "text-background-padding": "3px",
        "background-color": "#0e1a26",
        "background-opacity": 0.5,
        "border-style": "dashed",
        "border-color": "#D5B893",
        "border-width": 1.5,
        "border-opacity": 0.8,
        padding: "20px",
      }
    },

    // ── Class nodes ──────────────────────────────────────────────────────────
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
        "background-color": "#7aaed4",
        "border-width": 0,
        width: "label",
        height: "label",
        padding: "6px",
      }
    },

    // ── Function dots ────────────────────────────────────────────────────────
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

    // ── Selected edge — show callee label ────────────────────────────────────
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

    // ── Hover / selected node ────────────────────────────────────────────────
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
    name: "fcose",
    animate: false,
    quality: "default",
    randomize: true,
    // Compound node spacing
    nodeSeparation: 75,
    // Edge lengths
    idealEdgeLength: (edge: any) => 120,
    edgeElasticity: (edge: any) => 0.45,
    // Repulsion
    nodeRepulsion: (node: any) => 6500,
    gravity: 0.25,
    gravityRange: 3.8,
    gravityCompound: 1.0,
    gravityRangeCompound: 1.5,
    // Iterations
    numIter: 2500,
    tile: true,
    tilingPaddingVertical: 10,
    tilingPaddingHorizontal: 10,
    fit: true,
    padding: 40,
  };

  const fnCount    = otherNodes.filter(n => "Function" in n.node_type).length;
  const classCount = otherNodes.filter(n => "Class"    in n.node_type).length;
  const callsCount = edgeElements.length;

  return (
    <div className="graph-container">
      <div className="graph-legend">
        <span className="legend-module">⬚ File ({moduleNodes.length})</span>
        <span className="legend-class">■ Class ({classCount})</span>
        <span className="legend-fn" style={{ color: "#a3536b" }}>● Fn ({fnCount})</span>
        <span className="legend-calls" style={{ color: "#c9a870" }}>→ Calls ({callsCount})</span>
        <span className="legend-hint">Scroll to zoom · drag to pan · click edge to see call target</span>
      </div>
      <CytoscapeComponent
        elements={[...moduleElements, ...childElements, ...edgeElements]}
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
