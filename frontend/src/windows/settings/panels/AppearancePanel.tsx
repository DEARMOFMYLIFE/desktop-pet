import { useState } from 'react';
import { SettingsCard } from '../../../components/settings/SettingsCard';
import { SettingRow } from '../../../components/settings/SettingRow';
import { Button } from '../../../components/ui/Button';
import { Segmented } from '../../../components/ui/Segmented';
import { Select } from '../../../components/ui/Select';
import { useAppearancePanel } from '../../../hooks/useAppearancePanel';
import { useAppIconSrc } from '../../../hooks/useAppIconSrc';
import { useCustomIcons } from '../../../hooks/useCustomIcons';
import { PET_SKIN_OPTIONS } from '../../../lib/petSkins';
import { t } from '../../../lib/i18n';
import type { SettingsPanelProps } from '../settingsTypes';
import type { AppIcon, CustomIcon } from '../../../types';
import { AppIconDialog } from './appearance/AppIconDialog';

const SKINS = PET_SKIN_OPTIONS;
const BUILTIN_ICON_OPTIONS: Array<{ value: AppIcon; labelKey: string }> = [
  { value: 'clock', labelKey: 'settings.appearance.iconClock' },
  { value: 'star', labelKey: 'settings.appearance.iconStar' },
  { value: 'moon', labelKey: 'settings.appearance.iconMoon' },
  { value: 'coffee', labelKey: 'settings.appearance.iconCoffee' },
  { value: 'sun', labelKey: 'settings.appearance.iconSun' },
  { value: 'leaf', labelKey: 'settings.appearance.iconLeaf' },
  { value: 'heart', labelKey: 'settings.appearance.iconHeart' },
  { value: 'cloud', labelKey: 'settings.appearance.iconCloud' },
  { value: 'book', labelKey: 'settings.appearance.iconBook' },
  { value: 'flower', labelKey: 'settings.appearance.iconFlower' },
];

export function AppearancePanel({ settings, update }: SettingsPanelProps) {
  const { opacity, setOpacity, commitOpacity } = useAppearancePanel(settings, update);
  const { icons: customIcons, importIcon } = useCustomIcons();
  const [dialogOpen, setDialogOpen] = useState(false);
  const iconSrc = useAppIconSrc(settings.appIcon);

  const iconOptions = [
    ...BUILTIN_ICON_OPTIONS.map(({ value, labelKey }) => ({ value, label: t(labelKey) })),
    ...customIcons.map((icon) => ({ value: icon.value, label: icon.name })),
  ];

  const handleImported = (icon: CustomIcon) => {
    void update({ appIcon: icon.value });
  };

  return (
    <>
      <SettingsCard title={t('settings.appearance.skin')}>
        <p className="muted skin-hint">{t('settings.appearance.skinHint')}</p>
        <div className="skin-grid">
          {SKINS.map((skin) => (
            <button
              key={skin.id}
              type="button"
              className={`skin-card${settings.petSkin === skin.id ? ' is-active' : ''}`}
              aria-pressed={settings.petSkin === skin.id}
              onClick={() => void update({ petSkin: skin.id })}
            >
              <img
                src={`/assets/character/${skin.id}/preview.png`}
                alt=""
                loading="lazy"
                decoding="async"
              />
              <span>{t(skin.labelKey)}</span>
            </button>
          ))}
        </div>
      </SettingsCard>
      <SettingsCard title={t('settings.appearance.appIcon')}>
        <p className="muted icon-hint">{t('settings.appearance.appIconHint')}</p>
        <div className="app-icon-row">
          <img className="app-icon-preview" src={iconSrc} alt="" decoding="async" />
          <Select
            value={settings.appIcon}
            options={iconOptions}
            label={t('settings.appearance.appIcon')}
            onChange={(value) => void update({ appIcon: value })}
          />
          <Button onClick={() => setDialogOpen(true)}>
            {t('settings.appearance.addAppIcon')}
          </Button>
        </div>
      </SettingsCard>
      <SettingsCard>
        <SettingRow title={t('settings.appearance.opacity')}>
          <div className="range-wrap">
            <input
              type="range"
              min={0.6}
              max={1}
              step={0.05}
              value={opacity}
              aria-label={t('settings.appearance.opacity')}
              onChange={(event) => setOpacity(Number(event.target.value))}
              onPointerUp={commitOpacity}
              onKeyUp={commitOpacity}
            />
            <span>{Math.round(opacity * 100)}%</span>
          </div>
        </SettingRow>
        <SettingRow title={t('settings.appearance.reduceMotion')}>
          <Segmented
            value={settings.reduceMotion}
            options={[
              { value: 'system', label: t('settings.appearance.reduceSystem') },
              { value: 'on', label: t('settings.appearance.reduceOn') },
              { value: 'off', label: t('settings.appearance.reduceOff') },
            ]}
            label={t('settings.appearance.reduceMotion')}
            onChange={(value) => void update({ reduceMotion: value })}
          />
        </SettingRow>
      </SettingsCard>
      <AppIconDialog
        open={dialogOpen}
        existingNames={customIcons.map((icon) => icon.name)}
        onClose={() => setDialogOpen(false)}
        onImported={handleImported}
        onImport={importIcon}
      />
    </>
  );
}