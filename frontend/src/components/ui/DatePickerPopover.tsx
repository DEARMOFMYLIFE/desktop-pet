import { ChevronDown, ChevronLeft, ChevronRight } from 'lucide-react';
import { useState } from 'react';
import { firstYearOfGrid, monthLabel, pad, parseDate, startOfMonth, ymd } from '../../lib/datePicker';
import { getLocale, t } from '../../lib/i18n';
import type { DatePickerMode, DatePickerPosition } from '../../types';
import { Button } from './Button';
import { DatePickerCalendar } from './DatePickerCalendar';
import { IconButton } from './IconButton';
import { NumberInput } from './NumberInput';

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
      <div className="date-head">
        <IconButton
          label={mode === 'days' ? t('settings.todos.prevMonth') : t('settings.todos.prevYear')}
          className="date-nav"
          onClick={() => move(-1)}
        >
          <ChevronLeft size={15} aria-hidden="true" />
        </IconButton>
        {mode === 'days' && (
          <button
            type="button"
            className="date-month date-month-button"
            aria-label={t('settings.todos.chooseMonth')}
            onClick={() => setMode('months')}
          >
            {monthLabel(locale, view)}
            <ChevronDown size={12} aria-hidden="true" />
          </button>
        )}
        {mode === 'months' && (
          <button
            type="button"
            className="date-month date-month-button"
            aria-label={t('settings.todos.chooseYear')}
            onClick={() => setMode('years')}
          >
            {view.getFullYear()}
            <ChevronDown size={12} aria-hidden="true" />
          </button>
        )}
        {mode === 'years' && (
          <span className="date-month">
            {firstYear}–{firstYear + 11}
          </span>
        )}
        <IconButton
          label={mode === 'days' ? t('settings.todos.nextMonth') : t('settings.todos.nextYear')}
          className="date-nav"
          onClick={() => move(1)}
        >
          <ChevronRight size={15} aria-hidden="true" />
        </IconButton>
      </div>

      <DatePickerCalendar
        value={value}
        view={view}
        mode={mode}
        onSelectDay={selectDay}
        onSelectMonth={selectMonth}
        onSelectYear={selectYear}
      />

      <div className="date-time">
        <span className="date-time-label">{t('settings.todos.time')}</span>
        <div className="date-time-parts">
          <NumberInput
            compact
            min={0}
            max={23}
            value={hour}
            label={t('settings.todos.hour')}
            unit={t('settings.todos.hourUnit')}
            onChange={(next) => updateTime(next, minute)}
          />
          <span className="date-time-colon">:</span>
          <NumberInput
            compact
            min={0}
            max={59}
            value={minute}
            label={t('settings.todos.minute')}
            unit={t('settings.todos.minuteUnit')}
            onChange={(next) => updateTime(hour, next)}
          />
        </div>
      </div>

      <div className="date-foot">
        <div className="date-foot-left">
          <Button variant="ghost" size="sm" onClick={selectToday}>
            {t('settings.todos.dateToday')}
          </Button>
          {value && (
            <Button variant="ghost" size="sm" onClick={clear}>
              {t('settings.todos.dateClear')}
            </Button>
          )}
        </div>
        <Button variant="primary" size="sm" onClick={onClose}>
          {t('settings.todos.dateDone')}
        </Button>
      </div>
    </div>
  );
}