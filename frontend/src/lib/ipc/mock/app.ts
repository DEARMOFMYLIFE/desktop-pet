import type { Bootstrap, Settings } from '../../../types';
import { IpcError } from '../errors';
import { DEFAULT_SETTINGS } from '../defaults';
import { currentPet } from './pet';
import {
  MOCK_APP,
  openBrowserWindow,
  readMockSettings,
  removeMockSettings,
  writeMockSettings,
} from './settings';
import { currentTimer } from './timer';

export function handleApp<T>(
  cmd: string,
  args?: Record<string, unknown>,
): Promise<T> | null {
  switch (cmd) {
    case 'app_bootstrap': {
      const bootstrap: Bootstrap = {
        settings: readMockSettings(),
        app: MOCK_APP,
        pet: currentPet(),
        timer: currentTimer(),
      };
      return Promise.resolve(bootstrap as T);
    }
    case 'app_info':
      return Promise.resolve(MOCK_APP as T);
    case 'settings_get':
      return Promise.resolve(readMockSettings() as T);
    case 'settings_update': {
      const settings = (args?.settings as Settings | undefined) ?? readMockSettings();
      return Promise.resolve(writeMockSettings(settings) as T);
    }
    case 'click_through_set': {
      const settings = { ...readMockSettings(), clickThrough: Boolean(args?.enabled) };
      return Promise.resolve(writeMockSettings(settings) as T);
    }
    case 'window_open':
      openBrowserWindow(String(args?.label ?? 'settings'));
      return Promise.resolve(undefined as T);
    case 'window_close':
      window.close();
      return Promise.resolve(undefined as T);
    case 'app_quit':
      window.close();
      return Promise.resolve(undefined as T);
    case 'pet_menu':
      return Promise.resolve(undefined as T);
    case 'notification_permission_status':
    case 'notification_permission_request':
      return Promise.resolve('granted' as T);
    case 'shortcut_update': {
      const settings = readMockSettings();
      const action = String(args?.action ?? '');
      const accelerator = String(args?.accelerator ?? '');
      const key =
        action === 'click_through'
          ? 'clickThrough'
          : action === 'toggle_visible'
            ? 'toggleVisible'
            : 'focus';
      return Promise.resolve(
        writeMockSettings({
          ...settings,
          shortcuts: { ...settings.shortcuts, [key]: accelerator },
        }) as T,
      );
    }
    case 'shortcut_reset': {
      const settings = readMockSettings();
      return Promise.resolve(
        writeMockSettings({
          ...settings,
          shortcuts: { ...DEFAULT_SETTINGS.shortcuts },
        }) as T,
      );
    }
    case 'autostart_status':
      return Promise.resolve(false as T);
    case 'custom_icons_list':
      return Promise.resolve([] as T);
    case 'app_icon_import':
      return Promise.reject(
        new IpcError('UNSUPPORTED', 'Custom icons require the desktop app.'),
      );
    case 'app_icon_data':
      return Promise.reject(
        new IpcError('UNSUPPORTED', 'Custom icons require the desktop app.'),
      );
    case 'data_open_folder':
      return Promise.resolve(undefined as T);
    case 'data_clear':
      removeMockSettings();
      if (typeof window !== 'undefined') window.location.reload();
      return Promise.resolve(undefined as T);
    default:
      return null;
  }
}
