import { useCallback, useState } from 'react';
import { api } from '../lib/ipc';

export function useAboutPanel() {
  const [pendingClear, setPendingClear] = useState(false);

  const openDataFolder = () => {
    void api.openDataFolder().catch(console.error);
  };

  const requestClearData = useCallback(() => {
    setPendingClear(true);
  }, []);

  const cancelClearData = useCallback(() => {
    setPendingClear(false);
  }, []);

  const confirmClearData = async () => {
    setPendingClear(false);
    await api.clearData().catch(console.error);
  };

  const previewOnboarding = () => {
    void api.openWindow('onboarding');
  };

  const quit = () => {
    void api.quit();
  };

  return {
    pendingClear,
    openDataFolder,
    requestClearData,
    cancelClearData,
    confirmClearData,
    previewOnboarding,
    quit,
  };
}
