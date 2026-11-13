import type { PetSnapshot } from '../../../types';

let mockPet: PetSnapshot = { state: 'idle', mood: 70, energy: 80 };
let resetTimer: number | null = null;

export function currentPet(): PetSnapshot {
  return mockPet;
}

function setMockPet(state: PetSnapshot['state']): PetSnapshot {
  mockPet = { ...mockPet, state };
  return mockPet;
}

export function handlePet<T>(
  cmd: string,
  _args?: Record<string, unknown>,
): Promise<T> | null {
  switch (cmd) {
    case 'pet_snapshot':
      return Promise.resolve(mockPet as T);
    case 'pet_click': {
      const snapshot = { ...setMockPet('react'), mood: Math.min(100, mockPet.mood + 1) };
      mockPet = snapshot;
      if (resetTimer !== null) window.clearTimeout(resetTimer);
      resetTimer = window.setTimeout(() => {
        resetTimer = null;
        setMockPet('idle');
      }, 1200);
      return Promise.resolve(snapshot as T);
    }
    case 'pet_drag_start':
      return Promise.resolve(setMockPet('drag') as T);
    case 'pet_drag_move':
      return Promise.resolve(mockPet as T);
    case 'pet_drag_end':
      return Promise.resolve(setMockPet('idle') as T);
    default:
      return null;
  }
}
