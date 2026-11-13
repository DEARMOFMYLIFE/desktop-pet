import { ErrorPane } from '../../components/ui/ErrorPane';
import { LoadingPane } from '../../components/ui/LoadingPane';
import { useOnboarding } from '../../hooks/useOnboarding';
import { OnboardingFooter } from './OnboardingFooter';
import { OnboardingHeader } from './OnboardingHeader';
import { StepPreferences } from './steps/StepPreferences';
import { StepShortcuts } from './steps/StepShortcuts';
import { StepWelcome } from './steps/StepWelcome';
import './onboarding.css';

export function OnboardingWindow() {
  const onboarding = useOnboarding();

  if (onboarding.loadError) {
    return <ErrorPane message={onboarding.loadError} />;
  }

  if (!onboarding.settings) {
    return <LoadingPane />;
  }

  return (
    <div className="onboarding-shell app-shell">
      <OnboardingHeader
        step={onboarding.step}
        appIcon={onboarding.settings.appIcon}
        petName={onboarding.settings.petName}
      />

      <main className="onboarding-main">
        {onboarding.step === 0 && (
          <StepWelcome
            petName={onboarding.settings.petName}
            skin={onboarding.settings.petSkin}
            onNameChange={(petName) => onboarding.patchSettings({ petName })}
          />
        )}
        {onboarding.step === 1 && (
          <StepPreferences
            settings={onboarding.settings}
            permission={onboarding.permission}
            onLanguageChange={onboarding.setLanguage}
            onAutostartChange={(autostart) => onboarding.patchSettings({ autostart })}
            onRequestPermission={onboarding.requestPermission}
          />
        )}
        {onboarding.step === 2 && <StepShortcuts />}
      </main>

      <OnboardingFooter
        step={onboarding.step}
        saving={onboarding.saving}
        onBack={onboarding.back}
        onNext={onboarding.next}
        onFinish={() => void onboarding.finish()}
      />
    </div>
  );
}
