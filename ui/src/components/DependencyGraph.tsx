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

  return (
    <div className="graph-container">
      <CytoscapeComponent
        elements={elements}
        layout={{ name: "breadthfirst" }}
        style={{ width: "100%", height: "100%" }}
      />
    </div>
  );
}