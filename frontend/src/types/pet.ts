export type PetState =
  | 'idle'
  | 'walk'
  | 'sit'
  | 'sleep'
  | 'focus'
  | 'break'
  | 'celebrate'
  | 'drag'
  | 'react'
  | 'remind'
  | 'droop';

export interface PetSnapshot {
  state: PetState;
  mood: number;
  energy: number;
}
