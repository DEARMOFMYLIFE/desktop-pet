import type { AppInfo } from './app';
import type { Settings } from './settings';

export type TabId =
  | 'general'
  | 'timer'
  | 'reminders'
  | 'todos'
  | 'stats'
  | 'appearance'
  | 'shortcuts'
  | 'about';

export interface SettingsPanelProps {
  settings: Settings;
  appInfo: AppInfo | null;
  update: (patch: Partial<Settings>) => Promise<void>;
  notify: (message: string) => void;
}

export interface SettingsTab {
  id: TabId;
  labelKey: string;
}
