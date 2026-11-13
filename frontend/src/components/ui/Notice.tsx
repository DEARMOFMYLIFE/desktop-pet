import type { ReactNode } from 'react';

interface NoticeProps {
  icon?: ReactNode;
  children: ReactNode;
  className?: string;
  role?: 'alert' | 'status';
}

export function Notice({ icon, children, className, role }: NoticeProps) {
  return (
    <div className={`notice${className ? ` ${className}` : ''}`} role={role}>
      {icon && <span className="notice-icon">{icon}</span>}
      <span>{children}</span>
    </div>
  );
}
