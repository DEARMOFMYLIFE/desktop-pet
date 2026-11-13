import { useState } from 'react';
import type { PomodoroSettings, Settings } from '../types';

export const POMODORO_LIMITS: Record<
  keyof Omit<PomodoroSettings, 'autoStartNext'>,
  [number, number]
> = {
  focusMin: [1, 120],
  shortBreakMin: [1, 60],
  longBreakMin: [1, 120],
  rounds: [2, 8],
};

function clamp(value: number, [min, max]: [number, number]): number {
  if (!Number.isFinite(value)) return min;
  return Math.min(max, Math.max(min, Math.round(value)));
}

export function useTimerPanel(
  settings: Settings,
  update: (patch: Partial<Settings>) => Promise<void>,
) {
  const [presetDraft, setPresetDraft] = useState('');

  const setPomodoro = (key: keyof PomodoroSettings, value: number | boolean) => {
    const next = { ...settings.pomodoro, [key]: value };
    if (typeof value === 'number' && key !== 'autoStartNext') {
      next[key] = clamp(value, POMODORO_LIMITS[key as keyof typeof POMODORO_LIMITS]);
    }
    void update({ pomodoro: next });
  };

  const addPreset = () => {
    const minutes = clamp(Number(presetDraft), [1, 1440]);
    if (!presetDraft.trim() || settings.timerPresets.includes(minutes)) return;
    const next = [...settings.timerPresets, minutes].sort((a, b) => a - b);
    setPresetDraft('');
    void update({ timerPresets: next });
  };

  const removePreset = (minutes: number) => {
    void update({ timerPresets: settings.timerPresets.filter((value) => value !== minutes) });
  };

  return {
    presetDraft,
    setPresetDraft,
    limits: POMODORO_LIMITS,
    setPomodoro,
    addPreset,
    removePreset,
  };
}
