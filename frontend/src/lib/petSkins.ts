import type { PetSkin } from '../types';

export interface PetSkinOption {
  id: PetSkin;
  labelKey: string;
}

export const PET_SKIN_OPTIONS: PetSkinOption[] = [
  { id: 'coral', labelKey: 'settings.appearance.skins.coral' },
  { id: 'honey', labelKey: 'settings.appearance.skins.honey' },
  { id: 'cream', labelKey: 'settings.appearance.skins.cream' },
  { id: 'mint', labelKey: 'settings.appearance.skins.mint' },
  { id: 'peach', labelKey: 'settings.appearance.skins.peach' },
  { id: 'night', labelKey: 'settings.appearance.skins.night' },
  { id: 'rose', labelKey: 'settings.appearance.skins.rose' },
  { id: 'sky', labelKey: 'settings.appearance.skins.sky' },
  { id: 'lavender', labelKey: 'settings.appearance.skins.lavender' },
  { id: 'jade', labelKey: 'settings.appearance.skins.jade' },
  { id: 'silver', labelKey: 'settings.appearance.skins.silver' },
  { id: 'ocean', labelKey: 'settings.appearance.skins.ocean' },
  { id: 'berry', labelKey: 'settings.appearance.skins.berry' },
  { id: 'cocoa', labelKey: 'settings.appearance.skins.cocoa' },
  { id: 'snow', labelKey: 'settings.appearance.skins.snow' },
  { id: 'sunny', labelKey: 'settings.appearance.skins.sunny' },
];

export const PET_SKIN_IDS: PetSkin[] = PET_SKIN_OPTIONS.map((option) => option.id);