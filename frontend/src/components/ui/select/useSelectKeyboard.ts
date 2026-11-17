import type { KeyboardEvent as ReactKeyboardEvent } from 'react';
import type { SelectOption } from './types';

interface SelectKeyboardOptions<T extends string> {
  open: boolean;
  options: ReadonlyArray<SelectOption<T>>;
  activeIndex: number;
  setActiveIndex: (update: (index: number) => number) => void;
  openMenu: () => void;
  selectAt: (index: number) => void;
  close: () => void;
}

export function selectKeyDown<T extends string>({
  open,
  options,
  activeIndex,
  setActiveIndex,
  openMenu,
  selectAt,
  close,
}: SelectKeyboardOptions<T>) {
  return (event: ReactKeyboardEvent<HTMLButtonElement>) => {
    if (!open) {
      if (['ArrowDown', 'ArrowUp', 'Enter', ' '].includes(event.key)) {
        event.preventDefault();
        openMenu();
      }
      return;
    }

    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault();
        setActiveIndex((index) => Math.min(options.length - 1, index + 1));
        break;
      case 'ArrowUp':
        event.preventDefault();
        setActiveIndex((index) => Math.max(0, index - 1));
        break;
      case 'Home':
        event.preventDefault();
        setActiveIndex(() => 0);
        break;
      case 'End':
        event.preventDefault();
        setActiveIndex(() => options.length - 1);
        break;
      case 'Enter':
      case ' ':
        event.preventDefault();
        selectAt(activeIndex);
        break;
      case 'Escape':
        event.preventDefault();
        close();
        break;
      case 'Tab':
        close();
        break;
      default:
        break;
    }
  };
}
