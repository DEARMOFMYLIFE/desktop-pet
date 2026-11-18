import type { CharacterStyle } from '../types';

export interface CharacterStyleOption {
  value: CharacterStyle;
  labelKey: string;
}

export const CHARACTER_STYLE_OPTIONS: CharacterStyleOption[] = [
  { value: 'cutegirl', labelKey: 'settings.appearance.styles.cutegirl' },
  { value: 'cuteboy', labelKey: 'settings.appearance.styles.cuteboy' },
];
