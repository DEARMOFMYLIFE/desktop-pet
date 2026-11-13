import { useEffect, useState } from 'react';
import { isBuiltinAppIcon } from '../lib/appIcons';
import { api } from '../lib/ipc';

const FALLBACK = '/brand/icon-clock.png';
const cache = new Map<string, string>();

export function useAppIconSrc(icon: string): string {
  const [src, setSrc] = useState(() => {
    if (isBuiltinAppIcon(icon)) return `/brand/icon-${icon}.png`;
    return cache.get(icon) ?? FALLBACK;
  });

  useEffect(() => {
    if (isBuiltinAppIcon(icon)) {
      setSrc(`/brand/icon-${icon}.png`);
      return;
    }
    const cached = cache.get(icon);
    if (cached) {
      setSrc(cached);
      return;
    }
    let cancelled = false;
    api
      .appIconData(icon)
      .then((bytes) => {
        if (cancelled) return;
        const blob = new Blob([new Uint8Array(bytes)], { type: 'image/png' });
        const url = URL.createObjectURL(blob);
        cache.set(icon, url);
        setSrc(url);
      })
      .catch(() => {
        if (!cancelled) setSrc(FALLBACK);
      });
    return () => {
      cancelled = true;
    };
  }, [icon]);

  return src;
}