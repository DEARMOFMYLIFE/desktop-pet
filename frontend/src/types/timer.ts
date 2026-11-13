export type TimerKind = 'pomodoro' | 'countdown' | 'countup';
export type TimerPhase = 'focus' | 'short_break' | 'long_break';
export type TimerStatus = 'idle' | 'running' | 'paused';

export type TimerStartRequest =
  | { kind: 'pomodoro' }
  | { kind: 'countdown'; seconds: number }
  | { kind: 'countup' };

export interface TimerSnapshot {
  kind: TimerKind;
  phase: TimerPhase | null;
  status: TimerStatus;
  totalSec: number;
  remainingSec: number;
  startedAt: string | null;
  endsAt: string | null;
  roundsCompleted: number;
  roundsTotal: number;
  sessionId: number | null;
}
