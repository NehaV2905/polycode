import AppShell from "./components/AppShell";
import "./index.css";
import { useState } from "react";
import LandingPage from "./components/LandingPage";

export default function App() {
  const [showLanding, setShowLanding] = useState(true);

  return showLanding ? (
    <LandingPage onFinish={() => setShowLanding(false)} />
  ) : (
    <AppShell />
  );
}