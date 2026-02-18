import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { Preset } from '../types';

export function usePresets() {
  const [presets, setPresets] = useState<Preset[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function loadPresets() {
      try {
        setLoading(true);
        const result = await invoke<Preset[]>('load_presets');
        setPresets(result);
        setError(null);
      } catch (err) {
        setError(err instanceof Error ? err.message : String(err));
      } finally {
        setLoading(false);
      }
    }

    loadPresets();
  }, []);

  return { presets, loading, error };
}
