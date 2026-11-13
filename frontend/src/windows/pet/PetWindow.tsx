import { Lock } from 'lucide-react';
import { ErrorPane } from '../../components/ui/ErrorPane';
import { LoadingPane } from '../../components/ui/LoadingPane';
import { usePetWindow } from '../../hooks/usePetWindow';
import { t } from '../../lib/i18n';
import { PetStage } from './PetStage';
import './pet.css';

export function PetWindow() {
  const {
    settings,
    petState,
    hint,
    showHint,
    reduceMotion,
    opacity,
    size,
    handleClick,
    handleDoubleClick,
    handleContextMenu,
    handleDragStart,
    handleDragMove,
    handleDragEnd,
    loadError,
  } = usePetWindow();

  if (loadError) {
    return (
      <div className="pet-root">
        <ErrorPane transparent message={loadError} />
      </div>
    );
  }

  if (!settings) {
    return (
      <div className="pet-root">
        <LoadingPane transparent />
      </div>
    );
  }

  return (
    <div className={`pet-root${settings.clickThrough ? ' is-ghost' : ''}`}>
      <PetStage
        size={size}
        opacity={opacity}
        reduceMotion={reduceMotion}
        state={petState}
        skin={settings.petSkin}
        onClick={handleClick}
        onDoubleClick={handleDoubleClick}
        onContextMenu={handleContextMenu}
        onDragStart={handleDragStart}
        onDragMove={handleDragMove}
        onDragEnd={handleDragEnd}
      />
      {showHint && <div className="pet-hint">{hint}</div>}
      {settings.clickThrough && (
        <div className="pet-badge" aria-label={t('pet.clickThroughOn')}>
          <Lock aria-hidden="true" size={11} strokeWidth={2.2} />
        </div>
      )}
    </div>
  );
}
