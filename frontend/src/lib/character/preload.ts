import type { CharacterStyle, PetSkin } from '../../types';
import type { SpriteManifest } from '../animation/manifest';
import { characterAssetUrl, fetchCharacterManifest } from './api';

type CharacterKey = `${CharacterStyle}:${PetSkin}`;

const manifestCache = new Map<CharacterKey, SpriteManifest>();
const pendingManifests = new Map<CharacterKey, Promise<SpriteManifest>>();
const loadedImages = new Set<string>();
const imageCache = new Map<string, HTMLImageElement>();
const pendingImages = new Map<string, Promise<string | null>>();
const statesPreloaded = new Set<CharacterKey>();

function key(style: CharacterStyle, skin: PetSkin): CharacterKey {
  return `${style}:${skin}`;
}

export function getCachedManifest(
  style: CharacterStyle,
  skin: PetSkin,
): SpriteManifest | null {
  return manifestCache.get(key(style, skin)) ?? null;
}

export function isImageLoaded(url: string): boolean {
  return loadedImages.has(url);
}

export function getLoadedImage(url: string): HTMLImageElement | null {
  return imageCache.get(url) ?? null;
}

export function loadManifest(
  style: CharacterStyle,
  skin: PetSkin,
): Promise<SpriteManifest> {
  const cacheKey = key(style, skin);
  const cached = manifestCache.get(cacheKey);
  if (cached) return Promise.resolve(cached);
  const pending = pendingManifests.get(cacheKey);
  if (pending) return pending;
  const request = fetchCharacterManifest(style, skin)
    .then((manifest) => {
      manifestCache.set(cacheKey, manifest);
      pendingManifests.delete(cacheKey);
      return manifest;
    })
    .catch((error: unknown) => {
      pendingManifests.delete(cacheKey);
      throw error;
    });
  pendingManifests.set(cacheKey, request);
  return request;
}

/** Load one image; resolve to the URL that actually loaded (webp or png fallback). */
export function loadImage(url: string, fallbackUrl?: string | null): Promise<string | null> {
  if (loadedImages.has(url)) return Promise.resolve(url);
  const pending = pendingImages.get(url);
  if (pending) return pending;
  const request = new Promise<string | null>((resolve) => {
    const image = new Image();
    image.decoding = 'async';
    image.onload = () => {
      loadedImages.add(url);
      imageCache.set(url, image);
      if (typeof image.decode === 'function') {
        void image
          .decode()
          .catch(() => undefined)
          .then(() => resolve(url));
      } else {
        resolve(url);
      }
    };
    image.onerror = () => {
      if (fallbackUrl && fallbackUrl !== url) {
        void loadImage(fallbackUrl).then((resolved) => resolve(resolved));
        return;
      }
      resolve(null);
    };
    image.src = url;
  }).finally(() => {
    pendingImages.delete(url);
  });
  pendingImages.set(url, request);
  return request;
}

export function preloadSkinStates(style: CharacterStyle, skin: PetSkin): void {
  const cacheKey = key(style, skin);
  if (statesPreloaded.has(cacheKey)) return;
  statesPreloaded.add(cacheKey);
  void loadManifest(style, skin)
    .then((manifest) => {
      for (const config of Object.values(manifest.states)) {
        void loadImage(
          characterAssetUrl(style, skin, config.file),
          config.fallback ? characterAssetUrl(style, skin, config.fallback) : null,
        );
      }
    })
    .catch((error: unknown) => console.warn('sprite state preload failed', cacheKey, error));
}
