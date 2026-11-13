export interface ReminderRow {
  kind: string;
  enabled: boolean;
  intervalSec: number;
  nextDueAt: string | null;
  lastFiredAt: string | null;
  snoozedUntil: string | null;
}

export interface ReminderPayload {
  kind: string;
  title: string;
  body: string;
  todoId: number | null;
}
