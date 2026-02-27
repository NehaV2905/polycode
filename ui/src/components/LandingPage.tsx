export default function LandingPage({ onEnter }: { onEnter: () => void }) {
  return (
    <div className="landing-page">
      <h1 className="landing-title">PolyCode</h1>
      <div className="landing-divider" />
      <p className="landing-subtitle">IR Based Code Analysis Engine</p>
      <button className="landing-cta" onClick={onEnter}>
        Get Started
      </button>
    </div>
  );
}