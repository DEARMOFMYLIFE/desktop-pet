import { ClearButton } from '../../../../components/ui/ClearButton';
import { DateTimePicker } from '../../../../components/ui/DateTimePicker';
import { Input, Textarea } from '../../../../components/ui/Input';
import { t } from '../../../../lib/i18n';

interface TodoFormProps {
  title: string;
  note: string;
  due: string;
  remind: string;
  error: string;
  onTitleChange: (value: string) => void;
  onNoteChange: (value: string) => void;
  onDueChange: (value: string) => void;
  onRemindChange: (value: string) => void;
}

export function TodoForm({
  title,
  note,
  due,
  remind,
  error,
  onTitleChange,
  onNoteChange,
  onDueChange,
  onRemindChange,
}: TodoFormProps) {
  return (
    <div className="todo-form">
      <div className="clearable-field">
        <Input
          autoFocus
          placeholder={t('settings.todos.titlePlaceholder')}
          value={title}
          maxLength={120}
          invalid={Boolean(error) && !title.trim()}
          onChange={(event) => onTitleChange(event.target.value)}
        />
        {title && <ClearButton onClick={() => onTitleChange('')} />}
      </div>
      <div className="clearable-field is-textarea">
        <Textarea
          className="todo-note"
          placeholder={t('settings.todos.notePlaceholder')}
          value={note}
          maxLength={500}
          onChange={(event) => onNoteChange(event.target.value)}
        />
        {note && <ClearButton onClick={() => onNoteChange('')} />}
      </div>
      <div className="todo-time-grid">
        <div className="todo-time-field">
          <span>{t('settings.todos.due')}</span>
          <DateTimePicker
            value={due}
            onChange={onDueChange}
            label={t('settings.todos.due')}
            placeholder={t('settings.todos.datePlaceholder')}
          />
        </div>
        <div className="todo-time-field">
          <span>{t('settings.todos.remind')}</span>
          <DateTimePicker
            value={remind}
            onChange={onRemindChange}
            label={t('settings.todos.remind')}
            placeholder={t('settings.todos.datePlaceholder')}
          />
        </div>
      </div>
      {error && <div className="todo-error">{error}</div>}
    </div>
  );
}
