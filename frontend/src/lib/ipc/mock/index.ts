import { IpcError } from '../errors';
import { handleApp } from './app';
import { handlePet } from './pet';
import { handleReminder } from './reminder';
import { handleStats } from './stats';
import { handleTimer } from './timer';
import { handleTodo } from './todo';

export type MockHandler = <T>(
  cmd: string,
  args?: Record<string, unknown>,
) => Promise<T> | null;

const HANDLERS: MockHandler[] = [
  handleApp,
  handlePet,
  handleTimer,
  handleTodo,
  handleReminder,
  handleStats,
];

export function mockCall<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  for (const handler of HANDLERS) {
    const result = handler<T>(cmd, args);
    if (result) return result;
  }
  return Promise.reject(new IpcError('UNKNOWN_COMMAND', `Unknown command: ${cmd}`));
}
