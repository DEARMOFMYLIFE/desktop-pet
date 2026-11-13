import type { DailyStat } from '../../../../types';

interface StatsChartProps {
  daily: DailyStat[];
  max: number;
}

export function StatsChart({ daily, max }: StatsChartProps) {
  return (
    <div className="stats-chart">
      {daily.map((day) => (
        <div className="stats-bar-wrap" key={day.date}>
          <div className="stats-bar-value">{day.focusMin || ''}</div>
          <div
            className="stats-bar"
            style={{ height: `${Math.max(2, (day.focusMin / max) * 120)}px` }}
          />
          <span>{day.date.slice(5)}</span>
        </div>
      ))}
    </div>
  );
}
