import { Clock } from 'lucide-react';
import { memo } from 'react';
import { Button } from '../../../../components/ui/Button';
import { Checkbox } from '../../../../components/ui/Checkbox';
import { t } from '../../../../lib/i18n';
import type { Todo } from '../../../../types';

interface TodoListProps {
  todos: Todo[];
  language: string;
  onToggle: (todo: Todo) => void;
  onEdit: (todo: Todo) => void;
  onDelete: (todo: Todo) => void;
}

export const TodoList = memo(function TodoList({ todos, language, onToggle, onEdit, onDelete }: TodoListProps) {
  if (todos.length === 0) {
    return <div className="empty">{t('settings.todos.empty')}</div>;
  }

  return (
    <div className="todo-list" data-language={language}>
      {todos.map((todo) => (
        <div key={todo.id} className={`todo-row${todo.completed ? ' is-done' : ''}`}>
          <Checkbox
            checked={todo.completed}
            onChange={() => onToggle(todo)}
            ariaLabel={todo.title}
          />
          <div className="todo-main">
            <strong>{todo.title}</strong>
            {todo.note && <small>{todo.note}</small>}
            <div className="todo-meta">
              {todo.dueAt && (
                <span>
                  {t('settings.todos.due')}: {new Date(todo.dueAt).toLocaleString()}
                </span>
              )}
              {todo.remindAt && (
                <span className="inline-flex items-center gap-1">
                  <Clock size={11} aria-hidden="true" />
                  {new Date(todo.remindAt).toLocaleString()}
                </span>
              )}
            </div>
          </div>
          <div className="todo-actions">
            <Button variant="ghost" size="sm" onClick={() => onEdit(todo)}>
              {t('settings.todos.edit')}
            </Button>
            <Button
              variant="ghost"
              size="sm"
              className="todo-delete"
              onClick={() => onDelete(todo)}
            >
              {t('settings.todos.delete')}
            </Button>
          </div>
        </div>
      ))}
    </div>
  );
});
