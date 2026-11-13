import { Plus } from 'lucide-react';
import { memo } from 'react';
import { Button } from '../../../../components/ui/Button';
import { SearchField } from '../../../../components/ui/SearchField';
import { t } from '../../../../lib/i18n';
import type { TodoFilter } from '../../../../types';

interface TodoToolbarProps {
  filter: TodoFilter;
  query: string;
  language: string;
  onFilterChange: (filter: TodoFilter) => void;
  onQueryChange: (query: string) => void;
  onNew: () => void;
}

const FILTERS = [
  ['all', 'settings.todos.filterAll'],
  ['today', 'settings.todos.filterToday'],
  ['completed', 'settings.todos.filterCompleted'],
] as const;

export const TodoToolbar = memo(function TodoToolbar({
  filter,
  query,
  language,
  onFilterChange,
  onQueryChange,
  onNew,
}: TodoToolbarProps) {
  return (
    <div className="todo-toolbar-wrap" data-language={language}>
      <div className="todo-toolbar">
        <SearchField
          value={query}
          onChange={onQueryChange}
          label={t('settings.todos.search')}
          placeholder={t('settings.todos.searchPlaceholder')}
          className="todo-search"
        />
        <Button
          variant="primary"
          leadingIcon={<Plus size={14} aria-hidden="true" />}
          onClick={onNew}
        >
          {t('settings.todos.add')}
        </Button>
      </div>
      <div className="segmented" role="group" aria-label={t('settings.todos.title')}>
        {FILTERS.map(([value, key]) => (
          <button
            key={value}
            type="button"
            aria-pressed={filter === value}
            onClick={() => onFilterChange(value)}
          >
            {t(key)}
          </button>
        ))}
      </div>
    </div>
  );
});
