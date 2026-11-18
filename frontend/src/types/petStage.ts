import type { PetState } from './pet';
import type { CharacterStyle, PetSkin } from './settings';

export interface PetPoint {
  x: number;
  y: number;
}

export interface PetStageProps {
  size: number;
  opacity: number;
  reduceMotion: boolean;
  state: PetState;
  characterStyle: CharacterStyle;
  skin: PetSkin;
  onClick: () => void;
  onDoubleClick: () => void;
  onContextMenu: () => void;
  onDragStart: () => void;
  onDragMove: () => void;
  onDragEnd: () => void;
}