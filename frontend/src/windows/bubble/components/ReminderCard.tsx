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

export function ReminderCard({
  reminder,
  snoozeMin,
  onAcknowledge,
  onSnooze,
  onClose,
}: ReminderCardProps) {
  return (
    <BubbleShell label={reminder.title} onClose={onClose} className="reminder-card">
      <div className="reminder-body">{reminder.body}</div>
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
