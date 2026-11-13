import { AlertTriangle } from 'lucide-react';
import { t } from '../../lib/i18n';

interface ErrorPaneProps {
  message?: string;
  transparent?: boolean;
}

export function ErrorPane({ message, transparent = false }: ErrorPaneProps) {
  return (
    <div
      className={`pane pane-error${transparent ? ' pane-transparent' : ''}`}
      role="alert"
    >
      <AlertTriangle size={20} className="text-danger" aria-hidden="true" />
      <p>{message ?? t('error.generic')}</p>
    </div>
  );
}