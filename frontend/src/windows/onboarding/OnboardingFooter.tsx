import { Button } from '../../components/ui/Button';
import { t } from '../../lib/i18n';
import { TOTAL_STEPS } from '../../hooks/useOnboarding';

interface OnboardingFooterProps {
  step: number;
  saving: boolean;
  onBack: () => void;
  onNext: () => void;
  onFinish: () => void;
}

export function OnboardingFooter({ step, saving, onBack, onNext, onFinish }: OnboardingFooterProps) {
  return (
    <footer className="onboarding-foot">
      <div className="onboarding-dots" aria-hidden="true">
        {Array.from({ length: TOTAL_STEPS }, (_, index) => (
          <span key={index} className={index === step ? 'is-active' : ''} />
        ))}
      </div>
      <div className="onboarding-actions">
        <Button variant="ghost" onClick={onFinish}>
          {t('onboarding.skip')}
        </Button>
        {step > 0 && <Button onClick={onBack}>{t('onboarding.back')}</Button>}
        {step < TOTAL_STEPS - 1 ? (
          <Button variant="primary" onClick={onNext}>
            {t('onboarding.next')}
          </Button>
        ) : (
          <Button variant="primary" loading={saving} onClick={onFinish}>
            {t('onboarding.finish')}
          </Button>
        )}
      </div>
    </footer>
  );
}
