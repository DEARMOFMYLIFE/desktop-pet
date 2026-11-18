import { useCallback, useEffect, useState } from 'react';
import { errorMessage } from '../lib/errorMessage';
import { api } from '../lib/ipc';
import { preloadSkinStates } from '../lib/character/preload';
import { resolveLocale, setLocale, t } from '../lib/i18n';
import type { PetSnapshot, Settings } from '../types';

interface PetWindowEventsOptions {
  resetHint: () => void;
  showTransientHint: (message: string, duration: number) => void;
}

export function usePetWindowEvents({ resetHint, showTransientHint }: PetWindowEventsOptions) {
  const [settings, setSettings] = useState<Settings | null>(null);
  const [petState, setPetState] = useState<PetSnapshot['state']>('idle');
  const [loadError, setLoadError] = useState('');

  const refreshSettings = useCallback(() => {
    void api
      .getSettings()
      .then((next) => {
        setSettings(next);
        setLocale(resolveLocale(next.language));
      })
      .catch((error: unknown) => console.error('settings refresh failed', error));
  }, []);

  useEffect(() => {
    if (!settings) return;
    preloadSkinStates(settings.characterStyle, settings.petSkin);
  }, [settings]);

  useEffect(() => {
    let disposed = false;
    api
      .bootstrap()
      .then((bootstrap) => {
        if (disposed) return;
        setSettings(bootstrap.settings);
        setPetState(bootstrap.pet.state);
        setLocale(resolveLocale(bootstrap.settings.language));
        resetHint();
      })
      .catch((error: unknown) => {
        console.error('bootstrap failed', error);
        if (!disposed) setLoadError(errorMessage(error, t('error.generic')));
      });
    return () => {
      disposed = true;
    };
  }, [resetHint]);

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
      .on<Settings>('settings:changed', (next) => {
        setSettings(next);
        setLocale(resolveLocale(next.language));
        resetHint();
      })
      .then(keep);
    api
      .on<{ enabled: boolean }>('pet:click_through', ({ enabled }) => {
        setSettings((previous) => (previous ? { ...previous, clickThrough: enabled } : previous));
        showTransientHint(t(enabled ? 'pet.ghostOn' : 'pet.ghostOff'), 1800);
        // The native toggle already persisted the value; read it back so a
        // late duplicate event cannot leave the pet in the old visual state.
        refreshSettings();
      })
      .then(keep);
    api
      .on<PetSnapshot>('pet:changed', (snapshot) => {
        setPetState(snapshot.state);
      })
      .then(keep);
    return () => {
      disposed = true;
      unlisteners.forEach((unlisten) => unlisten());
    };
  }, [refreshSettings, resetHint, showTransientHint]);

  useEffect(() => {
    const refresh = () => {
      void api
        .petSnapshot()
        .then((snapshot) => setPetState(snapshot.state))
        .catch((error: unknown) => console.error('pet refresh failed', error));
      refreshSettings();
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
  }, [refreshSettings]);

  return { settings, petState, loadError, setPetState };
}
