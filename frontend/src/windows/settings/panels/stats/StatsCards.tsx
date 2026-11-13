import { t } from '../../../../lib/i18n';
import type { StatsResult } from '../../../../types';

export function StatsCards({ stats }: { stats: StatsResult | null }) {
  const completionRate =
    stats?.completionRate === null || stats?.completionRate === undefined
      ? '—'
      : `${Math.round(stats.completionRate * 100)}%`;

  const cards = [
    { value: stats?.focusMin ?? 0, label: t('settings.stats.focusMinutes') },
    { value: stats?.rounds ?? 0, label: t('settings.stats.rounds') },
    { value: completionRate, label: t('settings.stats.completionRate') },
    { value: stats?.breakMin ?? 0, label: t('settings.stats.breakMinutes') },
  ];

  return (
    <div className="stats-grid">
      {cards.map((card) => (
        <div className="stats-card" key={card.label}>
          <strong>{card.value}</strong>
          <span>{card.label}</span>
        </div>
      ))}
    </div>
  );
}
