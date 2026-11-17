import { ChevronDown, ChevronLeft, ChevronRight } from 'lucide-react';
import { monthLabel } from '../../lib/datePicker';
import { t } from '../../lib/i18n';
import type { DatePickerMode } from '../../types';
import { IconButton } from './IconButton';

interface DatePickerHeaderProps {
  mode: DatePickerMode;
  view: Date;
  firstYear: number;
  locale: string;
  onMove: (direction: number) => void;
  onModeChange: (mode: DatePickerMode) => void;
}

export function DatePickerHeader({
  mode,
  view,
  firstYear,
  locale,
  onMove,
  onModeChange,
}: DatePickerHeaderProps) {
  return (
    <div className="date-head">
      <IconButton
        label={mode === 'days' ? t('settings.todos.prevMonth') : t('settings.todos.prevYear')}
        className="date-nav"
        onClick={() => onMove(-1)}
      >
        <ChevronLeft size={15} aria-hidden="true" />
      </IconButton>
      {mode === 'days' && (
        <button
          type="button"
          className="date-month date-month-button"
          aria-label={t('settings.todos.chooseMonth')}
          onClick={() => onModeChange('months')}
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
          onClick={() => onModeChange('years')}
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
        onClick={() => onMove(1)}
      >
        <ChevronRight size={15} aria-hidden="true" />
      </IconButton>
    </div>
  );
}
