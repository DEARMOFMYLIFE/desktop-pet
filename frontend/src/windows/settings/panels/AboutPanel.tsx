import { SettingsCard } from '../../../components/settings/SettingsCard';
import { useAppIconSrc } from '../../../hooks/useAppIconSrc';
import { Button } from '../../../components/ui/Button';
import { ConfirmDialog } from '../../../components/ui/ConfirmDialog';
import { useAboutPanel } from '../../../hooks/useAboutPanel';
import { t } from '../../../lib/i18n';
import type { SettingsPanelProps } from '../settingsTypes';

export function AboutPanel({ appInfo, settings }: SettingsPanelProps) {
  const {
    pendingClear,
    openDataFolder,
    requestClearData,
    cancelClearData,
    confirmClearData,
    previewOnboarding,
    quit,
  } = useAboutPanel();
  const iconSrc = useAppIconSrc(settings.appIcon);

  return (
    <>
      <SettingsCard className="about-card">
        <div className="about-head">
          <div className="about-mark">
            <img src={iconSrc} alt="" decoding="async" />
          </div>
          <div>
            <h2>{settings.petName || t('app.name')}</h2>
            <p className="muted">{t('app.tagline')}</p>
          </div>
        </div>
        <div className="about-grid">
          <span>{t('settings.about.version')}</span>
          <span>{appInfo?.version ?? '0.1.0'}</span>
          <span>{t('settings.about.platform')}</span>
          <span>{appInfo?.platform ?? 'browser'}</span>
        </div>
        <p className="privacy">{t('settings.about.privacy')}</p>
      </SettingsCard>
      <SettingsCard title={t('settings.about.licenses')}>
        <p className="muted">{t('settings.about.licensesHint')}</p>
      </SettingsCard>
      <SettingsCard title={t('settings.about.data')}>
        <p className="muted about-data-hint">{t('settings.about.dataHint')}</p>
        <div className="button-row about-data-actions">
          <Button onClick={openDataFolder}>{t('settings.about.openDataFolder')}</Button>
          <Button variant="danger" onClick={requestClearData}>
            {t('settings.about.clearData')}
          </Button>
        </div>
      </SettingsCard>
      <SettingsCard>
        <div className="button-row">
          <Button onClick={previewOnboarding}>
            {t('settings.about.previewOnboarding')}
          </Button>
          <Button variant="danger-outline" onClick={quit}>
            {t('settings.about.quit')}
          </Button>
        </div>
      </SettingsCard>

      <ConfirmDialog
        open={pendingClear}
        title={t('settings.about.clearData')}
        message={t('settings.about.clearConfirm')}
        confirmLabel={t('settings.about.clearData')}
        cancelLabel={t('common.cancel')}
        tone="danger"
        onConfirm={() => void confirmClearData()}
        onCancel={cancelClearData}
      />
    </>
  );
}
