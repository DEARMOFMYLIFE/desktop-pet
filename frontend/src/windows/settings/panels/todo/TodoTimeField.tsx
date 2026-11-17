import { DateTimePicker } from '../../../../components/ui/DateTimePicker';
import { t } from '../../../../lib/i18n';
import type { TodoTimeKind } from '../../../../types';

interface TodoTimeFieldProps {
  kind: TodoTimeKind;
  value: string;
  onChange: (value: string) => void;
}

export function TodoTimeField({ kind, value, onChange }: TodoTimeFieldProps) {
  const label = t(`settings.todos.${kind}`);
  return (
    <div className="todo-time-field">
      <span>{label}</span>
      <DateTimePicker
        value={value}
        onChange={onChange}
        label={label}
        placeholder={t('settings.todos.datePlaceholder')}
      />
    </div>
  );
}
