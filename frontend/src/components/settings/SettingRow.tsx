import type { ReactNode } from 'react';

interface SettingRowProps {
  title: ReactNode;
  hint?: ReactNode;
  children: ReactNode;
}

export function SettingRow({ title, hint, children }: SettingRowProps) {
  return (
    <div className="row">
      <div className="row-label">
        <strong>{title}</strong>
        {hint && <small>{hint}</small>}
      </div>
      {children}
    </div>
  );
}
