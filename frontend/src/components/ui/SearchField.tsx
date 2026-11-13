import { Search } from 'lucide-react';
import type { InputHTMLAttributes } from 'react';
import { t } from '../../lib/i18n';
import { ClearButton } from './ClearButton';

interface SearchFieldProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'onChange' | 'value'> {
  value: string;
  onChange: (value: string) => void;
  label: string;
}

export function SearchField({
  value,
  onChange,
  label,
  className = '',
  placeholder,
  ...props
}: SearchFieldProps) {
  return (
    <div className={`search-field${className ? ` ${className}` : ''}`} role="search">
      <span className="search-icon-wrap">
        <Search size={15} aria-hidden="true" />
      </span>
      <input
        {...props}
        className="input search-input"
        type="search"
        value={value}
        placeholder={placeholder}
        aria-label={label}
        onChange={(event) => onChange(event.target.value)}
      />
      {value && (
        <ClearButton
          label={t('common.clear')}
          className="search-clear"
          onClick={() => onChange('')}
        />
      )}
    </div>
  );
}
