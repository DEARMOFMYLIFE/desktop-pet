import type { TimerSnapshot, TimerStartRequest } from '../../../types';

interface MockPomodoro {
  rounds: number;
  focusMin: number;
}

interface MockSettings {
  pomodoro: MockPomodoro;
}

export function startTimerPatch(
  request: TimerStartRequest,
  settings: MockSettings,
): Partial<TimerSnapshot> {
  const startedAt = new Date().toISOString();
  if (request.kind === 'pomodoro' || request.kind === 'focus') {
    const total = request.kind === 'focus' ? request.minutes * 60 : settings.pomodoro.focusMin * 60;
    return {
      kind: 'pomodoro',
      phase: 'focus',
      status: 'running',
      totalSec: total,
      remainingSec: total,
      startedAt,
      endsAt: new Date(Date.now() + total * 1000).toISOString(),
      roundsCompleted: 0,
      roundsTotal: settings.pomodoro.rounds,
      sessionId: null,
    };
  }
  if (request.kind === 'countdown') {
    return {
      kind: 'countdown',
      phase: null,
      status: 'running',
      totalSec: request.seconds,
      remainingSec: request.seconds,
      startedAt,
      endsAt: new Date(Date.now() + request.seconds * 1000).toISOString(),
      roundsCompleted: 0,
      roundsTotal: 0,
      sessionId: null,
    };
  }
  return {
    kind: 'countup',
    phase: null,
    status: 'running',
    totalSec: 0,
    remainingSec: 0,
    startedAt,
    endsAt: null,
    roundsCompleted: 0,
    roundsTotal: 0,
    sessionId: null,
  };
}
