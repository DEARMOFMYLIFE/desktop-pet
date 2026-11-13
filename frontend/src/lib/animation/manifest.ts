import type { PetState } from '../../types';

export interface SpriteStateConfig {
  file: string;
  fallback?: string;
  frames: number;
  fps: number;
  loop: boolean;
}

export interface SpriteManifest {
  character: string;
  version: number;
  frameSize: [number, number];
  anchor: [number, number];
  hitbox: [number, number, number, number];
  defaultState: PetState;
  states: Record<PetState, SpriteStateConfig>;
}
