import { api } from '../lib/ipc';
import type { ReminderPayload, Settings } from '../types';

export function useBubbleReminder(
  reminder: ReminderPayload | null,
  setReminder: (next: ReminderPayload | null) => void,
  settings: Settings | null,
) {
  const acknowledgeReminder = async () => {
    if (!reminder) return;
    if (reminder.kind === 'todo' && reminder.todoId !== null) {
      await api.todoToggle(reminder.todoId, true).catch(console.error);
    } else {
      await api.reminderAck(reminder.kind).catch(console.error);
    }
    setReminder(null);
  };

  const snoozeReminder = async () => {
    if (!reminder) return;
    if (reminder.kind === 'todo') {
      setReminder(null);
      return;
    }
    await api
      .reminderSnooze(reminder.kind, settings?.reminders.snoozeMin ?? 5)
      .catch(console.error);
    setReminder(null);
  };

  return {
    acknowledgeReminder,
    snoozeReminder,
    closeReminder: () => setReminder(null),
  };
}
