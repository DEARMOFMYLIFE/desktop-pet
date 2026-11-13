import { memo } from 'react';

interface SettingsHeaderProps {
  title: string;
  description: string;
}

export const SettingsHeader = memo(function SettingsHeader({
  title,
  description,
}: SettingsHeaderProps) {
  return (
    <header className="settings-header">
      <h1>{title}</h1>
      <p>{description}</p>
    </header>
  );
});
