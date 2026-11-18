import { useState } from 'react';
import { SettingsCard } from '../../../components/settings/SettingsCard';
import { SettingRow } from '../../../components/settings/SettingRow';
import { AppIconImage } from '../../../components/ui/AppIconImage';
import { Button } from '../../../components/ui/Button';
import { Segmented } from '../../../components/ui/Segmented';
import { Select } from '../../../components/ui/Select';
import { useAppearancePanel } from '../../../hooks/useAppearancePanel';
import { useAppIconSrc } from '../../../hooks/useAppIconSrc';
import { useCustomIcons } from '../../../hooks/useCustomIcons';
import {
  CHARACTER_STYLE_OPTIONS,
  defaultSkinForStyle,
  isSkinForStyle,
  skinsForStyle,
} from '../../../lib/characterStyles';
import { characterPreviewUrl } from '../../../lib/character/api';
import { t } from '../../../lib/i18n';
import type { SettingsPanelProps } from '../settingsTypes';
import type { AppIcon, CustomIcon } from '../../../types';
import { AppIconDialog } from './appearance/AppIconDialog';

const BUILTIN_ICON_OPTIONS: Array<{ value: AppIcon; labelKey: string }> = [
  { value: 'heart', labelKey: 'settings.appearance.iconHeart' },
  { value: 'clock', labelKey: 'settings.appearance.iconClock' },
  { value: 'star', labelKey: 'settings.appearance.iconStar' },
  { value: 'moon', labelKey: 'settings.appearance.iconMoon' },
  { value: 'coffee', labelKey: 'settings.appearance.iconCoffee' },
  { value: 'sun', labelKey: 'settings.appearance.iconSun' },
  { value: 'leaf', labelKey: 'settings.appearance.iconLeaf' },
  { value: 'cloud', labelKey: 'settings.appearance.iconCloud' },
  { value: 'book', labelKey: 'settings.appearance.iconBook' },
  { value: 'flower', labelKey: 'settings.appearance.iconFlower' },
];

export function AppearancePanel({ settings, update }: SettingsPanelProps) {
  const { opacity, setOpacity, commitOpacity } = useAppearancePanel(settings, update);
  const { icons: customIcons, importIcon } = useCustomIcons();
  const [dialogOpen, setDialogOpen] = useState(false);
  const iconSrc = useAppIconSrc(settings.appIcon);
  const skins = skinsForStyle(settings.characterStyle);

  const iconOptions = [
    ...BUILTIN_ICON_OPTIONS.map(({ value, labelKey }) => ({ value, label: t(labelKey) })),
    ...customIcons.map((icon) => ({ value: icon.value, label: icon.name })),
  ];

  const handleImported = (icon: CustomIcon) => {
    void update({ appIcon: icon.value });
  };

  return (
    <>
      <SettingsCard
        title={t('settings.appearance.style')}
        description={t('settings.appearance.styleHint')}
      >
        <Select
          className="style-select"
          value={settings.characterStyle}
          options={CHARACTER_STYLE_OPTIONS.map((option) => ({
            value: option.value,
            label: t(option.labelKey),
          }))}
          label={t('settings.appearance.style')}
          onChange={(value) => {
            const petSkin = isSkinForStyle(value, settings.petSkin)
              ? settings.petSkin
              : defaultSkinForStyle(value);
            void update({ characterStyle: value, petSkin });
          }}
        />
      </SettingsCard>
      <SettingsCard
        title={t('settings.appearance.skin')}
        description={t('settings.appearance.skinHint')}
      >
        <div className="skin-grid">
          {skins.map((skin) => (
            <button
              key={skin.id}
              type="button"
              className={`skin-card${settings.petSkin === skin.id ? ' is-active' : ''}`}
              aria-pressed={settings.petSkin === skin.id}
              onClick={() => void update({ petSkin: skin.id })}
            >
              <img
                src={characterPreviewUrl(settings.characterStyle, skin.id)}
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
        <p className="card-description">{t('settings.appearance.appIconHint')}</p>
        <div className="app-icon-row">
          <AppIconImage className="app-icon-preview" src={iconSrc} />
          <Select
            value={settings.appIcon}
            options={iconOptions}
            label={t('settings.appearance.appIcon')}
            onChange={(value) => void update({ appIcon: value })}
          />
          <Button className="btn-control" onClick={() => setDialogOpen(true)}>
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
