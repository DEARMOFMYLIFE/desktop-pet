import { useEffect, useState } from 'react';
import type { RefObject } from 'react';
import type { SelectMenuPosition } from './types';

export function useSelectPosition(
  open: boolean,
  close: () => void,
  triggerRef: RefObject<HTMLButtonElement | null>,
  menuRef: RefObject<HTMLDivElement | null>,
  optionCount: number,
) {
  const [position, setPosition] = useState<SelectMenuPosition | null>(null);

  useEffect(() => {
    if (!open) return;

    const updatePosition = () => {
      const trigger = triggerRef.current;
      if (!trigger) return;
      const rect = trigger.getBoundingClientRect();
      const estimatedHeight = Math.min(380, optionCount * 36 + 8);
      const openUp =
        window.innerHeight - rect.bottom < estimatedHeight + 12 && rect.top > estimatedHeight;
      setPosition({
        left: rect.left,
        top: Math.max(8, openUp ? rect.top - estimatedHeight - 6 : rect.bottom + 6),
        width: rect.width,
        maxHeight: Math.min(380, window.innerHeight - 16),
      });
    };

    const onPointerDown = (event: MouseEvent) => {
      const target = event.target as Node;
      if (triggerRef.current?.contains(target) || menuRef.current?.contains(target)) return;
      close();
    };

    updatePosition();
    window.addEventListener('resize', updatePosition);
    window.addEventListener('scroll', updatePosition, true);
    document.addEventListener('mousedown', onPointerDown);
    return () => {
      window.removeEventListener('resize', updatePosition);
      window.removeEventListener('scroll', updatePosition, true);
      document.removeEventListener('mousedown', onPointerDown);
    };
  }, [open, optionCount, close, triggerRef, menuRef]);

  return position;
}
