import { memo } from 'react';
import { t } from '../../../../lib/i18n';
import type { Todo } from '../../../../types';
import { TodoRow } from './TodoRow';

interface TodoListProps {
  todos: Todo[];
  language: string;
  onToggle: (todo: Todo) => void;
  onEdit: (todo: Todo) => void;
  onDelete: (todo: Todo) => void;
}

export const TodoList = memo(function TodoList({
  todos,
  language,
  onToggle,
  onEdit,
  onDelete,
}: TodoListProps) {
  if (todos.length === 0) {
    return <div className="empty">{t('settings.todos.empty')}</div>;
  }

  return (
    <div className="todo-list" data-language={language}>
      {todos.map((todo) => (
        <TodoRow
          key={todo.id}
          todo={todo}
          language={language}
          onToggle={onToggle}
          onEdit={onEdit}
          onDelete={onDelete}
        />
      ))}
    </div>
  );
});
