import {
  addDays,
  monthLabels,
  startOfMonth,
  weekdayLabels,
  ymd,
  yearGrid,
} from '../../lib/datePicker';
import { getLocale } from '../../lib/i18n';
import type { DatePickerMode } from '../../types';
import { DatePeriodGrid } from './DatePeriodGrid';

interface DatePickerCalendarProps {
  value: string;
  view: Date;
  mode: DatePickerMode;
  onSelectDay: (date: Date) => void;
  onSelectMonth: (month: number) => void;
  onSelectYear: (year: number) => void;
}

export function DatePickerCalendar({
  value,
  view,
  mode,
  onSelectDay,
  onSelectMonth,
  onSelectYear,
}: DatePickerCalendarProps) {
  const locale = getLocale();
  const selectedKey = value.slice(0, 10);
  const todayKey = ymd(new Date());

  if (mode === 'months') {
    return (
      <DatePeriodGrid
        items={monthLabels(locale).map((label, index) => ({ value: index, label }))}
        selected={view.getMonth()}
        onSelect={onSelectMonth}
      />
    );
  }

  if (mode === 'years') {
    return (
      <DatePeriodGrid
        items={yearGrid(view).map((year) => ({ value: year, label: year }))}
        selected={view.getFullYear()}
        onSelect={onSelectYear}
      />
    );
  }

  const first = startOfMonth(view);
  const offset = (first.getDay() + 6) % 7;
  const gridStart = addDays(first, -offset);
  const days = Array.from({ length: 42 }, (_, index) => addDays(gridStart, index));

  return (
    <>
      <div className="date-weekdays">
        {weekdayLabels(locale).map((weekday) => (
          <span key={weekday} className="date-weekday">
            {weekday}
          </span>
        ))}
      </div>
      <div className="date-grid">
        {days.map((day) => {
          const key = ymd(day);
          const outside = day.getMonth() !== view.getMonth();
          const isSelected = key === selectedKey;
          const isToday = key === todayKey;
          return (
            <button
              key={key}
              type="button"
              className={`date-day${outside ? ' is-outside' : ''}${isToday ? ' is-today' : ''}${isSelected ? ' is-selected' : ''}`}
              aria-label={day.toLocaleDateString(locale)}
              aria-pressed={isSelected}
              onClick={() => onSelectDay(day)}
            >
              {day.getDate()}
            </button>
          );
        })}
      </div>
    </>
  );
}
