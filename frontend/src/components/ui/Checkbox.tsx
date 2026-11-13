import { Check } from 'lucide-react';

interface CheckboxProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  label?: string;
  ariaLabel?: string;
  disabled?: boolean;
}

export function Checkbox({
  checked,
  onChange,
  label,
  ariaLabel,
  disabled = false,
}: CheckboxProps) {
  return (
    <label className={`checkbox${disabled ? ' is-disabled' : ''}`}>
      <input
        type="checkbox"
        className="peer sr-only"
        checked={checked}
        disabled={disabled}
        aria-label={ariaLabel}
        onChange={(event) => onChange(event.target.checked)}
      />
      <span className="checkbox-box">
        <Check size={12} strokeWidth={3} aria-hidden="true" />
      </span>
      {label && <span className="checkbox-label">{label}</span>}
    </label>
  );
}
