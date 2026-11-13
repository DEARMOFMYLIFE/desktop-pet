import { useCallback, useDeferredValue, useEffect, useMemo, useState } from 'react';
import { api } from '../lib/ipc';
import { errorMessage } from '../lib/errorMessage';
import { t } from '../lib/i18n';
import type { Todo, TodoFilter, TodoInput } from '../types';

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

export function useTodoPanel(notify?: (message: string) => void) {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [filter, setFilter] = useState<TodoFilter>('all');
  const [query, setQuery] = useState('');
  const [editing, setEditing] = useState<Todo | null>(null);
  const [pendingDelete, setPendingDelete] = useState<Todo | null>(null);
  const [showForm, setShowForm] = useState(false);
  const [title, setTitle] = useState('');
  const [note, setNote] = useState('');
  const [due, setDue] = useState('');
  const [remind, setRemind] = useState('');
  const [error, setError] = useState('');

  const load = () => {
    void api
      .todoList()
      .then(setTodos)
      .catch((reason: unknown) => setError(errorMessage(reason, t('error.generic'))));
  };

  useEffect(() => {
    load();
  }, []);

  const deferredQuery = useDeferredValue(query.trim().toLowerCase());

  const visible = useMemo(() => {
    const today = new Date().toISOString().slice(0, 10);
    const matchesQuery = (todo: Todo) =>
      !deferredQuery ||
      todo.title.toLowerCase().includes(deferredQuery) ||
      todo.note.toLowerCase().includes(deferredQuery);
    if (filter === 'completed') return todos.filter((todo) => todo.completed && matchesQuery(todo));
    if (filter === 'today') {
      return todos.filter(
        (todo) =>
          !todo.completed &&
          todo.dueAt &&
          new Date(todo.dueAt).toISOString().slice(0, 10) === today &&
          matchesQuery(todo),
      );
    }
    return todos.filter(matchesQuery);
  }, [todos, filter, deferredQuery]);

  const resetForm = useCallback(() => {
    setEditing(null);
    setShowForm(false);
    setTitle('');
    setNote('');
    setDue('');
    setRemind('');
    setError('');
  }, []);

  const openNew = useCallback(() => {
    setEditing(null);
    setTitle('');
    setNote('');
    setDue('');
    setRemind('');
    setShowForm(true);
    setError('');
  }, []);

  const openEdit = useCallback((todo: Todo) => {
    setEditing(todo);
    setTitle(todo.title);
    setNote(todo.note);
    setDue(toLocalInput(todo.dueAt));
    setRemind(toLocalInput(todo.remindAt));
    setShowForm(true);
    setError('');
  }, []);

  const save = async () => {
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
    try {
      if (editing) {
        await api.todoUpdate(editing.id, input);
      } else {
        await api.todoCreate(input);
      }
      resetForm();
      load();
      notify?.(t('settings.todos.saved'));
    } catch (reason) {
      setError(errorMessage(reason, t('settings.todos.saveFailed')));
    }
  };

  const toggle = useCallback(async (todo: Todo) => {
    try {
      const next = await api.todoToggle(todo.id, !todo.completed);
      setTodos((current) => current.map((item) => (item.id === next.id ? next : item)));
    } catch (reason) {
      setError(errorMessage(reason, t('error.generic')));
    }
  }, []);

  const requestRemove = useCallback((todo: Todo) => {
    setPendingDelete(todo);
  }, []);

  const cancelRemove = useCallback(() => {
    setPendingDelete(null);
  }, []);

  const confirmRemove = async () => {
    if (!pendingDelete) return;
    try {
      await api.todoDelete(pendingDelete.id);
      setPendingDelete(null);
      load();
      notify?.(t('settings.todos.deleted'));
    } catch (reason) {
      setPendingDelete(null);
      setError(errorMessage(reason, t('error.generic')));
    }
  };

  return {
    todos,
    filter,
    setFilter,
    query,
    setQuery,
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
    error,
    visible,
    pendingDelete,
    resetForm,
    openNew,
    openEdit,
    save,
    toggle,
    requestRemove,
    cancelRemove,
    confirmRemove,
  };
}
