import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface Preset {
  name: string;
  detail: string;
  fps: string;
}

const PRESETS: Preset[] = [
  { name: "Competitive", detail: "Lowest settings, max frames", fps: "~180 fps" },
  { name: "Balanced", detail: "Best of both", fps: "~120 fps" },
  { name: "Quality", detail: "Highest visual fidelity", fps: "~90 fps" },
];

export default function App() {
  const [fov, setFov] = useState(90);
  const [selectedPreset, setSelectedPreset] = useState<string>("balanced");
  const [steamPath, setSteamPath] = useState("");
  const [status, setStatus] = useState<string>("");

  useEffect(() => {
    invoke<string>("get_video_config", { steamPath }).catch(console.error);
  }, [steamPath]);

  const applyPreset = async (preset: string) => {
    setSelectedPreset(preset);
    try {
      await invoke("set_fov", { steamPath, fov });
      setStatus(`Preset ${preset} applied`);
    } catch (e) {
      setStatus(`Error: ${e}`);
    }
  };

  return (
    <div className="app">
      <header>
        <h1>Deadlock FPS Fix</h1>
        <span>{status}</span>
      </header>
      <section className="presets">
        {PRESETS.map((p) => (
          <button
            key={p.name}
            onClick={() => applyPreset(p.name)}
            className={selectedPreset === p.name ? "active" : ""}
          >
            <strong>{p.name}</strong>
            <small>{p.detail}</small>
            <em>{p.fps}</em>
          </button>
        ))}
      </section>
      <section className="fov">
        <label>Field of view: {fov}°</label>
        <input
          type="range"
          min={70}
          max={130}
          value={fov}
          onChange={(e) => setFov(Number(e.target.value))}
        />
        {fov > 90 && <p className="warn">Above 90° is experimental</p>}
      </section>
    </div>
  );
}
