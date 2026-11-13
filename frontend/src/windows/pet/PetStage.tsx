import { memo, useEffect, useRef } from 'react';
import type { PointerEvent as ReactPointerEvent } from 'react';
import type { PetPoint, PetStageProps } from '../../types';
import { useSpriteAnimation } from './useSpriteAnimation';

export const PetStage = memo(function PetStage({
  size,
  opacity,
  reduceMotion,
  state,
  skin,
  onClick,
  onDoubleClick,
  onContextMenu,
  onDragStart,
  onDragMove,
  onDragEnd,
}: PetStageProps) {
  const { canvasRef } = useSpriteAnimation(skin, state, reduceMotion);
  const clickTimerRef = useRef<number | null>(null);
  const startPointRef = useRef<PetPoint | null>(null);
  const draggingRef = useRef(false);
  const lastMoveAtRef = useRef(0);

  useEffect(() => {
    return () => {
      if (clickTimerRef.current !== null) {
        window.clearTimeout(clickTimerRef.current);
      }
    };
  }, []);

  const clearClickTimer = () => {
    if (clickTimerRef.current !== null) {
      window.clearTimeout(clickTimerRef.current);
      clickTimerRef.current = null;
    }
  };

  const handlePointerDown = (event: ReactPointerEvent<HTMLDivElement>) => {
    if (event.button !== 0) return;
    startPointRef.current = { x: event.clientX, y: event.clientY };
    draggingRef.current = false;
    lastMoveAtRef.current = 0;
    event.currentTarget.setPointerCapture(event.pointerId);
  };

  const handlePointerMove = (event: ReactPointerEvent<HTMLDivElement>) => {
    const start = startPointRef.current;
    if (!start) return;

    if (!draggingRef.current) {
      const distance = Math.hypot(event.clientX - start.x, event.clientY - start.y);
      if (distance <= 4) return;
      draggingRef.current = true;
      clearClickTimer();
      onDragStart();
    }

    const now = performance.now();
    if (now - lastMoveAtRef.current < 16) return;
    lastMoveAtRef.current = now;
    onDragMove();
  };

  const endPointer = (event: ReactPointerEvent<HTMLDivElement>) => {
    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId);
    }

    const wasDragging = draggingRef.current;
    startPointRef.current = null;
    draggingRef.current = false;

    if (wasDragging) {
      onDragEnd();
      return;
    }

    if (clickTimerRef.current !== null) return;
    clickTimerRef.current = window.setTimeout(() => {
      clickTimerRef.current = null;
      onClick();
    }, 300);
  };

  const handlePointerCancel = (event: ReactPointerEvent<HTMLDivElement>) => {
    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId);
    }
    if (draggingRef.current) onDragEnd();
    startPointRef.current = null;
    draggingRef.current = false;
  };

  return (
    <div
      className="pet-canvas"
      style={{
        width: size,
        height: size,
        opacity,
      }}
      role="button"
      tabIndex={0}
      aria-label="凪"
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={endPointer}
      onPointerCancel={handlePointerCancel}
      onDoubleClick={(event) => {
        event.stopPropagation();
        clearClickTimer();
        onDoubleClick();
      }}
      onContextMenu={(event) => {
        event.preventDefault();
        onContextMenu();
      }}
      onKeyDown={(event) => {
        if (event.key === 'Enter' || event.key === ' ') {
          event.preventDefault();
          onClick();
        }
      }}
    >
      <canvas
        ref={canvasRef}
        className="pet-sprite-canvas"
        aria-hidden="true"
      />
    </div>
  );
});