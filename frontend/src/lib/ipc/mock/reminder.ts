import type { ReminderRow } from '../../../types';

const MOCK_REMINDERS: ReminderRow[] = [
  {
    kind: 'water',
    enabled: true,
    intervalSec: 2700,
    nextDueAt: null,
    lastFiredAt: null,
    snoozedUntil: null,
  },
  {
    kind: 'sedentary',
    enabled: true,
    intervalSec: 3600,
    nextDueAt: null,
    lastFiredAt: null,
    snoozedUntil: null,
  },
  {
    kind: 'eyes',
    enabled: true,
    intervalSec: 1200,
    nextDueAt: null,
    lastFiredAt: null,
    snoozedUntil: null,
  },
];

export function handleReminder<T>(
  cmd: string,
  _args?: Record<string, unknown>,
): Promise<T> | null {
  switch (cmd) {
    case 'reminder_list':
      return Promise.resolve(MOCK_REMINDERS as T);
    case 'reminder_ack':
    case 'reminder_snooze':
      return Promise.resolve(undefined as T);
    default:
      return null;
  }
}
