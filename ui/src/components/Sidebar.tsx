import { dummyIR } from "../data/dummyIR";

export default function Sidebar() {
  return (
    <aside>
      <h3>Modules</h3>
      <ul>
        {dummyIR.modules.map(m => (
          <li key={m.name}>
            {m.name} ({m.language})
          </li>
        ))}
      </ul>
    </aside>
  );
}
