import { Card } from '../../../components/ui/Card';
import { formatShortcut } from '../../../lib/format';
import { t } from '../../../lib/i18n';
import type { ShortcutSettings } from '../../../types';

const SHORTCUT_LABELS = [
  ['onboarding.shortcutFocus', 'focus'],
  ['onboarding.shortcutGhost', 'clickThrough'],
  ['onboarding.shortcutVisible', 'toggleVisible'],
] as const;

interface StepShortcutsProps {
  shortcuts: ShortcutSettings;
}

export function StepShortcuts({ shortcuts }: StepShortcutsProps) {
  return (
    <>
      <h1>{t('onboarding.shortcutsTitle')}</h1>
      <p className="onboarding-lead">{t('onboarding.shortcutsHint')}</p>
      <Card className="onboarding-card">
        {SHORTCUT_LABELS.map(([labelKey, action]) => (
          <div className="onboarding-shortcut" key={labelKey}>
            <span>{t(labelKey)}</span>
            <kbd>{formatShortcut(shortcuts[action])}</kbd>
          </div>
        ))}
      </Card>
    </>
  );
}
