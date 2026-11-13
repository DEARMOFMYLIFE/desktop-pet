export type StatsRange = 'today' | '7d' | '30d';

export interface DailyStat {
  date: string;
  focusMin: number;
  rounds: number;
}

export interface StatsResult {
  focusMin: number;
  rounds: number;
  completionRate: number | null;
  breakMin: number;
  daily: DailyStat[];
}
