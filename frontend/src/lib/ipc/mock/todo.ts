import type { Todo, TodoInput } from '../../../types';

let mockTodos: Todo[] = [
  {
    id: 1,
    title: '给客户回邮件',
    note: '',
    dueAt: null,
    remindAt: null,
    remindedAt: null,
    completed: false,
    createdAt: new Date().toISOString(),
    completedAt: null,
    sortOrder: 1024,
  },
  {
    id: 2,
    title: '买咖啡豆',
    note: '',
    dueAt: null,
    remindAt: null,
    remindedAt: null,
    completed: false,
    createdAt: new Date().toISOString(),
    completedAt: null,
    sortOrder: 2048,
  },
];
let nextTodoId = 3;

function mockTodo(input: TodoInput): Todo {
  const todo: Todo = {
    id: nextTodoId++,
    title: input.title,
    note: input.note ?? '',
    dueAt: input.dueAt ?? null,
    remindAt: input.remindAt ?? null,
    remindedAt: null,
    completed: false,
    createdAt: new Date().toISOString(),
    completedAt: null,
    sortOrder: (mockTodos.length + 1) * 1024,
  };
  mockTodos = [...mockTodos, todo];
  return todo;
}

export function handleTodo<T>(
  cmd: string,
  args?: Record<string, unknown>,
): Promise<T> | null {
  switch (cmd) {
    case 'todo_list':
      return Promise.resolve(mockTodos as T);
    case 'todo_create':
      return Promise.resolve(mockTodo(args?.input as TodoInput) as T);
    case 'todo_update': {
      const id = Number(args?.id);
      const input = args?.input as TodoInput;
      mockTodos = mockTodos.map((todo) =>
        todo.id === id
          ? {
              ...todo,
              title: input.title,
              note: input.note ?? '',
              dueAt: input.dueAt ?? null,
              remindAt: input.remindAt ?? null,
              remindedAt: null,
            }
          : todo,
      );
      return Promise.resolve(mockTodos.find((todo) => todo.id === id) as T);
    }
    case 'todo_toggle': {
      const id = Number(args?.id);
      const completed = Boolean(args?.completed);
      mockTodos = mockTodos.map((todo) =>
        todo.id === id
          ? {
              ...todo,
              completed,
              completedAt: completed ? new Date().toISOString() : null,
            }
          : todo,
      );
      return Promise.resolve(mockTodos.find((todo) => todo.id === id) as T);
    }
    case 'todo_delete': {
      const id = Number(args?.id);
      mockTodos = mockTodos.filter((todo) => todo.id !== id);
      return Promise.resolve(undefined as T);
    }
    case 'todo_reorder': {
      const ids = (args?.ids as number[]) ?? [];
      mockTodos = [...mockTodos].sort((a, b) => ids.indexOf(a.id) - ids.indexOf(b.id));
      return Promise.resolve(undefined as T);
    }
    default:
      return null;
  }
}
