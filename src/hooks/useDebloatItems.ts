import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { DebloatItem, BloatwareApp } from '../types';

export function useDebloatItems(category: string) {
  const [items, setItems] = useState<DebloatItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function loadItems() {
      try {
        setLoading(true);
        
        if (category === 'apps') {
          const bloatwareApps = await invoke<BloatwareApp[]>('get_all_bloatware_with_status');
          
          const debloatItems: DebloatItem[] = bloatwareApps.map(app => ({
            id: app.id,
            name: app.name,
            description: app.description,
            category: 'apps',
            safe: app.safe,
            command: `Get-AppxPackage *${app.package_pattern}* | Remove-AppxPackage`,
            rollbackCommand: undefined,
            isInstalled: app.is_installed,
            packagePattern: app.package_pattern,
          }));
          
          setItems(debloatItems);
        } else {
          const result = await invoke<DebloatItem[]>('load_items', { category });
          setItems(result);
        }
        
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
