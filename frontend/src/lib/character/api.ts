import type { CharacterStyle, PetSkin } from '../../types';
import type { SpriteManifest } from '../animation/manifest';

export function characterBasePath(style: CharacterStyle, skin: PetSkin): string {
  return `/assets/character/${style}/${skin}`;
}

export function characterAssetUrl(style: CharacterStyle, skin: PetSkin, file: string): string {
  return `${characterBasePath(style, skin)}/${file}`;
}

export function characterPreviewUrl(style: CharacterStyle, skin: PetSkin): string {
  return characterAssetUrl(style, skin, 'preview.png');
}

export async function fetchCharacterManifest(
  style: CharacterStyle,
  skin: PetSkin,
): Promise<SpriteManifest> {
  const response = await fetch(characterAssetUrl(style, skin, 'manifest.json'));
  if (!response.ok) throw new Error(`manifest HTTP ${response.status}`);
  return (await response.json()) as SpriteManifest;
}
