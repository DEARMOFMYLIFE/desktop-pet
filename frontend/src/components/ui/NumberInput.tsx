import { Minus, Plus } from 'lucide-react';
import { useId } from 'react';

interface NumberInputProps {
  value: number;
  onChange: (value: number) => void;
  min?: number;
  max?: number;
  step?: number;
  label: string;
  unit?: string;
  compact?: boolean;
  disabled?: boolean;
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(max, Math.max(min, value));
}

export function NumberInput({
  value,
  onChange,
  min = 0,
  max = Number.MAX_SAFE_INTEGER,
  step = 1,
  label,
  unit,
  compact = false,
  disabled = false,
}: NumberInputProps) {
  const id = useId();
  const maxLength = String(Math.max(max, 0)).length;
  const update = (next: number) => {
    if (!Number.isFinite(next)) return;
    onChange(clamp(Math.round(next), min, max));
  };

  return (
    <div className={`number-control${compact ? ' number-control-compact' : ''}`}>
      <button
        type="button"
        className="number-step"
        aria-label={`${label} -`}
        disabled={disabled || value <= min}
        onClick={() => update(value - step)}
      >
        <Minus size={12} aria-hidden="true" />
      </button>
      <input
        id={id}
        className="number-control-input"
        type="text"
        inputMode="numeric"
        pattern="[0-9]*"
        autoComplete="off"
        maxLength={maxLength}
        value={value}
        disabled={disabled}
        aria-label={label}
        onInput={(event) => {
          const cleaned = event.currentTarget.value
            .replace(/[^0-9]/g, '')
            .slice(0, maxLength);
          if (cleaned !== event.currentTarget.value) {
            event.currentTarget.value = cleaned;
          }
        }}
        onChange={(event) => {
          const cleaned = event.currentTarget.value
            .replace(/[^0-9]/g, '')
            .slice(0, maxLength);
          if (cleaned !== '') update(Number(cleaned));
        }}
      />
      {unit && <span className="number-control-unit">{unit}</span>}
      <button
        type="button"
        className="number-step"
        aria-label={`${label} +`}
        disabled={disabled || value >= max}
        onClick={() => update(value + step)}
      >
        <Plus size={12} aria-hidden="true" />
      </button>
    </div>
  );
}