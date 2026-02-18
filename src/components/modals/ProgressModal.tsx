import { CheckCircle, XCircle, Loader2 } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '../ui/card';
import { cn } from '../../lib/utils';
import type { ProgressItem } from '../../types';

interface ProgressModalProps {
  isOpen: boolean;
  progress: ProgressItem[];
  onClose: () => void;
}

export function ProgressModal({ isOpen, progress, onClose }: ProgressModalProps) {
  if (!isOpen) return null;

  const completed = progress.filter(p => p.status === 'completed').length;
  const failed = progress.filter(p => p.status === 'failed').length;
  const total = progress.length;

  return (
    <div className="fixed inset-0 bg-background/80 backdrop-blur-sm flex items-center justify-center z-50">
      <Card className="w-full max-w-2xl max-h-[80vh] overflow-hidden">
        <CardHeader>
          <CardTitle className="flex items-center justify-between">
            <span>Applying Changes</span>
            <span className="text-sm font-normal text-muted-foreground">
              {completed} / {total} completed
            </span>
          </CardTitle>
        </CardHeader>
        <CardContent className="overflow-y-auto max-h-[60vh]">
          {total === 0 ? (
            <div className="flex items-center justify-center py-8">
              <Loader2 className="animate-spin text-primary" size={24} />
              <span className="ml-2">Starting...</span>
            </div>
          ) : (
            <div className="space-y-3">
              {progress.map((item) => (
                <div
                  key={item.id}
                  className={cn(
                    'flex items-start gap-3 p-3 rounded-lg',
                    item.status === 'failed' && 'bg-destructive/10',
                    item.status === 'completed' && 'bg-primary/10'
                  )}
                >
                  {item.status === 'pending' && (
                    <Loader2 className="animate-spin text-muted-foreground mt-1" size={18} />
                  )}
                  {item.status === 'running' && (
                    <Loader2 className="animate-spin text-primary mt-1" size={18} />
                  )}
                  {item.status === 'completed' && (
                    <CheckCircle className="text-primary mt-1" size={18} />
                  )}
                  {item.status === 'failed' && (
                    <XCircle className="text-destructive mt-1" size={18} />
                  )}

                  <div className="flex-1 min-w-0">
                    <p className={cn(
                      'font-medium',
                      item.status === 'failed' && 'text-destructive'
                    )}>
                      {item.name}
                    </p>
                    {item.status === 'running' && (
                      <p className="text-sm text-muted-foreground">Running...</p>
                    )}
                    {item.status === 'completed' && (
                      <p className="text-sm text-muted-foreground">Completed successfully</p>
                    )}
                    {item.status === 'failed' && (
                      <p className="text-sm text-destructive break-words whitespace-pre-wrap">{item.error || 'Failed to execute'}</p>
                    )}
                  </div>
                </div>
              ))}
            </div>
          )}

          {completed + failed === total && total > 0 && (
            <div className="mt-6 pt-6 border-t border-border">
              <button
                onClick={onClose}
                className="w-full py-2 bg-primary text-primary-foreground rounded-lg hover:opacity-90 transition-opacity"
              >
                Close
              </button>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
