import type { CharacterStyle, PetSkin } from '../types';
import { PET_SKIN_OPTIONS, type PetSkinOption } from './petSkins';

export interface CharacterStyleOption {
  value: CharacterStyle;
  labelKey: string;
  skins: ReadonlyArray<PetSkinOption>;
}

const HERO_PALETTES = ['coral', 'honey', 'mint', 'sky', 'lavender', 'rose', 'night', 'silver'];
const HERO_CHARACTERS: Partial<Record<CharacterStyle, ReadonlyArray<string>>> = {
  lighthero: ['girl', 'boy'],
  webguard: ['boy', 'cat'],
  ironarmor: ['robot', 'boy'],
  titanguard: ['robot', 'cat'],
  starcaptain: ['girl', 'robot'],
  capesentinel: ['girl', 'cat'],
};

function heroSkins(style: CharacterStyle): PetSkinOption[] {
  return (HERO_CHARACTERS[style] ?? []).flatMap((character) =>
    HERO_PALETTES.map((palette) => ({
      id: `${character}-${palette}`,
      nameKey: `settings.appearance.characters.${character}`,
      paletteKey: `settings.appearance.skins.${palette}`,
    })),
  );
}

export const CHARACTER_STYLE_OPTIONS: CharacterStyleOption[] = [
  { value: 'cutegirl', labelKey: 'settings.appearance.styles.cutegirl', skins: PET_SKIN_OPTIONS },
  { value: 'cuteboy', labelKey: 'settings.appearance.styles.cuteboy', skins: PET_SKIN_OPTIONS },
  { value: 'lighthero', labelKey: 'settings.appearance.styles.lighthero', skins: heroSkins('lighthero') },
  { value: 'webguard', labelKey: 'settings.appearance.styles.webguard', skins: heroSkins('webguard') },
  { value: 'ironarmor', labelKey: 'settings.appearance.styles.ironarmor', skins: heroSkins('ironarmor') },
  { value: 'titanguard', labelKey: 'settings.appearance.styles.titanguard', skins: heroSkins('titanguard') },
  { value: 'starcaptain', labelKey: 'settings.appearance.styles.starcaptain', skins: heroSkins('starcaptain') },
  { value: 'capesentinel', labelKey: 'settings.appearance.styles.capesentinel', skins: heroSkins('capesentinel') },
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
