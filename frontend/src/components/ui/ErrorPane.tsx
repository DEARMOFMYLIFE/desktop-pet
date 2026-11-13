import { AlertTriangle } from 'lucide-react';
import { t } from '../../lib/i18n';

interface ErrorPaneProps {
  message?: string;
  transparent?: boolean;
}

export function ErrorPane({ message, transparent = false }: ErrorPaneProps) {
  return (
    <div
      className={`flex h-full w-full flex-col items-center justify-center gap-2 p-6 text-center ${
        transparent ? 'bg-transparent' : 'bg-bg'
      }`}
      role="alert"
    >
      <AlertTriangle size={20} className="text-danger" aria-hidden="true" />
      <p className="m-0 max-w-xs text-xs leading-relaxed text-muted">
        {message ?? t('error.generic')}
      </p>
    </div>
  );
}