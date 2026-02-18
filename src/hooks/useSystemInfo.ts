import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { SystemInfo } from '../types';

export function useSystemInfo() {
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function loadSystemInfo() {
      try {
        setLoading(true);
        const result = await invoke<any>('get_system_info');
        setSystemInfo(result as SystemInfo);
      } catch (err) {
        console.error('Failed to load system info:', err);
      } finally {
        setLoading(false);
      }
    }

    loadSystemInfo();
  }, []);

  return { systemInfo, loading };
}
