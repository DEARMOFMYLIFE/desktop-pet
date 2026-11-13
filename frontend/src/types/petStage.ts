import type { PetState } from './pet';
import type { PetSkin } from './settings';

export interface PetPoint {
  x: number;
  y: number;
}

export interface PetStageProps {
  size: number;
  opacity: number;
  reduceMotion: boolean;
  state: PetState;
  skin: PetSkin;
  onClick: () => void;
  onDoubleClick: () => void;
  onContextMenu: () => void;
  onDragStart: () => void;
  onDragMove: () => void;
  onDragEnd: () => void;
}