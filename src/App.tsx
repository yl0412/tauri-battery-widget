import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

type Device = {
  name: string;
  address: string;
  device_type: "bluetooth" | "2.4g";
  battery_level: number | null;
};

function App() {
  const [devices, setDevices] = useState<Device[]>([]);
  const [scanning, setScanning] = useState(false);

  const scan = async () => {
    setScanning(true);
    try {
      const result = await invoke<Device[]>("scan_devices");
      setDevices(result);
    } catch (e) {
      console.error(e);
    } finally {
      setScanning(false);
    }
  };

  useEffect(() => {
    invoke<Device[]>("get_added_devices").then(setDevices).catch(console.error);
  }, []);

  return (
    <div className="container">
      <h1>Device Battery Monitor</h1>
      <button onClick={scan} disabled={scanning}>
        {scanning ? "Scanning..." : "Scan Devices"}
      </button>
      <ul>
        {devices.map((d) => (
          <li key={d.address}>
            <strong>{d.name}</strong> ({d.device_type})
            <div>Battery: {d.battery_level !== null ? `${d.battery_level}%` : "N/A"}</div>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;
