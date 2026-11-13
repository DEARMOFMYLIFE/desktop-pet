import { ArrowRight, Lightbulb } from 'lucide-react';
import { Notice } from '../../../components/ui/Notice';
import { SettingsCard } from '../../../components/settings/SettingsCard';
import { SettingRow } from '../../../components/settings/SettingRow';
import { Input } from '../../../components/ui/Input';
import { NumberInput } from '../../../components/ui/NumberInput';
import { Segmented } from '../../../components/ui/Segmented';
import { Switch } from '../../../components/ui/Switch';
import { useReminderPanel } from '../../../hooks/useReminderPanel';
import { t } from '../../../lib/i18n';
import type { SettingsPanelProps } from '../settingsTypes';

const REMINDERS = [
  { enabledKey: 'waterEnabled', intervalKey: 'waterIntervalMin', labelKey: 'settings.reminders.water' },
  {
    enabledKey: 'sedentaryEnabled',
    intervalKey: 'sedentaryIntervalMin',
    labelKey: 'settings.reminders.sedentary',
  },
  { enabledKey: 'eyesEnabled', intervalKey: 'eyesIntervalMin', labelKey: 'settings.reminders.eyes' },
] as const;

export function ReminderPanel({ settings, update }: SettingsPanelProps) {
  const { setReminder, setInterval } = useReminderPanel(settings, update);

  return (
    <>
      <SettingsCard
        title={t('settings.reminders.types')}
        description={t('settings.reminders.typesHint')}
      >
        {REMINDERS.map((item) => (
          <SettingRow key={item.enabledKey} title={t(item.labelKey)}>
            <div className="reminder-controls">
              <NumberInput
                compact
                min={15}
                max={180}
                value={settings.reminders[item.intervalKey]}
                label={t('settings.reminders.interval')}
                unit={t('settings.reminders.minutes')}
                onChange={(value) => setInterval(item.intervalKey, value)}
              />
              <Switch
                checked={settings.reminders[item.enabledKey]}
                label={t(item.labelKey)}
                onChange={(checked) => setReminder({ [item.enabledKey]: checked })}
              />
            </div>
          </SettingRow>
        ))}
      </SettingsCard>

      <SettingsCard title={t('settings.reminders.activeHours')}>
        <SettingRow
          title={t('settings.reminders.activeStart')}
          hint={t('settings.reminders.activeHint')}
        >
          <div className="time-range">
            <Input
              type="time"
              value={settings.reminders.activeStart}
              onChange={(event) => setReminder({ activeStart: event.target.value })}
            />
            <ArrowRight className="text-muted" size={12} aria-hidden="true" />
            <Input
              type="time"
              value={settings.reminders.activeEnd}
              onChange={(event) => setReminder({ activeEnd: event.target.value })}
            />
          </div>
        </SettingRow>
        <SettingRow title={t('settings.reminders.focusBehavior')}>
          <Segmented
            value={settings.reminders.focusBehavior}
            options={[
              { value: 'notify_only', label: t('settings.reminders.notifyOnly') },
              { value: 'pause', label: t('settings.reminders.pause') },
            ]}
            label={t('settings.reminders.focusBehavior')}
            onChange={(value) => setReminder({ focusBehavior: value })}
          />
        </SettingRow>
        <SettingRow title={t('settings.reminders.snooze')}>
          <Segmented
            value={String(settings.reminders.snoozeMin)}
            options={[
              { value: '5', label: t('settings.reminders.snooze5') },
              { value: '10', label: t('settings.reminders.snooze10') },
            ]}
            label={t('settings.reminders.snooze')}
            onChange={(value) => setReminder({ snoozeMin: Number(value) })}
          />
        </SettingRow>
      </SettingsCard>

      <Notice icon={<Lightbulb size={14} aria-hidden="true" />}>
        {t('settings.reminders.footer')}
      </Notice>
    </>
  );
}
