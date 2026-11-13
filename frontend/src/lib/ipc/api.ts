import { getCurrentWindow } from '@tauri-apps/api/window';
import type {
  AppInfo,
  Bootstrap,
  CustomIcon,
  PetSnapshot,
  ReminderRow,
  Settings,
  ShortcutAction,
  StatsRange,
  StatsResult,
  TimerSnapshot,
  TimerStartRequest,
  Todo,
  TodoInput,
} from '../../types';
import { call, on } from './core';

type WindowLabel = 'settings' | 'bubble' | 'onboarding' | 'pet';

export const api = {
  bootstrap: () => call<Bootstrap>('app_bootstrap'),
  info: () => call<AppInfo>('app_info'),
  getSettings: () => call<Settings>('settings_get'),
  updateSettings: (settings: Settings) => call<Settings>('settings_update', { settings }),
  setClickThrough: (enabled: boolean) => call<Settings>('click_through_set', { enabled }),
  openWindow: (label: WindowLabel) => call<void>('window_open', { label }),
  closeWindow: (label: WindowLabel) => call<void>('window_close', { label }),
  petClick: () => call<PetSnapshot>('pet_click'),
  petSnapshot: () => call<PetSnapshot>('pet_snapshot'),
  petDragStart: () => call<PetSnapshot>('pet_drag_start'),
  petDragMove: () => call<PetSnapshot>('pet_drag_move'),
  petDragEnd: () => call<PetSnapshot>('pet_drag_end'),
  petMenu: () => call<void>('pet_menu'),
  timerSnapshot: () => call<TimerSnapshot>('timer_snapshot'),
  timerStart: (request: TimerStartRequest) => call<TimerSnapshot>('timer_start', { request }),
  timerPause: () => call<TimerSnapshot>('timer_pause'),
  timerResume: () => call<TimerSnapshot>('timer_resume'),
  timerSkip: () => call<TimerSnapshot>('timer_skip'),
  timerStop: () => call<TimerSnapshot>('timer_stop'),
  timerReset: () => call<TimerSnapshot>('timer_reset'),
  notificationStatus: () => call<string>('notification_permission_status'),
  requestNotificationPermission: () => call<string>('notification_permission_request'),
  updateShortcut: (action: ShortcutAction, accelerator: string) =>
    call<Settings>('shortcut_update', { action, accelerator }),
  resetShortcuts: () => call<Settings>('shortcut_reset'),
  autostartStatus: () => call<boolean>('autostart_status'),
  reminderList: () => call<ReminderRow[]>('reminder_list'),
  reminderAck: (kind: string) => call<void>('reminder_ack', { kind }),
  reminderSnooze: (kind: string, minutes: number) =>
    call<void>('reminder_snooze', { kind, minutes }),
  statsQuery: (range: StatsRange) => call<StatsResult>('stats_query', { range }),
  todoList: () => call<Todo[]>('todo_list'),
  todoCreate: (input: TodoInput) => call<Todo>('todo_create', { input }),
  todoUpdate: (id: number, input: TodoInput) => call<Todo>('todo_update', { id, input }),
  todoToggle: (id: number, completed: boolean) =>
    call<Todo>('todo_toggle', { id, completed }),
  todoDelete: (id: number) => call<void>('todo_delete', { id }),
  todoReorder: (ids: number[]) => call<void>('todo_reorder', { ids }),
  customIcons: () => call<CustomIcon[]>('custom_icons_list'),
  importAppIcon: (name: string, bytes: number[]) =>
    call<CustomIcon>('app_icon_import', { name, bytes }),
  appIconData: (value: string) => call<number[]>('app_icon_data', { value }),
  openDataFolder: () => call<void>('data_open_folder'),
  clearData: () => call<void>('data_clear'),
  quit: () => call<void>('app_quit'),
  startWindowDrag: () => {
    if (typeof window === 'undefined' || !('__TAURI_INTERNALS__' in window)) {
      return Promise.resolve();
    }
    return getCurrentWindow().startDragging();
  },
  on,
};
