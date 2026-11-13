import { Check } from 'lucide-react';
import type { RefObject } from 'react';
import { useEffect, useRef } from 'react';
import { createPortal } from 'react-dom';

export interface SelectMenuPosition {
  left: number;
  top: number;
  width: number;
  maxHeight: number;
}

interface SelectMenuProps<T extends string> {
  id: string;
  labelId: string;
  position: SelectMenuPosition;
  options: ReadonlyArray<{ value: T; label: string }>;
  value: T;
  activeIndex: number;
  menuRef: RefObject<HTMLDivElement>;
  onSelect: (index: number) => void;
  onActive: (index: number) => void;
}

export function SelectMenu<T extends string>({
  id,
  labelId,
  position,
  options,
  value,
  activeIndex,
  menuRef,
  onSelect,
  onActive,
}: SelectMenuProps<T>) {
  const optionRefs = useRef<Array<HTMLButtonElement | null>>([]);

  useEffect(() => {
    optionRefs.current[activeIndex]?.scrollIntoView({ block: 'nearest' });
  }, [activeIndex]);

  return createPortal(
    <div
      ref={menuRef}
      id={`${id}-menu`}
      role="listbox"
      aria-labelledby={labelId}
      className="select-menu"
      style={{
        left: position.left,
        top: position.top,
        width: position.width,
        maxHeight: position.maxHeight,
      }}
    >
      {options.map((option, index) => (
        <button
          key={option.value}
          ref={(node) => {
            optionRefs.current[index] = node;
          }}
          id={`${id}-option-${index}`}
          type="button"
          role="option"
          aria-selected={option.value === value}
          className={`select-option${index === activeIndex ? ' is-active' : ''}`}
          onMouseEnter={() => onActive(index)}
          onClick={() => onSelect(index)}
        >
          <span>{option.label}</span>
          {option.value === value && <Check size={14} aria-hidden="true" />}
        </button>
      ))}
    </div>,
    document.body,
  );
}
