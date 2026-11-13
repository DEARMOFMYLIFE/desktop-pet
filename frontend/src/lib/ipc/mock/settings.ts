import type { AppInfo, Settings } from '../../../types';
import { DEFAULT_SETTINGS } from '../defaults';

const MOCK_KEY = 'yuzu.settings';

export const MOCK_APP: AppInfo = {
  name: '小柚',
  version: '0.1.0',
  platform: 'browser',
};

export function readMockSettings(): Settings {
  if (typeof localStorage === 'undefined') return { ...DEFAULT_SETTINGS };
  try {
    const raw = localStorage.getItem(MOCK_KEY);
    if (!raw) return { ...DEFAULT_SETTINGS };
    return { ...DEFAULT_SETTINGS, ...(JSON.parse(raw) as Partial<Settings>) };
  } catch {
    return { ...DEFAULT_SETTINGS };
  }
}

export function writeMockSettings(settings: Settings): Settings {
  if (typeof localStorage !== 'undefined') {
    localStorage.setItem(MOCK_KEY, JSON.stringify(settings));
  }
  return settings;
}

export function openBrowserWindow(label: string): void {
  if (typeof window === 'undefined') return;
  const width = label === 'settings' ? 760 : 520;
  const height = label === 'settings' ? 560 : 420;
  window.open(`/${label}.html`, '_blank', `width=${width},height=${height}`);
}

export function removeMockSettings(): void {
  if (typeof localStorage !== 'undefined') localStorage.removeItem(MOCK_KEY);
}
