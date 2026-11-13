import type { PetSnapshot } from './pet';
import type { Settings } from './settings';
import type { TimerSnapshot } from './timer';

export interface AppInfo {
  name: string;
  version: string;
  platform: string;
}

export interface Bootstrap {
  settings: Settings;
  app: AppInfo;
  pet: PetSnapshot;
  timer: TimerSnapshot;
}
