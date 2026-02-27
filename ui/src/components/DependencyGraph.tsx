import CytoscapeComponent from "react-cytoscapejs";
import { dummyIR } from "../data/dummyIR";

export default function DependencyGraph() {
  const elements = [
    ...dummyIR.functions.map((fn) => ({
      data: { id: fn.name, label: fn.name }
    })),
    ...dummyIR.functions.flatMap((fn) =>
      fn.calls.map((call) => ({
        data: { source: fn.name, target: call }
      }))
    )
  ];

  const stylesheet = [
    {
      selector: "node",
      style: {
        label: "data(label)",
        color: "#D5B893",
        "font-size": "13px",
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
      selector: "edge",
      style: {
        "line-color": "#617891",
        "target-arrow-color": "#D5B893",
        "target-arrow-shape": "triangle",
        "curve-style": "bezier",
        width: 1.5,
        opacity: 0.7,
      }
    }
  ];

  return (
    <div className="graph-container">
      <CytoscapeComponent
        elements={elements}
        layout={{ name: "breadthfirst" }}
        stylesheet={stylesheet}
        style={{ width: "100%", height: "100%" }}
      />
    </div>
  );
}