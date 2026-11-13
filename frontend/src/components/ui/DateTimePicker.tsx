import { CalendarDays } from 'lucide-react';
import { useEffect, useRef, useState } from 'react';
import { createPortal } from 'react-dom';
import { formatDateTime } from '../../lib/datePicker';
import { getLocale, t } from '../../lib/i18n';
import type { DatePickerPosition } from '../../types';
import { ClearButton } from './ClearButton';
import { DatePickerPopover } from './DatePickerPopover';

interface DateTimePickerProps {
  value: string;
  onChange: (value: string) => void;
  label: string;
  placeholder?: string;
}

export function DateTimePicker({ value, onChange, label, placeholder }: DateTimePickerProps) {
  const triggerRef = useRef<HTMLButtonElement>(null);
  const [open, setOpen] = useState(false);
  const [position, setPosition] = useState<DatePickerPosition | null>(null);

  useEffect(() => {
    if (!open) return;

    const updatePosition = () => {
      const trigger = triggerRef.current;
      if (!trigger) return;
      const rect = trigger.getBoundingClientRect();
      const width = 304;
      const height = 360;
      const spaceBelow = window.innerHeight - rect.bottom;
      const openUp = spaceBelow < height && rect.top > height;
      setPosition({
        left: Math.max(12, Math.min(rect.left, window.innerWidth - width - 12)),
        top: openUp ? Math.max(12, rect.top - height - 8) : rect.bottom + 8,
      });
    };

    const onPointerDown = (event: MouseEvent) => {
      const target = event.target as Element;
      if (triggerRef.current?.contains(target) || target.closest?.('.date-popover')) return;
      setOpen(false);
    };

    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') setOpen(false);
    };

    updatePosition();
    window.addEventListener('resize', updatePosition);
    window.addEventListener('scroll', updatePosition, true);
    document.addEventListener('mousedown', onPointerDown, true);
    document.addEventListener('keydown', onKeyDown);
    return () => {
      window.removeEventListener('resize', updatePosition);
      window.removeEventListener('scroll', updatePosition, true);
      document.removeEventListener('mousedown', onPointerDown, true);
      document.removeEventListener('keydown', onKeyDown);
    };
  }, [open]);

  return (
    <div className="date-field">
      <button
        ref={triggerRef}
        type="button"
        className={`date-trigger${value ? ' has-value' : ''}`}
        aria-haspopup="dialog"
        aria-expanded={open}
        aria-label={label}
        onClick={() => setOpen((current) => !current)}
      >
        <CalendarDays className="date-trigger-icon" size={15} aria-hidden="true" />
        <span className={value ? 'date-value' : 'date-placeholder'}>
          {value ? formatDateTime(value, getLocale()) : (placeholder ?? t('settings.todos.datePlaceholder'))}
        </span>
      </button>
      {value && <ClearButton className="date-clear" onClick={() => onChange('')} />}
      {open &&
        position &&
        createPortal(
          <DatePickerPopover
            value={value}
            position={position}
            onChange={onChange}
            onClose={() => setOpen(false)}
          />,
          document.body,
        )}
    </div>
  );
}