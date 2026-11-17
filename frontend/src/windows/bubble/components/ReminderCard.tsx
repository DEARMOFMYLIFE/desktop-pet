import { Button } from '../../../components/ui/Button';
import { t } from '../../../lib/i18n';
import type { ReminderPayload } from '../../../types';
import { BubbleShell } from './BubbleShell';

interface ReminderCardProps {
  reminder: ReminderPayload;
  snoozeMin: number;
  onAcknowledge: () => void;
  onSnooze: () => void;
  onClose: () => void;
}

function localizedReminder(reminder: ReminderPayload): { title: string; body: string } {
  switch (reminder.kind) {
    case 'water':
      return { title: t('reminder.water.title'), body: t('reminder.water.body') };
    case 'sedentary':
      return { title: t('reminder.sedentary.title'), body: t('reminder.sedentary.body') };
    case 'eyes':
      return { title: t('reminder.eyes.title'), body: t('reminder.eyes.body') };
    case 'todo':
      return { title: t('reminder.todo.title'), body: reminder.body };
    default:
      return { title: reminder.title, body: reminder.body };
  }
}

export function ReminderCard({
  reminder,
  snoozeMin,
  onAcknowledge,
  onSnooze,
  onClose,
}: ReminderCardProps) {
  const text = localizedReminder(reminder);
  return (
    <BubbleShell label={text.title} onClose={onClose} className="reminder-card">
      <div className="reminder-body">{text.body}</div>
      <div className="bubble-actions">
        <Button variant="primary" onClick={onAcknowledge}>
          {reminder.kind === 'todo' ? t('bubble.todoDone') : t('bubble.ack')}
        </Button>
        {reminder.kind !== 'todo' && (
          <Button onClick={onSnooze}>{t('bubble.snooze', { n: snoozeMin })}</Button>
        )}
      </div>
    </BubbleShell>
  );
}
