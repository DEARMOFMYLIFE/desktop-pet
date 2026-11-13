import { Keyboard } from 'lucide-react';
import { Notice } from '../../../components/settings/Notice';
import { Button } from '../../../components/ui/Button';
import { SettingsCard } from '../../../components/settings/SettingsCard';
import { useShortcutsPanel } from '../../../hooks/useShortcutsPanel';
import { t } from '../../../lib/i18n';
import type { SettingsPanelProps } from '../settingsTypes';
import { ShortcutRow } from './shortcuts/ShortcutRow';

export function ShortcutsPanel({ settings }: SettingsPanelProps) {
  const shortcuts = useShortcutsPanel(settings);

  return (
    <>
      <Notice icon={<Keyboard size={14} aria-hidden="true" />}>
        {t('settings.shortcuts.hint')}
      </Notice>
      <SettingsCard>
        <ShortcutRow
          label={t('settings.shortcuts.focus')}
          value={shortcuts.focus}
          onChange={(value) =>
            shortcuts.updateShortcut('focus', value, shortcuts.setFocus)
          }
          onError={shortcuts.setError}
        />
        <ShortcutRow
          label={t('settings.shortcuts.clickThrough')}
          value={shortcuts.clickThrough}
          onChange={(value) =>
            shortcuts.updateShortcut('click_through', value, shortcuts.setClickThrough)
          }
          onError={shortcuts.setError}
        />
        <ShortcutRow
          label={t('settings.shortcuts.toggleVisible')}
          value={shortcuts.toggleVisible}
          onChange={(value) =>
            shortcuts.updateShortcut('toggle_visible', value, shortcuts.setToggleVisible)
          }
          onError={shortcuts.setError}
        />
      </SettingsCard>
      {shortcuts.error && (
        <Notice className="notice-error">{shortcuts.error}</Notice>
      )}
      <SettingsCard>
        <Button onClick={shortcuts.resetShortcuts}>
          {t('settings.shortcuts.reset')}
        </Button>
      </SettingsCard>
    </>
  );
}
