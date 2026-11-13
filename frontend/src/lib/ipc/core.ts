import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { IpcError } from './errors';
import { mockCall } from './mock';

function isTauriRuntime(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

export async function call<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauriRuntime()) {
    return mockCall<T>(cmd, args);
  }
  try {
    return await invoke<T>(cmd, args);
  } catch (error) {
    const shaped = error as { code?: string; message?: string; details?: unknown };
    throw new IpcError(
      shaped.code ?? 'INTERNAL',
      shaped.message ?? 'Something went wrong',
      shaped.details,
    );
  }
}

export function on<T>(event: string, handler: (payload: T) => void): Promise<() => void> {
  if (!isTauriRuntime()) {
    return Promise.resolve(() => undefined);
  }
  return listen<T>(event, (payload) => handler(payload.payload));
}
