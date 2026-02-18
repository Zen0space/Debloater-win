import { invoke } from '@tauri-apps/api/core';
import type { DebloatItem, ProgressItem } from '../types';

interface CommandResult {
  success: boolean;
  output: string;
  error?: string | null;
}

export async function executeItems(
  items: DebloatItem[],
  onProgress?: (progress: ProgressItem[]) => void
): Promise<{ success: boolean; errors: string[] }> {
  const errors: string[] = [];
  const progress: ProgressItem[] = [];

  for (const item of items) {
    progress.push({
      id: item.id,
      name: item.name,
      status: 'running',
    });

    if (onProgress) {
      onProgress([...progress]);
    }

    try {
      const result: CommandResult = await invoke('execute_command', {
        command: item.command,
        isRollback: false,
      });

      const lastProgress = progress[progress.length - 1];
      if (lastProgress) {
        if (result.success) {
          lastProgress.status = 'completed';
        } else {
          lastProgress.status = 'failed';
          lastProgress.error = result.error || 'Unknown error';
          errors.push(`${item.name}: ${result.error || 'Unknown error'}`);
        }
      }

      if (onProgress) {
        onProgress([...progress]);
      }
    } catch (error) {
      const lastProgress = progress[progress.length - 1];
      if (lastProgress) {
        lastProgress.status = 'failed';
        lastProgress.error = error instanceof Error ? error.message : String(error);
        errors.push(`${item.name}: ${lastProgress.error}`);
      }

      if (onProgress) {
        onProgress([...progress]);
      }
    }
  }

  return {
    success: errors.length === 0,
    errors,
  };
}

export async function rollbackItem(item: DebloatItem): Promise<{ success: boolean; error?: string }> {
  if (!item.rollbackCommand) {
    return {
      success: false,
      error: 'No rollback command available for this item',
    };
  }

  try {
    const result: CommandResult = await invoke('execute_command', {
      command: item.rollbackCommand,
      isRollback: true,
    });

    return {
      success: result.success,
      error: result.error || undefined,
    };
  } catch (error) {
    return {
      success: false,
      error: error instanceof Error ? error.message : String(error),
    };
  }
}
