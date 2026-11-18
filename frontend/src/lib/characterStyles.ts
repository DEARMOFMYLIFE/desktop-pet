import type { CharacterStyle, PetSkin } from '../types';
import { PET_SKIN_OPTIONS, type PetSkinOption } from './petSkins';

export interface CharacterStyleOption {
  value: CharacterStyle;
  labelKey: string;
  skins: ReadonlyArray<PetSkinOption>;
}

const HERO_SKINS: PetSkinOption[] = [
  { id: 'default', labelKey: 'settings.appearance.skins.heroDefault' },
  { id: 'alt', labelKey: 'settings.appearance.skins.heroAlt' },
];

export const CHARACTER_STYLE_OPTIONS: CharacterStyleOption[] = [
  { value: 'cutegirl', labelKey: 'settings.appearance.styles.cutegirl', skins: PET_SKIN_OPTIONS },
  { value: 'cuteboy', labelKey: 'settings.appearance.styles.cuteboy', skins: PET_SKIN_OPTIONS },
  { value: 'lighthero', labelKey: 'settings.appearance.styles.lighthero', skins: HERO_SKINS },
  { value: 'webguard', labelKey: 'settings.appearance.styles.webguard', skins: HERO_SKINS },
  { value: 'ironarmor', labelKey: 'settings.appearance.styles.ironarmor', skins: HERO_SKINS },
  { value: 'titanguard', labelKey: 'settings.appearance.styles.titanguard', skins: HERO_SKINS },
  { value: 'starcaptain', labelKey: 'settings.appearance.styles.starcaptain', skins: HERO_SKINS },
  { value: 'capesentinel', labelKey: 'settings.appearance.styles.capesentinel', skins: HERO_SKINS },
  { value: 'catmouse', labelKey: 'settings.appearance.styles.catmouse', skins: HERO_SKINS },
  { value: 'pighero', labelKey: 'settings.appearance.styles.pighero', skins: HERO_SKINS },
];

export function styleDefinition(style: CharacterStyle): CharacterStyleOption {
  return CHARACTER_STYLE_OPTIONS.find((option) => option.value === style) ?? CHARACTER_STYLE_OPTIONS[0];
}

export function skinsForStyle(style: CharacterStyle): ReadonlyArray<PetSkinOption> {
  return styleDefinition(style).skins;
}

export function defaultSkinForStyle(style: CharacterStyle): PetSkin {
  return styleDefinition(style).skins[0]?.id ?? 'coral';
}

export function isSkinForStyle(style: CharacterStyle, skin: PetSkin): boolean {
  return skinsForStyle(style).some((option) => option.id === skin);
}