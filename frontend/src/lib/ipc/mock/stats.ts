import type { StatsResult } from '../../../types';

export function handleStats<T>(
  cmd: string,
  args?: Record<string, unknown>,
): Promise<T> | null {
  if (cmd !== 'stats_query') return null;

  const range = String(args?.range ?? 'today');
  const days = range === '30d' ? 30 : range === '7d' ? 7 : 1;
  const daily = Array.from({ length: days }, (_, index) => {
    const date = new Date();
    date.setDate(date.getDate() - (days - 1 - index));
    return {
      date: date.toISOString().slice(0, 10),
      focusMin: 0,
      rounds: 0,
    };
  });
  const result: StatsResult = {
    focusMin: 0,
    rounds: 0,
    completionRate: null,
    breakMin: 0,
    daily,
  };
  return Promise.resolve(result as T);
}
