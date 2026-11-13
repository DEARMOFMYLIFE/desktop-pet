import { ClearButton } from '../../../components/ui/ClearButton';
import { Input } from '../../../components/ui/Input';
import { PetStage } from '../../pet/PetStage';
import { t } from '../../../lib/i18n';
import type { PetSkin } from '../../../types';

interface StepWelcomeProps {
  petName: string;
  skin: PetSkin;
  onNameChange: (value: string) => void;
}

export function StepWelcome({ petName, skin, onNameChange }: StepWelcomeProps) {
  return (
    <>
      <div className="onboarding-mascot">
        <PetStage
          size={132}
          opacity={1}
          reduceMotion={false}
          state="idle"
          skin={skin}
          onClick={() => undefined}
          onDoubleClick={() => undefined}
          onContextMenu={() => undefined}
          onDragStart={() => undefined}
          onDragMove={() => undefined}
          onDragEnd={() => undefined}
        />
      </div>
      <h1>{t('onboarding.welcomeTitle', { name: petName || t('app.name') })}</h1>
      <p className="onboarding-lead">{t('onboarding.welcomeBody')}</p>
      <label className="onboarding-field">
        <span>{t('onboarding.nameLabel')}</span>
        <div className="clearable-field">
          <Input
            maxLength={20}
            value={petName}
            placeholder={t('onboarding.namePlaceholder')}
            onChange={(event) => onNameChange(event.target.value)}
          />
          {petName && <ClearButton onClick={() => onNameChange('')} />}
        </div>
      </label>
    </>
  );
}
