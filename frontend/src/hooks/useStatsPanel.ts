import { useEffect, useMemo, useState } from 'react';
import { errorMessage } from '../lib/errorMessage';
import { api } from '../lib/ipc';
import { t } from '../lib/i18n';
import type { StatsRange, StatsResult } from '../types';

export function useStatsPanel() {
  const [range, setRange] = useState<StatsRange>('today');
  const [stats, setStats] = useState<StatsResult | null>(null);
  const [error, setError] = useState('');

  useEffect(() => {
    let cancelled = false;
    setError('');
    void api
      .statsQuery(range)
      .then((next) => {
        if (!cancelled) setStats(next);
      })
      .catch((reason: unknown) => {
        if (!cancelled) {
          console.error('stats failed', reason);
          setError(errorMessage(reason, t('error.generic')));
        }
      });
    return () => {
      cancelled = true;
    };
  }, [range]);

  const max = useMemo(
    () => Math.max(1, ...(stats?.daily.map((day) => day.focusMin) ?? [0])),
    [stats],
  );
  const hasData = (stats?.focusMin ?? 0) > 0 || (stats?.rounds ?? 0) > 0;

  const startFocus = () => {
    void api.timerStart({ kind: 'pomodoro' }).catch(console.error);
  };

  return { range, setRange, stats, max, hasData, startFocus, error };
}
