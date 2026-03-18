import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect, useCallback } from "react";
import type { Settings, ServerStatus } from "../types";

export function useSettings() {
  const [settings, setSettings] = useState<Settings | null>(null);
  const [serverStatus, setServerStatus] = useState<ServerStatus | null>(null);
  const [loading, setLoading] = useState(true);

  const loadAll = useCallback(async () => {
    try {
      const [s, st] = await Promise.all([
        invoke<Settings>("get_settings"),
        invoke<ServerStatus>("get_server_status"),
      ]);
      setSettings(s);
      setServerStatus(st);
    } catch (e) {
      console.error("Failed to load settings:", e);
    } finally {
      setLoading(false);
    }
  }, []);

  const saveSettings = useCallback(
    async (newSettings: Settings) => {
      await invoke("save_settings", { newSettings });
      await invoke("restart_server");
      await loadAll();
    },
    [loadAll]
  );

  const regenerateApiKey = useCallback(async () => {
    const newKey = await invoke<string>("regenerate_api_key");
    await loadAll();
    return newKey;
  }, [loadAll]);

  useEffect(() => {
    loadAll();
  }, [loadAll]);

  return { settings, serverStatus, loading, saveSettings, regenerateApiKey, reload: loadAll };
}
