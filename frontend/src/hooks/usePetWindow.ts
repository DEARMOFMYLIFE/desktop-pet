import { useCallback, useEffect, useRef, useState } from 'react';
import { api } from '../lib/ipc';
import { t } from '../lib/i18n';
import type { Settings } from '../types';
import { usePetWindowEvents } from './usePetWindowEvents';

const SIZES: Record<Settings['petSize'], number> = { S: 160, M: 208, L: 256 };

export function usePetWindow() {
  const [hint, setHint] = useState(t('pet.hint'));
  const [showHint, setShowHint] = useState(true);
  const reactTimerRef = useRef<number | null>(null);
  const hintTimerRef = useRef<number | null>(null);

  const resetHint = useCallback(() => setHint(t('pet.hint')), []);

  const showTransientHint = useCallback((message: string, duration: number) => {
    setHint(message);
    setShowHint(true);
    if (hintTimerRef.current !== null) window.clearTimeout(hintTimerRef.current);
    hintTimerRef.current = window.setTimeout(() => setShowHint(false), duration);
  }, []);

  const { settings, petState, loadError, setPetState } = usePetWindowEvents({
    resetHint,
    showTransientHint,
  });

  useEffect(() => {
    hintTimerRef.current = window.setTimeout(() => setShowHint(false), 3600);
    return () => {
      if (hintTimerRef.current !== null) window.clearTimeout(hintTimerRef.current);
      if (reactTimerRef.current !== null) window.clearTimeout(reactTimerRef.current);
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

    reactTimerRef.current = window.setTimeout(
      () => setPetState((current) => (current === 'react' ? 'idle' : current)),
      1600,
    );
    showTransientHint(t('pet.click'), 1200);
  }, [setPetState, showTransientHint]);

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
  }, [setPetState]);

  const handleDragMove = useCallback(() => {
    void api.petDragMove().catch((error: unknown) => console.error('drag move failed', error));
  }, []);

  const handleDragEnd = useCallback(() => {
    void api
      .petDragEnd()
      .then((snapshot) => setPetState(snapshot.state))
      .catch((error: unknown) => console.error('drag end failed', error));
  }, [setPetState]);

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
