import type { CharacterStyle, PetSkin } from '../types';
import { PET_SKIN_OPTIONS, type PetSkinOption } from './petSkins';

export interface CharacterStyleOption {
  value: CharacterStyle;
  labelKey: string;
  skins: ReadonlyArray<PetSkinOption>;
}

const HERO_PALETTES = ['coral', 'honey', 'mint', 'sky', 'lavender', 'rose', 'night', 'silver'];
interface HeroCharacter {
  id: string;
  labelKey: string;
}
const HERO_CHARACTERS: Partial<Record<CharacterStyle, ReadonlyArray<HeroCharacter>>> = {
  lighthero: [
    { id: 'girl', labelKey: 'settings.appearance.styleCharacters.lighthero.girl' },
    { id: 'boy', labelKey: 'settings.appearance.styleCharacters.lighthero.boy' },
  ],
  webguard: [
    { id: 'boy', labelKey: 'settings.appearance.styleCharacters.webguard.boy' },
    { id: 'cat', labelKey: 'settings.appearance.styleCharacters.webguard.cat' },
  ],
  ironarmor: [
    { id: 'robot', labelKey: 'settings.appearance.styleCharacters.ironarmor.robot' },
    { id: 'boy', labelKey: 'settings.appearance.styleCharacters.ironarmor.boy' },
  ],
  titanguard: [
    { id: 'robot', labelKey: 'settings.appearance.styleCharacters.titanguard.robot' },
    { id: 'cat', labelKey: 'settings.appearance.styleCharacters.titanguard.cat' },
  ],
  starcaptain: [
    { id: 'girl', labelKey: 'settings.appearance.styleCharacters.starcaptain.girl' },
    { id: 'robot', labelKey: 'settings.appearance.styleCharacters.starcaptain.robot' },
  ],
  capesentinel: [
    { id: 'girl', labelKey: 'settings.appearance.styleCharacters.capesentinel.girl' },
    { id: 'cat', labelKey: 'settings.appearance.styleCharacters.capesentinel.cat' },
  ],
};

function heroSkins(style: CharacterStyle): PetSkinOption[] {
  return (HERO_CHARACTERS[style] ?? []).flatMap((character) =>
    HERO_PALETTES.map((palette) => ({
      id: `${character.id}-${palette}`,
      nameKey: character.labelKey,
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
