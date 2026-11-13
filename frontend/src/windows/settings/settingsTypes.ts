import type { PetSize, SettingsTab } from '../../types';

export type { SettingsPanelProps, SettingsTab, TabId } from '../../types';

export const TABS: SettingsTab[] = [
  { id: 'general', labelKey: 'settings.tabs.general' },
  { id: 'timer', labelKey: 'settings.tabs.timer' },
  { id: 'reminders', labelKey: 'settings.tabs.reminders' },
  { id: 'todos', labelKey: 'settings.tabs.todos' },
  { id: 'stats', labelKey: 'settings.tabs.stats' },
  { id: 'appearance', labelKey: 'settings.tabs.appearance' },
  { id: 'shortcuts', labelKey: 'settings.tabs.shortcuts' },
  { id: 'about', labelKey: 'settings.tabs.about' },
];

export const SIZES: Array<{ value: PetSize; labelKey: string }> = [
  { value: 'S', labelKey: 'settings.general.sizeS' },
  { value: 'M', labelKey: 'settings.general.sizeM' },
  { value: 'L', labelKey: 'settings.general.sizeL' },
];
