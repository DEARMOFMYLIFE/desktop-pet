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
  selectedMinutes: number;
  onSelectPreset: (minutes: number) => void;
  onStartCountUp: () => void;
  onClose: () => void;
}

const DEFAULT_FOCUS_MINUTES = 25;

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
  selectedMinutes,
  onSelectPreset,
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
          {Array.from(new Set([...settings.timerPresets, DEFAULT_FOCUS_MINUTES, selectedMinutes]))
            .sort((a, b) => a - b)
            .map((minutes) => (
              <button
                key={minutes}
                type="button"
                className={
                  minutes === selectedMinutes
                    ? 'chip bubble-chip bubble-chip-selected'
                    : 'chip bubble-chip'
                }
                aria-pressed={minutes === selectedMinutes}
                onClick={() => onSelectPreset(minutes)}
              >
                {minutes}
              </button>
            ))}
          <button type="button" className="chip bubble-chip" onClick={onStartCountUp}>
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
