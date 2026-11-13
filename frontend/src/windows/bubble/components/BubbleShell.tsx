import { X } from 'lucide-react';
import type { PointerEvent as ReactPointerEvent, ReactNode } from 'react';
import { api } from '../../../lib/ipc';
import { IconButton } from '../../../components/ui/IconButton';
import { t } from '../../../lib/i18n';

interface BubbleShellProps {
  label: string;
  onClose: () => void;
  className?: string;
  children: ReactNode;
}

export function BubbleShell({ label, onClose, className, children }: BubbleShellProps) {
  const startDrag = (event: ReactPointerEvent<HTMLDivElement>) => {
    if (event.button !== 0) return;
    if ((event.target as HTMLElement).closest('button, a, input, select, textarea')) return;
    void api.startWindowDrag();
  };

  return (
    <div className="bubble-root">
      <div className={`bubble-card${className ? ` ${className}` : ''}`} onPointerDown={startDrag}>
        <div className="bubble-head">
          <span className="bubble-label">{label}</span>
          <IconButton label={t('bubble.close')} className="bubble-close" onClick={onClose}>
            <X size={15} aria-hidden="true" />
          </IconButton>
        </div>
        {children}
      </div>
    </div>
  );
}
