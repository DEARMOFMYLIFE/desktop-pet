import { Eraser } from 'lucide-react';
import { t } from '../../lib/i18n';
import { IconButton } from './IconButton';

interface ClearButtonProps {
  onClick: () => void;
  className?: string;
  label?: string;
}

export function ClearButton({ onClick, className = '', label }: ClearButtonProps) {
  return (
    <IconButton
      label={label ?? t('common.clear')}
      className={`clear-btn${className ? ` ${className}` : ''}`}
      onClick={onClick}
    >
      <Eraser size={15} strokeWidth={1.8} aria-hidden="true" />
    </IconButton>
  );
}