import { useEffect, useRef, useState } from 'react';
import type { RefObject } from 'react';
import type { PetSkin, PetState } from '../../types';
import type { SpriteManifest, SpriteStateConfig } from '../../lib/animation/manifest';
import { characterAssetUrl } from '../../lib/character/api';
import {
  getCachedManifest,
  getLoadedImage,
  isImageLoaded,
  loadImage,
  loadManifest,
} from '../../lib/character/preload';

export function useSpriteAnimation(
  skin: PetSkin,
  state: PetState,
  reduceMotion: boolean,
): { canvasRef: RefObject<HTMLCanvasElement> } {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [manifest, setManifest] = useState<SpriteManifest | null>(
    () => getCachedManifest(skin),
  );

  useEffect(() => {
    const cached = getCachedManifest(skin);
    if (cached) {
      // Keep the current state as the trigger; the animation effect reads the
      // manifest for the *current* skin from the cache. Re-setting it here
      // would restart the animation twice on every cached skin switch.
      return;
    }
    setManifest(null);
    const canvas = canvasRef.current;
    const ctx = canvas?.getContext('2d');
    if (canvas && ctx) ctx.clearRect(0, 0, canvas.width, canvas.height);
    let cancelled = false;
    void loadManifest(skin)
      .then((next) => {
        if (!cancelled) setManifest(next);
      })
      .catch((error: unknown) => console.warn('sprite manifest unavailable', error));
    return () => {
      cancelled = true;
    };
  }, [skin]);

  useEffect(() => {
    const canvas = canvasRef.current;
    // Read the manifest for the *current* skin: after a switch the state may
    // still hold the previous skin's manifest for one render.
    const active = getCachedManifest(skin);
    const config = active?.states[state];
    if (!canvas || !config || !active) return;
    const ctx = canvas.getContext('2d', { alpha: true });
    if (!ctx) return;

    const [fw, fh] = active.frameSize;
    if (canvas.width !== fw || canvas.height !== fh) {
      canvas.width = fw;
      canvas.height = fh;
    }
    ctx.imageSmoothingEnabled = true;
    ctx.imageSmoothingQuality = 'high';

    const primaryUrl = characterAssetUrl(skin, config.file);
    const fallbackUrl = config.fallback ? characterAssetUrl(skin, config.fallback) : null;
    let cancelled = false;
    let rafId = 0;

    const clear = () => ctx.clearRect(0, 0, fw, fh);
    const draw = (sheet: HTMLImageElement, frame: number) => {
      clear();
      ctx.drawImage(
        sheet,
        frame * fw,
        0,
        fw,
        fh,
        0,
        0,
        fw,
        fh,
      );
    };

    const runAnimation = (sheet: HTMLImageElement, stateConfig: SpriteStateConfig) => {
      cancelAnimationFrame(rafId);
      draw(sheet, 0);
      if (reduceMotion || stateConfig.frames <= 1) return;

      const interval = 1000 / stateConfig.fps;
      const startedAt = performance.now();
      const tick = (now: number) => {
        if (cancelled) return;
        const rawFrame = Math.floor((now - startedAt) / interval);
        const frame = stateConfig.loop
          ? rawFrame % stateConfig.frames
          : Math.min(rawFrame, stateConfig.frames - 1);
        draw(sheet, frame);
        if (stateConfig.loop || frame < stateConfig.frames - 1) {
          rafId = requestAnimationFrame(tick);
        }
      };
      rafId = requestAnimationFrame(tick);
    };

    const start = (stateConfig: SpriteStateConfig, url: string) => {
      const sheet = getLoadedImage(url);
      if (sheet) {
        runAnimation(sheet, stateConfig);
        return true;
      }
      return false;
    };

    clear();
    if (isImageLoaded(primaryUrl) && start(config, primaryUrl)) {
      // started
    } else {
      const idleConfig = active.states.idle;
      const idleUrl = idleConfig ? characterAssetUrl(skin, idleConfig.file) : null;
      if (state !== 'idle' && idleConfig && idleUrl && isImageLoaded(idleUrl)) {
        start(idleConfig, idleUrl);
      }
      void loadImage(primaryUrl, fallbackUrl).then((resolved) => {
        if (cancelled || !resolved) return;
        start(config, resolved);
      });
    }

    return () => {
      cancelled = true;
      cancelAnimationFrame(rafId);
    };
  }, [manifest, skin, state, reduceMotion]);

  return { canvasRef };
}