export type Language = 'system' | 'zh-CN' | 'en-US';
export type PetSize = 'S' | 'M' | 'L';
export type AppIcon = string;

export interface CustomIcon {
  value: string;
  name: string;
}

export type PetSkin =
  | 'honey'
  | 'cream'
  | 'mint'
  | 'peach'
  | 'night'
  | 'rose'
  | 'sky'
  | 'lavender'
  | 'jade'
  | 'coral'
  | 'silver'
  | 'ocean'
  | 'berry'
  | 'cocoa'
  | 'snow'
  | 'sunny';
export type ReduceMotion = 'system' | 'on' | 'off';
export type FocusBehavior = 'notify_only' | 'pause';
export type ShortcutAction = 'focus' | 'click_through' | 'toggle_visible';

export interface PetPosition {
  monitor: string | null;
  x: number;
  y: number;
}

export interface PomodoroSettings {
  focusMin: number;
  shortBreakMin: number;
  longBreakMin: number;
  rounds: number;
  autoStartNext: boolean;
}

export interface ShortcutSettings {
  focus: string;
  clickThrough: string;
  toggleVisible: string;
}

export interface ReminderSettings {
  waterEnabled: boolean;
  waterIntervalMin: number;
  sedentaryEnabled: boolean;
  sedentaryIntervalMin: number;
  eyesEnabled: boolean;
  eyesIntervalMin: number;
  activeStart: string;
  activeEnd: string;
  focusBehavior: FocusBehavior;
  snoozeMin: number;
}

export interface Settings {
  language: Language;
  petName: string;
  petSize: PetSize;
  petSkin: PetSkin;
  appIcon: AppIcon;
  petOpacity: number;
  clickThrough: boolean;
  alwaysOnTop: boolean;
  reduceMotion: ReduceMotion;
  autostart: boolean;
  firstRun: boolean;
  petPosition: PetPosition | null;
  pomodoro: PomodoroSettings;
  timerPresets: number[];
  shortcuts: ShortcutSettings;
  reminders: ReminderSettings;
}
