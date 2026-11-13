import type { ReminderSettings, Settings } from '../types';

export function useReminderPanel(
  settings: Settings,
  update: (patch: Partial<Settings>) => Promise<void>,
) {
  const setReminder = (patch: Partial<ReminderSettings>) => {
    void update({ reminders: { ...settings.reminders, ...patch } });
  };

  const setInterval = (key: keyof ReminderSettings, value: number) => {
    setReminder({
      [key]: Math.min(180, Math.max(15, Math.round(value))),
    } as Partial<ReminderSettings>);
  };

  return { setReminder, setInterval };
}
