import { ErrorNotice } from '../../../components/ui/ErrorNotice';
import { SettingsCard } from '../../../components/settings/SettingsCard';
import { Button } from '../../../components/ui/Button';
import { useStatsPanel } from '../../../hooks/useStatsPanel';
import { t } from '../../../lib/i18n';
import type { SettingsPanelProps } from '../settingsTypes';
import { StatsCards } from './stats/StatsCards';
import { StatsChart } from './stats/StatsChart';

const RANGES = [
  ['today', 'settings.stats.today'],
  ['7d', 'settings.stats.7d'],
  ['30d', 'settings.stats.30d'],
] as const;

export function StatsPanel(_props: SettingsPanelProps) {
  const { range, setRange, stats, max, hasData, startFocus, error } = useStatsPanel();

  return (
    <>
      <div className="stats-toolbar">
        <div className="segmented" role="group" aria-label={t('settings.stats.title')}>
          {RANGES.map(([value, key]) => (
            <button
              key={value}
              type="button"
              aria-pressed={range === value}
              onClick={() => setRange(value)}
            >
              {t(key)}
            </button>
          ))}
        </div>
      </div>

      {error && <ErrorNotice>{error}</ErrorNotice>}

      <StatsCards stats={stats} />

      <SettingsCard title={t('settings.stats.daily')}>
        {hasData ? (
          <StatsChart daily={stats?.daily ?? []} max={max} />
        ) : (
          <div className="empty">
            <div>
              <p>{t('settings.stats.empty')}</p>
              <Button variant="primary" onClick={startFocus}>
                {t('settings.stats.startFocus')}
              </Button>
            </div>
          </div>
        )}
      </SettingsCard>
    </>
  );
}
