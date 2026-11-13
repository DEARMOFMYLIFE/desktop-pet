import { useAppIconSrc } from '../../hooks/useAppIconSrc';
import { t } from '../../lib/i18n';
import type { AppIcon } from '../../types';

export function OnboardingHeader({
  step,
  appIcon,
  petName,
}: {
  step: number;
  appIcon: AppIcon;
  petName: string;
}) {
  const iconSrc = useAppIconSrc(appIcon);

  return (
    <header className="onboarding-head">
      <span className="onboarding-brand">
        <img className="onboarding-logo" src={iconSrc} alt="" decoding="async" />
        {petName || t('app.name')}
      </span>
      <span className="onboarding-step">{t('onboarding.step', { n: step + 1 })}</span>
    </header>
  );
}
