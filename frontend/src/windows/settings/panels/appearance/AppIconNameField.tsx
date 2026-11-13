import { ClearButton } from '../../../../components/ui/ClearButton';
import { Input } from '../../../../components/ui/Input';
import { t } from '../../../../lib/i18n';

interface AppIconNameFieldProps {
  name: string;
  onChange: (value: string) => void;
  onClear: () => void;
}

export function AppIconNameField({ name, onChange, onClear }: AppIconNameFieldProps) {
  return (
    <label className="icon-name-field">
      <span>{t('settings.appearance.iconName')}</span>
      <div className="clearable-field">
        <Input
          autoFocus
          maxLength={32}
          value={name}
          placeholder={t('settings.appearance.iconNamePlaceholder')}
          onChange={(event) => onChange(event.target.value)}
        />
        {name && <ClearButton onClick={onClear} />}
      </div>
    </label>
  );
}
