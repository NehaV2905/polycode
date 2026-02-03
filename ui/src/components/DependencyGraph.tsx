import CytoscapeComponent from "react-cytoscapejs";
//import cytoscape from "cytoscape";
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

  return (
    <div style={{ height: "300px", border: "1px solid #ccc" }}>
      <CytoscapeComponent
        elements={elements}
        layout={{ name: "breadthfirst" }}
        style={{ width: "100%", height: "100%" }}
      />
    </div>
  );
}
