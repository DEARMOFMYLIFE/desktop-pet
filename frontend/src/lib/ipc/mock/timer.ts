import type { TimerPhase, TimerSnapshot, TimerStartRequest } from '../../../types';
import { IpcError } from '../errors';
import { readMockSettings } from './settings';

let mockTimer: TimerSnapshot = {
  kind: 'pomodoro',
  phase: null,
  status: 'idle',
  totalSec: 25 * 60,
  remainingSec: 25 * 60,
  startedAt: null,
  endsAt: null,
  roundsCompleted: 0,
  roundsTotal: 4,
  sessionId: null,
};

export function currentTimer(): TimerSnapshot {
  return mockTimer;
}

function setMockTimer(patch: Partial<TimerSnapshot>): TimerSnapshot {
  mockTimer = { ...mockTimer, ...patch };
  return mockTimer;
}

function mockStartTimer(request: TimerStartRequest): TimerSnapshot {
  if (mockTimer.status !== 'idle' || mockTimer.phase !== null) {
    throw new IpcError('TIMER_CONFLICT', 'A timer is already active');
  }
  const settings = readMockSettings();
  if (request.kind === 'pomodoro') {
    const total = settings.pomodoro.focusMin * 60;
    return setMockTimer({
      kind: 'pomodoro',
      phase: 'focus',
      status: 'running',
      totalSec: total,
      remainingSec: total,
      startedAt: new Date().toISOString(),
      endsAt: new Date(Date.now() + total * 1000).toISOString(),
      roundsCompleted: 0,
      roundsTotal: settings.pomodoro.rounds,
      sessionId: null,
    });
  }
  if (request.kind === 'countdown') {
    return setMockTimer({
      kind: 'countdown',
      phase: null,
      status: 'running',
      totalSec: request.seconds,
      remainingSec: request.seconds,
      startedAt: new Date().toISOString(),
      endsAt: new Date(Date.now() + request.seconds * 1000).toISOString(),
      roundsCompleted: 0,
      roundsTotal: 0,
      sessionId: null,
    });
  }
  return setMockTimer({
    kind: 'countup',
    phase: null,
    status: 'running',
    totalSec: 0,
    remainingSec: 0,
    startedAt: new Date().toISOString(),
    endsAt: null,
    roundsCompleted: 0,
    roundsTotal: 0,
    sessionId: null,
  });
}

function mockNextPhase(phase: TimerPhase | null): TimerPhase | null {
  if (phase === 'focus') return 'short_break';
  if (phase === 'short_break' || phase === 'long_break') return 'focus';
  return null;
}

export function handleTimer<T>(
  cmd: string,
  args?: Record<string, unknown>,
): Promise<T> | null {
  switch (cmd) {
    case 'timer_snapshot':
      return Promise.resolve(mockTimer as T);
    case 'timer_start':
      return Promise.resolve(mockStartTimer(args?.request as TimerStartRequest) as T);
    case 'timer_pause':
      return Promise.resolve(setMockTimer({ status: 'paused' }) as T);
    case 'timer_resume':
      return Promise.resolve(setMockTimer({ status: 'running' }) as T);
    case 'timer_skip': {
      const next = mockNextPhase(mockTimer.phase);
      const settings = readMockSettings();
      const total =
        next === 'focus'
          ? settings.pomodoro.focusMin * 60
          : next === 'long_break'
            ? settings.pomodoro.longBreakMin * 60
            : next === 'short_break'
              ? settings.pomodoro.shortBreakMin * 60
              : 0;
      return Promise.resolve(
        setMockTimer({
          phase: next,
          status: 'idle',
          totalSec: total,
          remainingSec: total,
          startedAt: null,
          endsAt: null,
        }) as T,
      );
    }
    case 'timer_stop':
      return Promise.resolve(
        setMockTimer({
          kind: 'pomodoro',
          phase: null,
          status: 'idle',
          totalSec: 0,
          remainingSec: 0,
          startedAt: null,
          endsAt: null,
          roundsCompleted: 0,
          roundsTotal: 0,
        }) as T,
      );
    case 'timer_reset':
      return Promise.resolve(
        setMockTimer({
          status: 'idle',
          remainingSec: mockTimer.totalSec,
          startedAt: null,
          endsAt: null,
        }) as T,
      );
    default:
      return null;
  }
}
