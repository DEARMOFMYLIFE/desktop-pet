import { api } from '../lib/ipc';

export function useGeneralPanel() {
  return {
    openBubble: () => void api.openWindow('bubble'),
    openOnboarding: () => void api.openWindow('onboarding'),
  };
}
