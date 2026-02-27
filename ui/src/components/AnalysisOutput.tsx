import { dummyIR } from "../data/dummyIR";

export default function AnalysisOutput() {
  const allCalls = dummyIR.functions.flatMap((f) => f.calls);

  const unusedFunctions = dummyIR.functions.filter(
    (fn) => !allCalls.includes(fn.name)
  );

  const totalNodes = dummyIR.functions.length;
  const totalEdges = allCalls.length;

  return (
    <section className="analysis-output">
      <h2>Code Summary</h2>
      <h3>Functions</h3>
      <ul>
        {dummyIR.functions.map((fn) => (
          <li key={fn.name}>
            {fn.name} <span className="module">({fn.module})</span>
          </li>
        ))}
      </ul>

      <h3>Unused Functions</h3>
      {unusedFunctions.length === 0 ? (
        <p className="no-unused">None 🎉</p>
      ) : (
        <ul>
          {unusedFunctions.map((fn) => (
            <li key={fn.name}>
              {fn.name} <span className="module">({fn.module})</span>
            </li>
          ))}
        </ul>
      )}
      <h3>Graph Summary</h3>
      <p className="graph-summary">
        Nodes: <strong>{totalNodes}</strong> | Edges:{" "}
        <strong>{totalEdges}</strong>
      </p>
    </section>
  );
}