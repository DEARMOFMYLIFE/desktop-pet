import { useCallback, useDeferredValue, useEffect, useMemo, useState } from 'react';
import { api } from '../lib/ipc';
import { errorMessage } from '../lib/errorMessage';
import { t } from '../lib/i18n';
import type { Todo, TodoFilter } from '../types';

interface TodoListOptions {
  notify?: (message: string) => void;
  setError: (message: string) => void;
}

export function useTodoList({ notify, setError }: TodoListOptions) {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [filter, setFilter] = useState<TodoFilter>('all');
  const [query, setQuery] = useState('');
  const [pendingDelete, setPendingDelete] = useState<Todo | null>(null);
  const [loading, setLoading] = useState(true);

  const load = useCallback(() => {
    void api
      .todoList()
      .then(setTodos)
      .catch((reason: unknown) => setError(errorMessage(reason, t('error.generic'))));
  }, [setError]);

  useEffect(() => {
    let cancelled = false;
    void api
      .todoList()
      .then((next) => {
        if (cancelled) return;
        setTodos(next);
        setLoading(false);
      })
      .catch((reason: unknown) => {
        if (cancelled) return;
        setError(errorMessage(reason, t('error.generic')));
        setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [setError]);

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

  const toggle = useCallback(
    async (todo: Todo) => {
      try {
        const next = await api.todoToggle(todo.id, !todo.completed);
        setTodos((current) => current.map((item) => (item.id === next.id ? next : item)));
      } catch (reason) {
        setError(errorMessage(reason, t('error.generic')));
      }
    },
    [setError],
  );

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
    pendingDelete,
    loading,
    visible,
    reload: load,
    toggle,
    requestRemove,
    cancelRemove,
    confirmRemove,
  };
}
