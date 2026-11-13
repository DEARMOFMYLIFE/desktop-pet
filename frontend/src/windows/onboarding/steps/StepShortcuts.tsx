import { Keyboard } from 'lucide-react';
import { Notice } from '../../../components/settings/Notice';
import { t } from '../../../lib/i18n';

const SHORTCUTS = [
  ['onboarding.shortcutFocus', 'Ctrl + Alt + P'],
  ['onboarding.shortcutGhost', 'Ctrl + Alt + X'],
  ['onboarding.shortcutVisible', 'Ctrl + Alt + H'],
] as const;

export function StepShortcuts() {
  return (
    <>
      <h1>{t('onboarding.shortcutsTitle')}</h1>
      <p className="onboarding-lead">{t('onboarding.shortcutsHint')}</p>
      <div className="onboarding-card">
        {SHORTCUTS.map(([labelKey, keys]) => (
          <div className="onboarding-shortcut" key={labelKey}>
            <span>{t(labelKey)}</span>
            <kbd>{keys}</kbd>
          </div>
        ))}
      </div>
      <Notice icon={<Keyboard size={14} aria-hidden="true" />}>
        {t('onboarding.shortcutsHint')}
      </Notice>
    </>
  );
}
