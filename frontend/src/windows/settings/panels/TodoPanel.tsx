import { ErrorNotice } from '../../../components/ui/ErrorNotice';
import { useTodoPanel } from '../../../hooks/useTodoPanel';
import { SettingsCard } from '../../../components/settings/SettingsCard';
import { ConfirmDialog } from '../../../components/ui/ConfirmDialog';
import { t } from '../../../lib/i18n';
import type { SettingsPanelProps } from '../settingsTypes';
import { TodoList } from './todo/TodoList';
import { TodoDialog } from './todo/TodoDialog';
import { TodoToolbar } from './todo/TodoToolbar';

export function TodoPanel({ notify, settings }: SettingsPanelProps) {
  const todo = useTodoPanel(notify);

  return (
    <SettingsCard>
      <TodoToolbar
        filter={todo.filter}
        query={todo.query}
        language={settings.language}
        onFilterChange={todo.setFilter}
        onQueryChange={todo.setQuery}
        onNew={todo.openNew}
      />
      {todo.error && !todo.showForm && <ErrorNotice>{todo.error}</ErrorNotice>}

      <TodoList
        todos={todo.visible}
        language={settings.language}
        onToggle={todo.toggle}
        onEdit={todo.openEdit}
        onDelete={todo.requestRemove}
      />
      <TodoDialog
        open={todo.showForm}
        editing={todo.editing !== null}
        title={todo.title}
        note={todo.note}
        due={todo.due}
        remind={todo.remind}
        error={todo.error}
        saving={todo.saving}
        onTitleChange={todo.setTitle}
        onNoteChange={todo.setNote}
        onDueChange={todo.setDue}
        onRemindChange={todo.setRemind}
        onSave={() => void todo.save()}
        onCancel={todo.resetForm}
      />
      <ConfirmDialog
        open={todo.pendingDelete !== null}
        title={t('settings.todos.delete')}
        message={t('settings.todos.deleteConfirm')}
        confirmLabel={t('settings.todos.delete')}
        cancelLabel={t('common.cancel')}
        tone="danger"
        onConfirm={() => void todo.confirmRemove()}
        onCancel={todo.cancelRemove}
      />
    </SettingsCard>
  );
}
