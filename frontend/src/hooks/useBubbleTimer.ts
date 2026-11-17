import { useMemo } from 'react';
import { api } from '../lib/ipc';
import { t } from '../lib/i18n';
import type { TimerSnapshot } from '../types';

function phaseLabel(timer: TimerSnapshot): string {
  if (timer.kind === 'countdown') return t('bubble.countdown');
  if (timer.kind === 'countup') return t('bubble.countUp');
  switch (timer.phase) {
    case 'short_break':
      return t('bubble.shortBreak');
    case 'long_break':
      return t('bubble.longBreak');
    default:
      return t('bubble.focus');
  }
}

type SetTimer = (next: TimerSnapshot | null) => void;

export function useBubbleTimer(timer: TimerSnapshot | null, setTimer: SetTimer) {
  const progress = useMemo(() => {
    if (!timer || timer.totalSec <= 0) return 0;
    return Math.min(
      100,
      Math.max(0, ((timer.totalSec - timer.remainingSec) / timer.totalSec) * 100),
    );
  }, [timer]);

  const active = timer ? timer.status !== 'idle' || timer.phase !== null : false;
  const isRunning = timer?.status === 'running';
  const isPaused = timer?.status === 'paused';
  const canStartPending = timer?.status === 'idle' && timer.phase !== null;
  const primaryLabel = !timer
    ? ''
    : isRunning
      ? t('bubble.pause')
      : isPaused
        ? t('bubble.resume')
        : canStartPending
          ? timer.phase === 'focus'
            ? t('bubble.resumeFocus')
            : t('bubble.startBreak')
          : t('bubble.startFocus');

  const handlePrimary = () => {
    if (!timer) return;
    if (isRunning) {
      void api.timerPause().then(setTimer).catch(console.error);
    } else if (isPaused || canStartPending) {
      void api.timerResume().then(setTimer).catch(console.error);
    } else {
      void api.timerStart({ kind: 'pomodoro' }).then(setTimer).catch(console.error);
    }
  };

  const skip = () => {
    void api.timerSkip().then(setTimer).catch(console.error);
  };

  const reset = () => {
    void api.timerReset().then(setTimer).catch(console.error);
  };

  const startPreset = (minutes: number) => {
    void api
      .timerStart({ kind: 'countdown', seconds: minutes * 60 })
      .then(setTimer)
      .catch(console.error);
  };

  const startCountUp = () => {
    void api.timerStart({ kind: 'countup' }).then(setTimer).catch(console.error);
  };

  return {
    progress,
    active,
    primaryLabel,
    timerLabel: timer ? phaseLabel(timer) : '',
    handlePrimary,
    skip,
    reset,
    startPreset,
    startCountUp,
  };
}
