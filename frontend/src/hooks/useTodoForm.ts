import { useCallback, useState } from 'react';
import { api } from '../lib/ipc';
import { errorMessage } from '../lib/errorMessage';
import { t } from '../lib/i18n';
import type { Todo, TodoInput } from '../types';

function toLocalInput(value: string | null): string {
  if (!value) return '';
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return '';
  const local = new Date(date.getTime() - date.getTimezoneOffset() * 60_000);
  return local.toISOString().slice(0, 16);
}

function fromLocalInput(value: string): string | null {
  return value ? new Date(value).toISOString() : null;
}

interface TodoFormOptions {
  notify?: (message: string) => void;
  setError: (message: string) => void;
  reload: () => void;
}

export function useTodoForm({ notify, setError, reload }: TodoFormOptions) {
  const [editing, setEditing] = useState<Todo | null>(null);
  const [showForm, setShowForm] = useState(false);
  const [title, setTitle] = useState('');
  const [note, setNote] = useState('');
  const [due, setDue] = useState('');
  const [remind, setRemind] = useState('');
  const [saving, setSaving] = useState(false);

  const resetForm = useCallback(() => {
    setEditing(null);
    setShowForm(false);
    setTitle('');
    setNote('');
    setDue('');
    setRemind('');
    setError('');
  }, [setError]);

  const openNew = useCallback(() => {
    setEditing(null);
    setTitle('');
    setNote('');
    setDue('');
    setRemind('');
    setShowForm(true);
    setError('');
  }, [setError]);

  const openEdit = useCallback(
    (todo: Todo) => {
      setEditing(todo);
      setTitle(todo.title);
      setNote(todo.note);
      setDue(toLocalInput(todo.dueAt));
      setRemind(toLocalInput(todo.remindAt));
      setShowForm(true);
      setError('');
    },
    [setError],
  );

  const save = async () => {
    if (saving) return;
    const input: TodoInput = {
      title: title.trim(),
      note: note.trim(),
      dueAt: fromLocalInput(due),
      remindAt: fromLocalInput(remind),
    };
    if (!input.title) {
      setError(t('settings.todos.titleRequired'));
      return;
    }
    setSaving(true);
    try {
      if (editing) {
        await api.todoUpdate(editing.id, input);
      } else {
        await api.todoCreate(input);
      }
      resetForm();
      reload();
      notify?.(t('settings.todos.saved'));
    } catch (reason) {
      setError(errorMessage(reason, t('settings.todos.saveFailed')));
    } finally {
      setSaving(false);
    }
  };

  return {
    editing,
    showForm,
    title,
    setTitle,
    note,
    setNote,
    due,
    setDue,
    remind,
    setRemind,
    saving,
    resetForm,
    openNew,
    openEdit,
    save,
  };
}
