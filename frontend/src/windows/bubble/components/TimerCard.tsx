import { Button } from '../../../components/ui/Button';
import { formatClock } from '../../../lib/format';
import { t } from '../../../lib/i18n';
import type { Settings, TimerSnapshot } from '../../../types';
import { BubbleShell } from './BubbleShell';

interface TimerCardProps {
  timer: TimerSnapshot;
  settings: Settings | null;
  label: string;
  progress: number;
  primaryLabel: string;
  active: boolean;
  onPrimary: () => void;
  onSkip: () => void;
  onReset: () => void;
  onStartPreset: (minutes: number) => void;
  onStartCountUp: () => void;
  onClose: () => void;
}

export function TimerCard({
  timer,
  settings,
  label,
  progress,
  primaryLabel,
  active,
  onPrimary,
  onSkip,
  onReset,
  onStartPreset,
  onStartCountUp,
  onClose,
}: TimerCardProps) {
  return (
    <BubbleShell label={label} onClose={onClose}>
      <div className="bubble-time" aria-live="polite">
        {formatClock(timer.remainingSec)}
      </div>

      <div className="bubble-progress" aria-hidden="true">
        <span style={{ width: `${progress}%` }} />
      </div>

      <div className="bubble-actions">
        <Button variant="primary" onClick={onPrimary}>
          {primaryLabel}
        </Button>
        {active && <Button onClick={onSkip}>{t('bubble.skip')}</Button>}
        {active && (
          <Button variant="ghost" onClick={onReset}>
            {t('bubble.reset')}
          </Button>
        )}
      </div>

      {!active && settings && (
        <div className="bubble-presets">
          {settings.timerPresets.map((minutes) => (
            <button
              key={minutes}
              type="button"
              className="bubble-chip"
              onClick={() => onStartPreset(minutes)}
            >
              {minutes}
            </button>
          ))}
          <button type="button" className="bubble-chip" onClick={onStartCountUp}>
            {t('bubble.countUpAction')}
          </button>
        </div>
      )}

      {!active && (
        <div className="bubble-foot">
          <span>{t('bubble.presets')}</span>
          <span className="bubble-rounds">
            {timer.roundsCompleted}/{timer.roundsTotal}
          </span>
        </div>
      )}
    </BubbleShell>
  );
}
