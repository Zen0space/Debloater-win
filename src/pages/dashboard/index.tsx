import { useState } from 'react';
import { Shield, Settings, Play } from 'lucide-react';
import { useAppStore } from '../../store/useAppStore';
import { useSystemInfo } from '../../hooks/useSystemInfo';
import { usePresets } from '../../hooks/usePresets';
import { useDebloatItems } from '../../hooks/useDebloatItems';
import { executeItems } from '../../hooks/useExecution';
import { Card, CardContent, CardHeader, CardTitle } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import { ProgressModal } from '../../components/modals/ProgressModal';
import type { ProgressItem } from '../../types';

export default function Dashboard() {
  const { selectedItems, selectedItemCount, applyPreset, addToHistory, clearProgress, updateProgress } = useAppStore();
  const { systemInfo } = useSystemInfo();
  const { presets } = usePresets();
  const { items: allItems } = useDebloatItems('apps');
  
  const [isProgressModalOpen, setIsProgressModalOpen] = useState(false);
  const [currentProgress, setCurrentProgress] = useState<ProgressItem[]>([]);
  const [isExecuting, setIsExecuting] = useState(false);

  const handlePresetClick = (presetId: string) => {
    const preset = presets.find(p => p.id === presetId);
    if (preset) {
      applyPreset(preset, allItems);
    }
  };

  const handleApplySelected = async () => {
    const selectedItemsList = allItems.filter(item => selectedItems.has(item.id));
    
    if (selectedItemsList.length === 0) {
      return;
    }

    setIsExecuting(true);
    setIsProgressModalOpen(true);
    setCurrentProgress([]);
    updateProgress([]);

    await executeItems(selectedItemsList, (progress) => {
      setCurrentProgress(progress);
      updateProgress(progress);
    });

    addToHistory({
      id: Date.now().toString(),
      timestamp: Date.now(),
      items: selectedItemsList.map(item => item.id),
      type: 'apply',
    });

    setIsExecuting(false);
    clearProgress();
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold mb-2">Dashboard</h1>
        <p className="text-muted-foreground">Overview of your debloating status</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <Card>
          <CardHeader>
            <CardTitle>Selected Items</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-4xl font-bold text-primary">{selectedItemCount}</p>
            <p className="text-sm text-muted-foreground mt-1">items to apply</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Available Presets</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-4xl font-bold">{presets.length}</p>
            <p className="text-sm text-muted-foreground mt-1">presets configured</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>System</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-2xl font-bold">
              {systemInfo?.buildNumber || 'Unknown'}
            </p>
            <p className="text-sm text-muted-foreground mt-1">Windows Build</p>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Quick Actions</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {presets.map((preset) => (
              <Button
                key={preset.id}
                variant="outline"
                className="h-auto p-4 justify-start hover:bg-accent"
                onClick={() => handlePresetClick(preset.id)}
              >
                <div className="flex items-center gap-3 w-full">
                  {preset.id === 'minimal' && <Shield size={24} className="text-primary flex-shrink-0" />}
                  {preset.id === 'balanced' && <Settings size={24} className="text-primary flex-shrink-0" />}
                  {preset.id === 'full' && <Shield size={24} className="text-primary flex-shrink-0" />}
                  <div className="text-left flex-1">
                    <p className="font-medium">{preset.name}</p>
                    <p className="text-sm text-muted-foreground">{preset.description}</p>
                  </div>
                </div>
              </Button>
            ))}

            <Button
              variant="outline"
              className="h-auto p-4 justify-start hover:bg-accent"
              disabled={selectedItemCount === 0 || isExecuting}
              onClick={handleApplySelected}
            >
              <div className="flex items-center gap-3 w-full">
                <Play size={24} className="text-primary flex-shrink-0" />
                <div className="text-left flex-1">
                  <p className="font-medium">Apply Selected</p>
                  <p className="text-sm text-muted-foreground">
                    {selectedItemCount > 0 
                      ? `Apply ${selectedItemCount} selected items` 
                      : 'Select items to apply'}
                  </p>
                </div>
              </div>
            </Button>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>System Information</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-2">
            <div className="flex justify-between py-2 border-b border-border">
              <span className="text-muted-foreground">Operating System</span>
              <span className="font-medium">
                {systemInfo?.windowsVersion || 'Checking...'}
              </span>
            </div>
            <div className="flex justify-between py-2 border-b border-border">
              <span className="text-muted-foreground">Build</span>
              <span className="font-medium">
                {systemInfo?.buildNumber || 'Checking...'}
              </span>
            </div>
            <div className="flex justify-between py-2">
              <span className="text-muted-foreground">User</span>
              <span className="font-medium">
                {systemInfo?.username || 'Checking...'}
              </span>
            </div>
          </div>
        </CardContent>
      </Card>

      <ProgressModal
        isOpen={isProgressModalOpen}
        progress={currentProgress}
        onClose={() => setIsProgressModalOpen(false)}
      />
    </div>
  );
}
