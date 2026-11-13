import { X } from 'lucide-react';
import type { ReactNode } from 'react';
import { useEffect, useId } from 'react';
import { createPortal } from 'react-dom';
import { t } from '../../lib/i18n';
import { IconButton } from './IconButton';

interface ModalProps {
  open: boolean;
  title: string;
  description?: string;
  onClose: () => void;
  children?: ReactNode;
  footer?: ReactNode;
  icon?: ReactNode;
  className?: string;
}

export function Modal({
  open,
  title,
  description,
  onClose,
  children,
  footer,
  icon,
  className,
}: ModalProps) {
  const titleId = useId();

  useEffect(() => {
    if (!open) return;
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') onClose();
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
  }, [open, onClose]);

  if (!open) return null;

  return createPortal(
    <div className="dialog-overlay" onMouseDown={onClose}>
      <div
        className={`dialog-panel${className ? ` ${className}` : ''}`}
        role="dialog"
        aria-modal="true"
        aria-labelledby={titleId}
        onMouseDown={(event) => event.stopPropagation()}
      >
        <header className="dialog-head">
          <div className="dialog-heading">
            {icon && <span className="dialog-icon">{icon}</span>}
            <div className="min-w-0">
              <h2 id={titleId} className="dialog-title">
                {title}
              </h2>
              {description && <p className="dialog-description">{description}</p>}
            </div>
          </div>
          <IconButton
            label={t('common.close')}
            className="dialog-close"
            onClick={onClose}
          >
            <X size={16} aria-hidden="true" />
          </IconButton>
        </header>
        {children != null && <div className="dialog-body">{children}</div>}
        {footer && <footer className="dialog-foot">{footer}</footer>}
      </div>
    </div>,
    document.body,
  );
}