import type { ReactNode } from 'react';
import { Card } from '../ui/Card';

interface SettingsCardProps {
  title?: ReactNode;
  description?: ReactNode;
  descriptionClassName?: string;
  className?: string;
  children: ReactNode;
}

export function SettingsCard({
  title,
  description,
  descriptionClassName,
  className,
  children,
}: SettingsCardProps) {
  return (
    <Card className={className}>
      {title && <h2 className="card-title">{title}</h2>}
      {description && (
        <p className={`muted${descriptionClassName ? ` ${descriptionClassName}` : ''}`}>
          {description}
        </p>
      )}
      {children}
    </Card>
  );
}
