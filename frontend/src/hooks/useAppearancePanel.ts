import { useEffect, useState } from 'react';
import type { Settings } from '../types';

export function useAppearancePanel(
  settings: Settings,
  update: (patch: Partial<Settings>) => Promise<void>,
) {
  const [opacity, setOpacity] = useState(settings.petOpacity);

  useEffect(() => {
    setOpacity(settings.petOpacity);
  }, [settings.petOpacity]);

  const commitOpacity = () => {
    void update({ petOpacity: opacity });
  };

  return { opacity, setOpacity, commitOpacity };
}
