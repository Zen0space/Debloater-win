import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { DebloatItem, Preset, HistoryEntry, ProgressItem } from '../types';

interface AppState {
  selectedItems: Set<string>;
  currentPreset: string | null;
  history: HistoryEntry[];
  currentProgress: ProgressItem[];
  selectedItemCount: number;
  
  toggleItem: (id: string) => void;
  selectPreset: (presetId: string) => void;
  applyPreset: (preset: Preset, items: DebloatItem[]) => void;
  addToHistory: (entry: HistoryEntry) => void;
  updateProgress: (items: ProgressItem[]) => void;
  clearProgress: () => void;
  clearSelection: () => void;
  selectAll: (items: DebloatItem[]) => void;
  deselectAll: () => void;
}

export const useAppStore = create<AppState>()(
  persist(
    (set) => ({
      selectedItems: new Set(),
      currentPreset: null,
      history: [],
      currentProgress: [],
      selectedItemCount: 0,
      
      toggleItem: (id) => set((state) => {
        const newSet = new Set(state.selectedItems);
        if (newSet.has(id)) {
          newSet.delete(id);
        } else {
          newSet.add(id);
        }
        return { selectedItems: newSet, selectedItemCount: newSet.size };
      }),
      
      selectPreset: (presetId) => set({ currentPreset: presetId }),
      
      applyPreset: (preset, items) => set(() => {
        const presetItems = items.filter(item => preset.items.includes(item.id));
        const newSet = new Set(presetItems.map(item => item.id));
        return { selectedItems: newSet, selectedItemCount: newSet.size, currentPreset: preset.id };
      }),
      
      addToHistory: (entry) => set((state) => ({
        history: [entry, ...state.history].slice(0, 100)
      })),
      
      updateProgress: (items) => set({ currentProgress: items }),
      
      clearProgress: () => set({ currentProgress: [] }),
      
      clearSelection: () => set({ selectedItems: new Set(), selectedItemCount: 0, currentPreset: null }),
      
      selectAll: (items) => set(() => {
        const newSet = new Set(items.map(item => item.id));
        return { selectedItems: newSet, selectedItemCount: newSet.size };
      }),
      
      deselectAll: () => set(() => ({ selectedItems: new Set(), selectedItemCount: 0 })),
    }),
    {
      name: 'debloater-storage',
      partialize: (state) => ({
        selectedItems: Array.from(state.selectedItems),
        currentPreset: state.currentPreset,
        history: state.history,
      }),
      merge: (persistedState: any, currentState) => ({
        ...currentState,
        selectedItems: new Set(persistedState.selectedItems || []),
        currentPreset: persistedState.currentPreset || null,
        history: persistedState.history || [],
      }),
    }
  )
);
