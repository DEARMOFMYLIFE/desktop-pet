import { useCallback, useEffect, useRef, useState } from 'react';
import { errorMessage } from '../lib/errorMessage';
import { api } from '../lib/ipc';
import { preloadAllSkins, preloadSkinStates } from '../lib/character/preload';
import { resolveLocale, setLocale, t } from '../lib/i18n';
import type { PetSnapshot, Settings } from '../types';

const SIZES: Record<Settings['petSize'], number> = { S: 160, M: 208, L: 256 };

export function usePetWindow() {
  const [settings, setSettings] = useState<Settings | null>(null);
  const [petState, setPetState] = useState<PetSnapshot['state']>('idle');
  const [hint, setHint] = useState(t('pet.hint'));
  const [showHint, setShowHint] = useState(true);
  const [loadError, setLoadError] = useState('');
  const reactTimerRef = useRef<number | null>(null);
  const hintTimerRef = useRef<number | null>(null);

  useEffect(() => {
    preloadAllSkins();
  }, []);

  useEffect(() => {
    if (!settings) return;
    preloadSkinStates(settings.petSkin);
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
        setHint(t('pet.hint'));
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
      .on<Settings>('settings:changed', (next) => {
        setSettings(next);
        setLocale(resolveLocale(next.language));
        setHint(t('pet.hint'));
      })
      .then(keep);
    api
      .on<{ enabled: boolean }>('pet:click_through', ({ enabled }) => {
        setSettings((previous) => (previous ? { ...previous, clickThrough: enabled } : previous));
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
  }, []);

  useEffect(() => {
    hintTimerRef.current = window.setTimeout(() => setShowHint(false), 3600);
    return () => {
      if (hintTimerRef.current !== null) window.clearTimeout(hintTimerRef.current);
    };
  }, []);

  useEffect(() => {
    return () => {
      if (reactTimerRef.current !== null) window.clearTimeout(reactTimerRef.current);
      if (hintTimerRef.current !== null) window.clearTimeout(hintTimerRef.current);
    };
  }, []);

  useEffect(() => {
    const refresh = () => {
      void api
        .petSnapshot()
        .then((snapshot) => setPetState(snapshot.state))
        .catch((error: unknown) => console.error('pet refresh failed', error));
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

  const reduceMotion =
    settings?.reduceMotion === 'on' ||
    (settings?.reduceMotion === 'system' &&
      typeof window.matchMedia === 'function' &&
      window.matchMedia('(prefers-reduced-motion: reduce)').matches);
  const opacity = settings
    ? settings.clickThrough
      ? Math.min(settings.petOpacity, 0.6)
      : settings.petOpacity
    : 1;

  const handleClick = useCallback(() => {
    void api
      .petClick()
      .then((snapshot) => setPetState(snapshot.state))
      .catch((error: unknown) => console.error('pet click failed', error));

    if (reactTimerRef.current !== null) window.clearTimeout(reactTimerRef.current);
    if (hintTimerRef.current !== null) window.clearTimeout(hintTimerRef.current);

    reactTimerRef.current = window.setTimeout(
      () => setPetState((current) => (current === 'react' ? 'idle' : current)),
      1600,
    );
    setHint(t('pet.click'));
    setShowHint(true);
    hintTimerRef.current = window.setTimeout(() => setShowHint(false), 900);
  }, []);

  const handleDoubleClick = useCallback(() => {
    void api.openWindow('settings');
  }, []);

  const handleContextMenu = useCallback(() => {
    void api.petMenu();
  }, []);

  const handleDragStart = useCallback(() => {
    void api
      .petDragStart()
      .then((snapshot) => setPetState(snapshot.state))
      .catch((error: unknown) => console.error('drag start failed', error));
  }, []);

  const handleDragMove = useCallback(() => {
    void api.petDragMove().catch((error: unknown) => console.error('drag move failed', error));
  }, []);

  const handleDragEnd = useCallback(() => {
    void api
      .petDragEnd()
      .then((snapshot) => setPetState(snapshot.state))
      .catch((error: unknown) => console.error('drag end failed', error));
  }, []);

  return {
    settings,
    petState,
    hint,
    showHint,
    loadError,
    reduceMotion,
    opacity,
    size: settings ? SIZES[settings.petSize] : SIZES.M,
    handleClick,
    handleDoubleClick,
    handleContextMenu,
    handleDragStart,
    handleDragMove,
    handleDragEnd,
  };
}
