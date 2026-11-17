import { useEffect, useState } from 'react';
import { errorMessage } from '../lib/errorMessage';
import { api } from '../lib/ipc';
import { resolveLocale, setLocale, t } from '../lib/i18n';
import type { ReminderPayload, Settings, TimerSnapshot } from '../types';
import { useBubbleReminder } from './useBubbleReminder';
import { useBubbleTimer } from './useBubbleTimer';

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
    api.on<TimerSnapshot>('timer:changed', (next) => setTimer(next)).then(keep);
    api
      .on<Settings>('settings:changed', (next) => {
        setSettings(next);
        setLocale(resolveLocale(next.language));
      })
      .then(keep);
    api.on<ReminderPayload>('reminder:fired', (next) => setReminder(next)).then(keep);
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

  const timerApi = useBubbleTimer(timer, setTimer);
  const reminderApi = useBubbleReminder(reminder, setReminder, settings);

  return {
    timer,
    settings,
    reminder,
    loadError,
    ...timerApi,
    ...reminderApi,
    closeBubble: () => {
      void api.closeWindow('bubble');
    },
  };
}
