interface DatePeriodItem {
  value: number;
  label: string | number;
}

interface DatePeriodGridProps {
  items: DatePeriodItem[];
  selected: number;
  onSelect: (value: number) => void;
}

export function DatePeriodGrid({ items, selected, onSelect }: DatePeriodGridProps) {
  return (
    <div className="date-periods">
      {items.map((item) => (
        <button
          key={item.value}
          type="button"
          className={`date-period${item.value === selected ? ' is-selected' : ''}`}
          aria-pressed={item.value === selected}
          onClick={() => onSelect(item.value)}
        >
          {item.label}
        </button>
      ))}
    </div>
  );
}
