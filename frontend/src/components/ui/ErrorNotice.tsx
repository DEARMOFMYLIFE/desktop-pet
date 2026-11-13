import { AlertTriangle } from 'lucide-react';
import type { ReactNode } from 'react';
import { Notice } from './Notice';

interface ErrorNoticeProps {
  children: ReactNode;
  className?: string;
}

// Shared error banner so panels do not repeat the alert icon and error class.
export function ErrorNotice({ children, className }: ErrorNoticeProps) {
  return (
    <Notice
      className={`notice-error${className ? ` ${className}` : ''}`}
      icon={<AlertTriangle size={14} aria-hidden="true" />}
      role="alert"
    >
      {children}
    </Notice>
  );
}
