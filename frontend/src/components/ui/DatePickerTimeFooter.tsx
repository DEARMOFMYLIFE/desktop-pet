import { t } from '../../lib/i18n';
import { Button } from './Button';
import { NumberInput } from './NumberInput';

interface DatePickerTimeFooterProps {
  hour: number;
  minute: number;
  hasValue: boolean;
  onTimeChange: (hour: number, minute: number) => void;
  onToday: () => void;
  onClear: () => void;
  onClose: () => void;
}

export function DatePickerTimeFooter({
  hour,
  minute,
  hasValue,
  onTimeChange,
  onToday,
  onClear,
  onClose,
}: DatePickerTimeFooterProps) {
  return (
    <>
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
            onChange={(next) => onTimeChange(next, minute)}
          />
          <span className="date-time-colon">:</span>
          <NumberInput
            compact
            min={0}
            max={59}
            value={minute}
            label={t('settings.todos.minute')}
            unit={t('settings.todos.minuteUnit')}
            onChange={(next) => onTimeChange(hour, next)}
          />
        </div>
      </div>

      <div className="date-foot">
        <div className="date-foot-left">
          <Button variant="ghost" size="sm" onClick={onToday}>
            {t('settings.todos.dateToday')}
          </Button>
          {hasValue && (
            <Button variant="ghost" size="sm" onClick={onClear}>
              {t('settings.todos.dateClear')}
            </Button>
          )}
        </div>
        <Button variant="primary" size="sm" onClick={onClose}>
          {t('settings.todos.dateDone')}
        </Button>
      </div>
    </>
  );
}
