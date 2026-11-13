import { ErrorPane } from '../../components/ui/ErrorPane';
import { LoadingPane } from '../../components/ui/LoadingPane';
import { useBubbleWindow } from '../../hooks/useBubbleWindow';
import { ReminderCard } from './components/ReminderCard';
import { TimerCard } from './components/TimerCard';
import './bubble.css';

export function BubbleWindow() {
  const bubble = useBubbleWindow();

  if (bubble.loadError) {
    return (
      <div className="bubble-root">
        <ErrorPane transparent message={bubble.loadError} />
      </div>
    );
  }

  if (bubble.reminder) {
    return (
      <ReminderCard
        reminder={bubble.reminder}
        snoozeMin={bubble.settings?.reminders.snoozeMin ?? 5}
        onAcknowledge={() => void bubble.acknowledgeReminder()}
        onSnooze={() => void bubble.snoozeReminder()}
        onClose={bubble.closeReminder}
      />
    );
  }

  if (!bubble.timer) {
    return (
      <div className="bubble-root">
        <LoadingPane transparent />
      </div>
    );
  }

  return (
    <TimerCard
      timer={bubble.timer}
      settings={bubble.settings}
      label={bubble.timerLabel}
      progress={bubble.progress}
      primaryLabel={bubble.primaryLabel}
      active={bubble.active}
      onPrimary={bubble.handlePrimary}
      onSkip={bubble.skip}
      onReset={bubble.reset}
      onStartPreset={bubble.startPreset}
      onStartCountUp={bubble.startCountUp}
      onClose={bubble.closeBubble}
    />
  );
}
