import type { ReactNode } from 'react';

interface NoticeProps {
  icon?: ReactNode;
  children: ReactNode;
  className?: string;
}

export function Notice({ icon, children, className }: NoticeProps) {
  return (
    <div className={`notice${className ? ` ${className}` : ''}`}>
      {icon && <span className="notice-icon">{icon}</span>}
      <span>{children}</span>
    </div>
  );
}
