import { useSettings } from "./hooks/useSettings";
import { ServerStatus } from "./components/ServerStatus";
import { ApiKeySection } from "./components/ApiKeySection";
import { SettingsForm } from "./components/SettingsForm";
import { ShortcutGuide } from "./components/ShortcutGuide";
import "./App.css";

function App() {
  const { settings, serverStatus, loading, saveSettings, regenerateApiKey } =
    useSettings();

  if (loading || !settings || !serverStatus) {
    return (
      <main className="container">
        <p>Loading...</p>
      </main>
    );
  }

  const serverUrl = `http://${serverStatus.local_ip}:${serverStatus.port}`;

  return (
    <main className="container">
      <h1>Cross-Paste</h1>
      <p className="subtitle">iPhone - Windows Clipboard Sharing</p>

      <ServerStatus status={serverStatus} />
      <ApiKeySection apiKey={settings.api_key} onRegenerate={regenerateApiKey} />
      <SettingsForm settings={settings} onSave={saveSettings} />
      <ShortcutGuide serverUrl={serverUrl} apiKey={settings.api_key} />
    </main>
  );
}

export default App;
