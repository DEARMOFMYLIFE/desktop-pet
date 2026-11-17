import { ClearButton } from '../../../../components/ui/ClearButton';
import { TodoTimeField } from './TodoTimeField';
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
        <TodoTimeField kind="remind" value={remind} onChange={onRemindChange} />
        <TodoTimeField kind="due" value={due} onChange={onDueChange} />
      </div>
      {error && <p className="form-error">{error}</p>}
    </div>
  );
}
