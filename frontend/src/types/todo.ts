export type TodoFilter = 'all' | 'today' | 'completed';

export interface Todo {
  id: number;
  title: string;
  note: string;
  dueAt: string | null;
  remindAt: string | null;
  remindedAt: string | null;
  completed: boolean;
  createdAt: string;
  completedAt: string | null;
  sortOrder: number;
}

export interface TodoInput {
  title: string;
  note?: string;
  dueAt?: string | null;
  remindAt?: string | null;
}
