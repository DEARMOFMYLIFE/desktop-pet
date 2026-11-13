import { Sparkles } from 'lucide-react';
import { Notice } from '../../../components/ui/Notice';
import { SettingsCard } from '../../../components/settings/SettingsCard';
import { SettingRow } from '../../../components/settings/SettingRow';
import { Button } from '../../../components/ui/Button';
import { Segmented } from '../../../components/ui/Segmented';
import { Switch } from '../../../components/ui/Switch';
import { useGeneralPanel } from '../../../hooks/useGeneralPanel';
import { t } from '../../../lib/i18n';
import type { Language } from '../../../types';
import { SIZES, type SettingsPanelProps } from '../settingsTypes';

export function GeneralPanel({ settings, update }: SettingsPanelProps) {
  const { openBubble, openOnboarding } = useGeneralPanel();
  const sizeOptions = SIZES.map((size) => ({ value: size.value, label: t(size.labelKey) }));
  const languageOptions: Array<{ value: Language; label: string }> = [
    { value: 'system', label: t('settings.general.languageSystem') },
    { value: 'zh-CN', label: t('settings.general.languageZh') },
    { value: 'en-US', label: t('settings.general.languageEn') },
  ];

  return (
    <>
      {settings.firstRun && (
        <Notice icon={<Sparkles size={14} aria-hidden="true" />}>
          <strong>{t('settings.general.firstRun')}</strong>
          <br />
          {t('settings.general.firstRunHint')}
        </Notice>
      )}

      <SettingsCard title={t('settings.general.languageSection')}>
        <SettingRow
          title={t('settings.general.language')}
          hint={t('settings.general.languageHint')}
        >
          <Segmented
            value={settings.language}
            options={languageOptions}
            label={t('settings.general.language')}
            onChange={(value) => void update({ language: value })}
          />
        </SettingRow>
      </SettingsCard>

      <SettingsCard title={t('settings.general.windowSection')}>
        <SettingRow title={t('settings.general.alwaysOnTop')}>
          <Switch
            checked={settings.alwaysOnTop}
            label={t('settings.general.alwaysOnTop')}
            onChange={(checked) => void update({ alwaysOnTop: checked })}
          />
        </SettingRow>
        <SettingRow
          title={t('settings.general.clickThrough')}
          hint={t('settings.general.clickThroughHint')}
        >
          <Switch
            checked={settings.clickThrough}
            label={t('settings.general.clickThrough')}
            onChange={(checked) => void update({ clickThrough: checked })}
          />
        </SettingRow>
        <SettingRow title={t('settings.general.petSize')}>
          <Segmented
            value={settings.petSize}
            options={sizeOptions}
            label={t('settings.general.petSize')}
            onChange={(value) => void update({ petSize: value })}
          />
        </SettingRow>
        <SettingRow
          title={t('settings.general.resetPosition')}
          hint={t('settings.general.resetPositionHint')}
        >
          <Button disabled>{t('settings.general.resetPosition')}</Button>
        </SettingRow>
      </SettingsCard>

      <SettingsCard title={t('settings.general.quickSection')}>
        <div className="button-row">
          <Button variant="primary" onClick={openBubble}>
            {t('settings.general.openBubble')}
          </Button>
          <Button onClick={openOnboarding}>{t('settings.general.openOnboarding')}</Button>
        </div>
      </SettingsCard>
    </>
  );
}
