import { useEffect } from "react";

interface LandingPageProps {
  onFinish: () => void;
}

export default function LandingPage({ onFinish }: LandingPageProps) {
  useEffect(() => {
    const timer = setTimeout(() => {
      onFinish();
    }, 2500); // 2.5 seconds

    return () => clearTimeout(timer);
  }, [onFinish]);

  return (
    <div className="landing-page">
      <h1 className="landing-title">
        PolyCode
      </h1>
      <p className="landing-subtitle">
        IR-Based Multilingual Code Analysis
      </p>
    </div>
  );
}