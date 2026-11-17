import { ChevronDown } from 'lucide-react';
import { useCallback, useId, useRef, useState } from 'react';
import { SelectMenu } from './SelectMenu';
import type { SelectOption } from './select/types';
import { selectKeyDown } from './select/useSelectKeyboard';
import { useSelectPosition } from './select/useSelectPosition';

export type { SelectOption } from './select/types';

interface SelectProps<T extends string> {
  value: T;
  options: ReadonlyArray<SelectOption<T>>;
  onChange: (value: T) => void;
  label: string;
  placeholder?: string;
  disabled?: boolean;
  className?: string;
}

export function Select<T extends string>({
  value,
  options,
  onChange,
  label,
  placeholder,
  disabled = false,
  className = '',
}: SelectProps<T>) {
  const id = useId();
  const labelId = `${id}-label`;
  const [open, setOpen] = useState(false);
  const [activeIndex, setActiveIndex] = useState(0);
  const triggerRef = useRef<HTMLButtonElement>(null);
  const menuRef = useRef<HTMLDivElement>(null);
  const selectedIndex = options.findIndex((option) => option.value === value);

  const close = useCallback(() => setOpen(false), []);

  const openMenu = () => {
    if (disabled) return;
    setActiveIndex(Math.max(0, selectedIndex));
    setOpen(true);
  };

  const selectAt = (index: number) => {
    const option = options[index];
    if (!option) return;
    onChange(option.value);
    close();
    triggerRef.current?.focus();
  };

  const position = useSelectPosition(open, close, triggerRef, menuRef, options.length);
  const onKeyDown = selectKeyDown({
    open,
    options,
    activeIndex,
    setActiveIndex,
    openMenu,
    selectAt,
    close,
  });

  const selected = options.find((option) => option.value === value);

  return (
    <>
      <span id={labelId} className="sr-only">
        {label}
      </span>
      <button
        ref={triggerRef}
        type="button"
        className={`select-trigger${open ? ' is-open' : ''}${className ? ` ${className}` : ''}`}
        aria-haspopup="listbox"
        aria-expanded={open}
        aria-labelledby={labelId}
        aria-controls={open ? `${id}-menu` : undefined}
        aria-activedescendant={open ? `${id}-option-${activeIndex}` : undefined}
        disabled={disabled}
        onClick={() => (open ? close() : openMenu())}
        onKeyDown={onKeyDown}
      >
        <span className={`select-label${selected ? '' : ' text-muted'}`}>
          {selected?.label ?? placeholder ?? ''}
        </span>
        <ChevronDown
          size={14}
          className={`shrink-0 text-muted transition-transform ${open ? 'rotate-180' : ''}`}
          aria-hidden="true"
        />
      </button>
      {open && position && (
        <SelectMenu
          id={id}
          labelId={labelId}
          position={position}
          options={options}
          value={value}
          activeIndex={activeIndex}
          menuRef={menuRef}
          onSelect={selectAt}
          onActive={setActiveIndex}
        />
      )}
    </>
  );
}
