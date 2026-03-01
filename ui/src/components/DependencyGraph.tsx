import CytoscapeComponent from "react-cytoscapejs";
import cytoscape from "cytoscape";
// @ts-ignore — no bundled types for fcose
import fcose from "cytoscape-fcose";
import type { IRGraph, IRNode } from "../types";

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

  const nodeById = new Map([...moduleNodes, ...otherNodes].map(n => [n.id, n]));

  // Module nodes → compound containers
  const moduleElements = moduleNodes.map(n => ({
    data: { id: n.id, label: getLabel(n), kind: "Module" }
  }));

  // Class / Function nodes → nested inside their file's container
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

  const allNodeIds = new Set([...moduleNodes, ...otherNodes].map(n => n.id));

  // HasMember edges — pink dashed lines inside boxes
  const memberEdges = ir.edges
    .filter(e => allNodeIds.has(e.from) && allNodeIds.has(e.to))
    .filter(e => e.edge_type === "HasMember" || (typeof e.edge_type === "object" && e.edge_type !== null && "HasMember" in (e.edge_type as object)))
    .map((e, i) => ({
      data: { id: `member-${i}`, source: e.from, target: e.to, kind: "HasMember" }
    }));

  // Split Calls edges into:
  //   intra-file → drawn between function dots inside the same box
  //   inter-file → collapsed to module→module arrows (one per file pair, always visible)
  const intraEdges: any[] = [];
  const interEdgeMap = new Map<string, { fromMod: string; toMod: string; calls: string[] }>();

  ir.edges
    .filter(e => allNodeIds.has(e.from) && allNodeIds.has(e.to))
    .filter(e => typeof e.edge_type === "object" && e.edge_type !== null && "Calls" in (e.edge_type as object))
    .forEach((e, i) => {
      const fromNode = nodeById.get(e.from);
      const toNode   = nodeById.get(e.to);
      if (!fromNode || !toNode) return;

      const fromFile = fromNode.metadata.file_path;
      const toFile   = toNode.metadata.file_path;

      if (fromFile === toFile) {
        // Intra-file: connect individual function/class nodes
        intraEdges.push({
          data: { id: `intra-${i}`, source: e.from, target: e.to, kind: "IntraCall", label: getLabel(toNode) }
        });
      } else {
        // Inter-file: one arrow per ordered file pair, visible between boxes
        const fromMod = fileToModuleId.get(fromFile);
        const toMod   = fileToModuleId.get(toFile);
        if (!fromMod || !toMod || fromMod === toMod) return;
        const key = `${fromMod}→${toMod}`;
        if (!interEdgeMap.has(key)) {
          interEdgeMap.set(key, { fromMod, toMod, calls: [] });
        }
        const callee = getLabel(toNode);
        const entry  = interEdgeMap.get(key)!;
        if (!entry.calls.includes(callee)) entry.calls.push(callee);
      }
    });

  // One visible edge per file-pair with the callee names as label
  const interEdges = Array.from(interEdgeMap.values()).map(({ fromMod, toMod, calls }, i) => ({
    data: {
      id: `inter-${i}`,
      source: fromMod,
      target: toMod,
      kind: "InterCall",
      label: calls.slice(0, 4).join(", ") + (calls.length > 4 ? ` +${calls.length - 4}` : ""),
    }
  }));

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

    // ── HasMember: pink dashed lines inside a box ────────────────────────────
    {
      selector: 'edge[kind = "HasMember"]',
      style: {
        "line-color": "#a3536b",
        "line-style": "dashed",
        "line-dash-pattern": [6, 4],
        "target-arrow-shape": "none",
        "curve-style": "bezier",
        width: 1.8,
        opacity: 0.6,
      }
    },

    // ── Intra-file calls (inside a box) ──────────────────────────────────────
    {
      selector: 'edge[kind = "IntraCall"]',
      style: {
        "line-color": "#c9a870",
        "target-arrow-color": "#c9a870",
        "target-arrow-shape": "triangle",
        "arrow-scale": 1.1,
        "curve-style": "bezier",
        width: 1.2,
        opacity: 0.7,
      }
    },
    {
      selector: 'edge[kind = "IntraCall"]:selected',
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
        width: 2,
        opacity: 1,
      }
    },

    // ── Inter-file calls (box → box, always labelled) ─────────────────────────
    {
      selector: 'edge[kind = "InterCall"]',
      style: {
        label: "data(label)",
        "font-size": "9px",
        "font-family": "ui-monospace, 'Cascadia Code', monospace",
        color: "#e8c87a",
        "text-background-color": "#0e1520",
        "text-background-opacity": 0.88,
        "text-background-shape": "roundrectangle",
        "text-background-padding": "3px",
        "text-rotation": "autorotate",
        "line-color": "#e8c87a",
        "target-arrow-color": "#e8c87a",
        "target-arrow-shape": "triangle",
        "arrow-scale": 1.3,
        "curve-style": "bezier",
        width: 2,
        opacity: 0.9,
      }
    },
    {
      selector: 'edge[kind = "InterCall"]:selected',
      style: {
        width: 3,
        opacity: 1,
        "line-color": "#ffd97a",
        "target-arrow-color": "#ffd97a",
        color: "#ffd97a",
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
    nodeSeparation: 80,
    idealEdgeLength: (edge: any) => edge.data("kind") === "InterCall" ? 200 : 80,
    edgeElasticity: (edge: any) => edge.data("kind") === "InterCall" ? 0.6 : 0.35,
    nodeRepulsion: () => 8000,
    gravity: 0.25,
    gravityRange: 3.8,
    gravityCompound: 1.0,
    gravityRangeCompound: 1.5,
    numIter: 2500,
    tile: true,
    tilingPaddingVertical: 10,
    tilingPaddingHorizontal: 10,
    fit: true,
    padding: 40,
  };

  const fnCount    = otherNodes.filter(n => "Function" in n.node_type).length;
  const classCount = otherNodes.filter(n => "Class"    in n.node_type).length;

  return (
    <div className="graph-container">
      <div className="graph-legend">
        <span className="legend-module">⬚ File ({moduleNodes.length})</span>
        <span className="legend-class">■ Class ({classCount})</span>
        <span className="legend-fn" style={{ color: "#a3536b" }}>● Fn ({fnCount})</span>
        <span className="legend-member">- - Member ({memberEdges.length})</span>
        <span className="legend-calls" style={{ color: "#e8c87a" }}>→ Cross-file ({interEdges.length})</span>
        <span className="legend-hint">Scroll to zoom · drag to pan · click node/edge for details</span>
      </div>
      <CytoscapeComponent
        elements={[...moduleElements, ...childElements, ...memberEdges, ...intraEdges, ...interEdges]}
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
