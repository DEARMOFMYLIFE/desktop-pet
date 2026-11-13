import { Loader2 } from 'lucide-react';
import { t } from '../../lib/i18n';

interface LoadingPaneProps {
  transparent?: boolean;
}

export function LoadingPane({ transparent = false }: LoadingPaneProps) {
  return (
    <div
      className={`flex h-full w-full items-center justify-center ${transparent ? 'bg-transparent' : 'bg-bg'}`}
      aria-busy="true"
    >
      <Loader2 className="animate-spin text-muted" size={18} aria-hidden="true" />
      <span className="sr-only">{t('common.loading')}</span>
    </div>
  );
}
