import { useCallback, useEffect, useState } from 'react';
import { api } from '../lib/ipc';
import type { CustomIcon } from '../types';

export function useCustomIcons() {
  const [icons, setIcons] = useState<CustomIcon[]>([]);

  useEffect(() => {
    let active = true;
    api
      .customIcons()
      .then((next) => {
        if (active) setIcons(next);
      })
      .catch(() => undefined);
    return () => {
      active = false;
    };
  }, []);

  const importIcon = useCallback(async (name: string, bytes: number[]) => {
    const icon = await api.importAppIcon(name, bytes);
    setIcons((current) => [
      ...current.filter((item) => item.value !== icon.value),
      icon,
    ]);
    return icon;
  }, []);

  return { icons, importIcon };
}
