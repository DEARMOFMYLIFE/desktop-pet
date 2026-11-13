import type { PetSkin } from '../../types';
import type { SpriteManifest } from '../animation/manifest';

export function characterBasePath(skin: PetSkin): string {
  return `/assets/character/${skin}`;
}

export function characterAssetUrl(skin: PetSkin, file: string): string {
  return `${characterBasePath(skin)}/${file}`;
}

export async function fetchCharacterManifest(skin: PetSkin): Promise<SpriteManifest> {
  const response = await fetch(characterAssetUrl(skin, 'manifest.json'));
  if (!response.ok) throw new Error(`manifest HTTP ${response.status}`);
  return (await response.json()) as SpriteManifest;
}