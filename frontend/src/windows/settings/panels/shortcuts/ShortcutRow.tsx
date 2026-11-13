import { ShortcutRecorder } from '../../../../components/ui/ShortcutRecorder';
import { t } from '../../../../lib/i18n';

interface ShortcutRowProps {
  label: string;
  value: string;
  onChange: (value: string) => void;
  onError: (message: string) => void;
}

export function ShortcutRow({ label, value, onChange, onError }: ShortcutRowProps) {
  return (
    <div className="shortcut-row">
      <span>{label}</span>
      <ShortcutRecorder value={value} label={label} onChange={onChange} onError={onError} />
      <span className="badge">{t('settings.shortcuts.enabled')}</span>
    </div>
  );
}
