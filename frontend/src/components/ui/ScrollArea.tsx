import { useCallback, useEffect, useRef, useState } from 'react';
import type { PointerEvent as ReactPointerEvent, ReactNode } from 'react';

interface ScrollAreaProps {
  children: ReactNode;
  className?: string;
  label?: string;
  compact?: boolean;
}

interface ThumbGeometry {
  height: number;
  top: number;
  visible: boolean;
}

const INSET = 4;
const MIN_THUMB = 44;

export function ScrollArea({ children, className = '', label, compact = false }: ScrollAreaProps) {
  const viewportRef = useRef<HTMLDivElement>(null);
  const [thumb, setThumb] = useState<ThumbGeometry>({ height: 0, top: 0, visible: false });

  const update = useCallback(() => {
    const viewport = viewportRef.current;
    if (!viewport) return;
    const track = viewport.clientHeight - INSET * 2;
    const overflow = viewport.scrollHeight - viewport.clientHeight;
    if (track <= 0 || overflow <= 1) {
      setThumb((current) => (current.visible ? { height: 0, top: 0, visible: false } : current));
      return;
    }
    const height = Math.min(track, Math.max(MIN_THUMB, (viewport.clientHeight / viewport.scrollHeight) * track));
    const top = INSET + (viewport.scrollTop / overflow) * Math.max(0, track - height);
    setThumb({ height, top, visible: true });
  }, []);

  useEffect(() => {
    const viewport = viewportRef.current;
    if (!viewport) return;
    update();
    const observer = typeof ResizeObserver === 'undefined' ? null : new ResizeObserver(update);
    observer?.observe(viewport);
    if (observer) {
      for (const child of Array.from(viewport.children)) observer.observe(child);
    }
    window.addEventListener('resize', update);
    return () => {
      observer?.disconnect();
      window.removeEventListener('resize', update);
    };
  }, [update, children]);

  const startDrag = (event: ReactPointerEvent<HTMLDivElement>) => {
    const viewport = viewportRef.current;
    if (!viewport) return;
    event.preventDefault();
    const startY = event.clientY;
    const startScroll = viewport.scrollTop;
    const track = viewport.clientHeight - INSET * 2;
    const maxScroll = viewport.scrollHeight - viewport.clientHeight;
    const dragRange = Math.max(1, track - thumb.height);
    const move = (next: PointerEvent) => {
      const delta = (next.clientY - startY) * (maxScroll / dragRange);
      viewport.scrollTop = startScroll + delta;
      update();
    };
    const stop = () => {
      window.removeEventListener('pointermove', move);
      window.removeEventListener('pointerup', stop);
    };
    window.addEventListener('pointermove', move);
    window.addEventListener('pointerup', stop);
  };

  return (
    <div
      className={`scroll-area${compact ? ' is-compact' : ''}${className ? ` ${className}` : ''}`}
    >
      <div
        ref={viewportRef}
        className="scroll-viewport"
        onScroll={update}
        tabIndex={0}
        aria-label={label}
      >
        {children}
      </div>
      {thumb.visible && (
        <div className="scroll-rail" aria-hidden="true">
          <div
            className="scroll-thumb"
            style={{ height: thumb.height, top: thumb.top }}
            onPointerDown={startDrag}
          />
        </div>
      )}
    </div>
  );
}
