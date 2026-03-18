import { useState, useEffect } from "react";
import type { Settings } from "../types";

interface Props {
  settings: Settings;
  onSave: (settings: Settings) => Promise<void>;
}

export function SettingsForm({ settings, onSave }: Props) {
  const [port, setPort] = useState(settings.port);
  const [autoStart, setAutoStart] = useState(settings.auto_start);
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    setPort(settings.port);
    setAutoStart(settings.auto_start);
  }, [settings]);

  const handleSave = async () => {
    setSaving(true);
    setSaved(false);
    try {
      await onSave({
        ...settings,
        port,
        auto_start: autoStart,
      });
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } finally {
      setSaving(false);
    }
  };

  const hasChanges = port !== settings.port || autoStart !== settings.auto_start;

  return (
    <div className="card">
      <div className="card-header">
        <h2>Settings</h2>
      </div>
      <div className="card-body">
        <div className="form-group">
          <label htmlFor="port">Port</label>
          <input
            id="port"
            type="number"
            min={1024}
            max={65535}
            value={port}
            onChange={(e) => setPort(Number(e.target.value))}
          />
        </div>
        <div className="form-group">
          <label className="checkbox-label">
            <input
              type="checkbox"
              checked={autoStart}
              onChange={(e) => setAutoStart(e.target.checked)}
            />
            Auto-start with Windows
          </label>
        </div>
        <button
          className="btn-primary"
          onClick={handleSave}
          disabled={saving || !hasChanges}
        >
          {saving ? "Saving..." : saved ? "Saved!" : "Save & Restart Server"}
        </button>
      </div>
    </div>
  );
}
