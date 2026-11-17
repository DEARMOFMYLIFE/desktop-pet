import { useState } from 'react';
import { firstYearOfGrid, pad, parseDate, startOfMonth, ymd } from '../../lib/datePicker';
import { getLocale, t } from '../../lib/i18n';
import type { DatePickerMode, DatePickerPosition } from '../../types';
import { DatePickerCalendar } from './DatePickerCalendar';
import { DatePickerHeader } from './DatePickerHeader';
import { DatePickerTimeFooter } from './DatePickerTimeFooter';

interface DatePickerPopoverProps {
  value: string;
  position: DatePickerPosition;
  onChange: (value: string) => void;
  onClose: () => void;
}

export function DatePickerPopover({ value, position, onChange, onClose }: DatePickerPopoverProps) {
  const selected = parseDate(value);
  const [view, setView] = useState(() => startOfMonth(selected ?? new Date()));
  const [mode, setMode] = useState<DatePickerMode>('days');
  const locale = getLocale();
  const time = value.length >= 16 ? value.slice(11, 16) : '09:00';
  const hour = Number(time.slice(0, 2));
  const minute = Number(time.slice(3, 5));
  const todayKey = ymd(new Date());
  const firstYear = firstYearOfGrid(view);

  const move = (direction: number) => {
    if (mode === 'days') {
      setView(new Date(view.getFullYear(), view.getMonth() + direction, 1));
    } else if (mode === 'months') {
      setView(new Date(view.getFullYear() + direction, view.getMonth(), 1));
    } else {
      setView(new Date(view.getFullYear() + direction * 12, view.getMonth(), 1));
    }
  };

  const selectDay = (date: Date) => {
    onChange(`${ymd(date)}T${time}`);
  };

  const selectMonth = (month: number) => {
    setView(new Date(view.getFullYear(), month, 1));
    setMode('days');
  };

  const selectYear = (year: number) => {
    setView(new Date(year, view.getMonth(), 1));
    setMode('months');
  };

  const selectToday = () => {
    const now = new Date();
    setView(startOfMonth(now));
    onChange(`${ymd(now)}T${time}`);
  };

  const updateTime = (nextHour: number, nextMinute: number) => {
    const datePart = value ? value.slice(0, 10) : todayKey;
    onChange(`${datePart}T${pad(nextHour)}:${pad(nextMinute)}`);
  };

  const clear = () => {
    onChange('');
    onClose();
  };

  return (
    <div
      className="date-popover"
      style={{ left: position.left, top: position.top }}
      role="dialog"
      aria-label={t('settings.todos.datePlaceholder')}
    >
      <DatePickerHeader
        mode={mode}
        view={view}
        firstYear={firstYear}
        locale={locale}
        onMove={move}
        onModeChange={setMode}
      />

      <DatePickerCalendar
        value={value}
        view={view}
        mode={mode}
        onSelectDay={selectDay}
        onSelectMonth={selectMonth}
        onSelectYear={selectYear}
      />

      <DatePickerTimeFooter
        hour={hour}
        minute={minute}
        hasValue={Boolean(value)}
        onTimeChange={updateTime}
        onToday={selectToday}
        onClear={clear}
        onClose={onClose}
      />
    </div>
  );
}
