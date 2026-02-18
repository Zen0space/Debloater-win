import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { DebloatItem } from '../types';

export function useDebloatItems(category: string) {
  const [items, setItems] = useState<DebloatItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function loadItems() {
      try {
        setLoading(true);
        
        let result: DebloatItem[];
        if (category === 'apps') {
          result = await invoke<DebloatItem[]>('load_apps_with_status');
        } else {
          result = await invoke<DebloatItem[]>('load_items', { category });
        }
        
        setItems(result);
        setError(null);
      } catch (err) {
        setError(err instanceof Error ? err.message : String(err));
      } finally {
        setLoading(false);
      }
    }

    loadItems();
  }, [category]);

  return { items, loading, error };
}
