import { Button } from '../../../components/ui/Button';
import { Card } from '../../../components/ui/Card';
import { Segmented } from '../../../components/ui/Segmented';
import { Switch } from '../../../components/ui/Switch';
import { t } from '../../../lib/i18n';
import type { Language, Settings } from '../../../types';

interface StepPreferencesProps {
  settings: Settings;
  permission: string;
  onLanguageChange: (value: Language) => void;
  onAutostartChange: (checked: boolean) => void;
  onRequestPermission: () => void;
}

export function StepPreferences({
  settings,
  permission,
  onLanguageChange,
  onAutostartChange,
  onRequestPermission,
}: StepPreferencesProps) {
  return (
    <>
      <h1>{t('onboarding.preferencesTitle')}</h1>
      <Card className="onboarding-card">
        <div className="onboarding-row">
          <span>{t('onboarding.language')}</span>
          <Segmented
            value={settings.language}
            options={[
              { value: 'system', label: t('settings.general.languageSystem') },
              { value: 'zh-CN', label: t('settings.general.languageZh') },
              { value: 'en-US', label: t('settings.general.languageEn') },
            ]}
            label={t('onboarding.language')}
            onChange={onLanguageChange}
          />
        </div>
        <div className="onboarding-row">
          <span>
            {t('onboarding.autostart')}
            <small>{t('onboarding.autostartHint')}</small>
          </span>
          <Switch
            checked={settings.autostart}
            label={t('onboarding.autostart')}
            onChange={onAutostartChange}
          />
        </div>
        <div className="onboarding-row">
          <span>
            {t('onboarding.notifications')}
            <small>{t('onboarding.notificationsHint')}</small>
          </span>
          <Button disabled={permission === 'granted'} onClick={onRequestPermission}>
            {permission === 'granted'
              ? t('onboarding.notificationsGranted')
              : permission === 'denied'
                ? t('onboarding.notificationsDenied')
                : t('onboarding.allow')}
          </Button>
        </div>
      </Card>
    </>
  );
}
