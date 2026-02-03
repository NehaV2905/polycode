import { dummyIR } from "../data/dummyIR";

export default function FunctionTable() {
  return (
    <table>
      <thead>
        <tr>
          <th>Function</th>
          <th>Module</th>
          <th>Calls</th>
          <th>Returns</th>
        </tr>
      </thead>
      <tbody>
        {dummyIR.functions.map(fn => (
          <tr key={fn.name}>
            <td>{fn.name}</td>
            <td>{fn.module}</td>
            <td>{fn.calls.join(", ") || "—"}</td>
            <td>{fn.returns}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
