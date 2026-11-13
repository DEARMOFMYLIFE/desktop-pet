import type { PetSkin } from '../../types';
import type { SpriteManifest } from '../animation/manifest';
import { characterAssetUrl, fetchCharacterManifest } from './api';

const manifestCache = new Map<PetSkin, SpriteManifest>();
const pendingManifests = new Map<PetSkin, Promise<SpriteManifest>>();
const loadedImages = new Set<string>();
const imageCache = new Map<string, HTMLImageElement>();
const pendingImages = new Map<string, Promise<string | null>>();
const statesPreloaded = new Set<PetSkin>();

export function getCachedManifest(skin: PetSkin): SpriteManifest | null {
  return manifestCache.get(skin) ?? null;
}

export function isImageLoaded(url: string): boolean {
  return loadedImages.has(url);
}

export function getLoadedImage(url: string): HTMLImageElement | null {
  return imageCache.get(url) ?? null;
}

export function loadManifest(skin: PetSkin): Promise<SpriteManifest> {
  const cached = manifestCache.get(skin);
  if (cached) return Promise.resolve(cached);
  const pending = pendingManifests.get(skin);
  if (pending) return pending;
  const request = fetchCharacterManifest(skin)
    .then((manifest) => {
      manifestCache.set(skin, manifest);
      pendingManifests.delete(skin);
      return manifest;
    })
    .catch((error: unknown) => {
      pendingManifests.delete(skin);
      throw error;
    });
  pendingManifests.set(skin, request);
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

export function preloadSkinStates(skin: PetSkin): void {
  if (statesPreloaded.has(skin)) return;
  statesPreloaded.add(skin);
  void loadManifest(skin)
    .then((manifest) => {
      for (const config of Object.values(manifest.states)) {
        void loadImage(
          characterAssetUrl(skin, config.file),
          config.fallback ? characterAssetUrl(skin, config.fallback) : null,
        );
      }
    })
    .catch((error: unknown) => console.warn('sprite state preload failed', skin, error));
}
