import { useEffect, useMemo, useState } from 'react';
import { errorMessage } from '../lib/errorMessage';
import { api } from '../lib/ipc';
import { resolveLocale, setLocale, t } from '../lib/i18n';
import type { ReminderPayload, Settings, TimerSnapshot } from '../types';

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

export function useBubbleWindow() {
  const [timer, setTimer] = useState<TimerSnapshot | null>(null);
  const [settings, setSettings] = useState<Settings | null>(null);
  const [reminder, setReminder] = useState<ReminderPayload | null>(null);
  const [loadError, setLoadError] = useState('');

  useEffect(() => {
    let disposed = false;
    api
      .bootstrap()
      .then((bootstrap) => {
        if (disposed) return;
        setTimer(bootstrap.timer);
        setSettings(bootstrap.settings);
        setLocale(resolveLocale(bootstrap.settings.language));
      })
      .catch((error: unknown) => {
        console.error('bootstrap failed', error);
        if (!disposed) setLoadError(errorMessage(error, t('error.generic')));
      });
    return () => {
      disposed = true;
    };
  }, []);

  useEffect(() => {
    let disposed = false;
    const unlisteners: Array<() => void> = [];
    const keep = (unlisten: () => void) => {
      if (disposed) {
        unlisten();
      } else {
        unlisteners.push(unlisten);
      }
    };
    api
      .on<TimerSnapshot>('timer:changed', (next) => setTimer(next))
      .then(keep);
    api
      .on<Settings>('settings:changed', (next) => {
        setSettings(next);
        setLocale(resolveLocale(next.language));
      })
      .then(keep);
    api
      .on<ReminderPayload>('reminder:fired', (next) => setReminder(next))
      .then(keep);
    return () => {
      disposed = true;
      unlisteners.forEach((unlisten) => unlisten());
    };
  }, []);

  useEffect(() => {
    const refresh = () => {
      void api
        .timerSnapshot()
        .then(setTimer)
        .catch((error: unknown) => console.error('timer refresh failed', error));
    };
    const onVisibility = () => {
      if (!document.hidden) refresh();
    };
    window.addEventListener('focus', refresh);
    document.addEventListener('visibilitychange', onVisibility);
    return () => {
      window.removeEventListener('focus', refresh);
      document.removeEventListener('visibilitychange', onVisibility);
    };
  }, []);

  const progress = useMemo(() => {
    if (!timer || timer.totalSec <= 0) return 0;
    return Math.min(100, Math.max(0, ((timer.totalSec - timer.remainingSec) / timer.totalSec) * 100));
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

  const closeBubble = () => {
    void api.closeWindow('bubble');
  };

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
    timer,
    settings,
    reminder,
    loadError,
    progress,
    active,
    primaryLabel,
    timerLabel: timer ? phaseLabel(timer) : '',
    handlePrimary,
    skip,
    reset,
    startPreset,
    startCountUp,
    closeBubble,
    closeReminder: () => setReminder(null),
    acknowledgeReminder,
    snoozeReminder,
  };
}
