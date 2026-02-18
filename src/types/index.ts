export interface DebloatItem {
  id: string;
  name: string;
  description: string;
  category: Category;
  safe: boolean;
  command: string;
  rollbackCommand?: string;
  enabled?: boolean;
  isInstalled?: boolean;
}

export type Category = 
  | 'apps'
  | 'privacy'
  | 'services'
  | 'registry'
  | 'updates'
  | 'system';

export interface Preset {
  id: string;
  name: string;
  description: string;
  items: string[];
}

export interface HistoryEntry {
  id: string;
  timestamp: number;
  items: string[];
  type: 'apply' | 'rollback';
}

export interface SystemInfo {
  windowsVersion: string;
  buildNumber: string;
  username: string;
}

export interface ProgressItem {
  id: string;
  name: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  error?: string;
}
