import { useCallback } from 'react';
import { api } from '../lib/ipc';

// Window drag is an IPC action; keep the call out of presentation components.
export function useWindowDrag() {
  return useCallback(() => {
    void api.startWindowDrag();
  }, []);
}
