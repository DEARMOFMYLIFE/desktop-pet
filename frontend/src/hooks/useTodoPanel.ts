import { useState } from 'react';
import { useTodoForm } from './useTodoForm';
import { useTodoList } from './useTodoList';

export function useTodoPanel(notify?: (message: string) => void) {
  const [error, setError] = useState('');
  const list = useTodoList({ notify, setError });
  const form = useTodoForm({ notify, setError, reload: list.reload });

  return {
    ...list,
    ...form,
    error,
  };
}
