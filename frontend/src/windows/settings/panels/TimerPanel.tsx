import { X } from 'lucide-react';
import { SettingsCard } from '../../../components/settings/SettingsCard';
import { SettingRow } from '../../../components/settings/SettingRow';
import { Button } from '../../../components/ui/Button';
import { ClearButton } from '../../../components/ui/ClearButton';
import { IconButton } from '../../../components/ui/IconButton';
import { Input } from '../../../components/ui/Input';
import { NumberInput } from '../../../components/ui/NumberInput';
import { Switch } from '../../../components/ui/Switch';
import { useTimerPanel } from '../../../hooks/useTimerPanel';
import { t } from '../../../lib/i18n';
import type { SettingsPanelProps } from '../settingsTypes';

const NUMBER_FIELDS = [
  ['focusMin', 'settings.timer.focus'],
  ['shortBreakMin', 'settings.timer.shortBreak'],
  ['longBreakMin', 'settings.timer.longBreak'],
  ['rounds', 'settings.timer.rounds'],
] as const;

export function TimerPanel({ settings, update }: SettingsPanelProps) {
  const timer = useTimerPanel(settings, update);

  return (
    <>
      <SettingsCard title={t('settings.timer.pomodoro')}>
        {NUMBER_FIELDS.map(([key, label]) => (
          <SettingRow key={key} title={t(label)}>
            <NumberInput
              compact
              value={settings.pomodoro[key]}
              min={timer.limits[key][0]}
              max={timer.limits[key][1]}
              label={t(label)}
              unit={key === 'rounds' ? t('settings.timer.rounds') : t('settings.timer.minutes')}
              onChange={(value) => timer.setPomodoro(key, value)}
            />
          </SettingRow>
        ))}
        <SettingRow
          title={t('settings.timer.autoStart')}
          hint={t('settings.timer.autoStartHint')}
        >
          <Switch
            checked={settings.pomodoro.autoStartNext}
            label={t('settings.timer.autoStart')}
            onChange={(checked) => timer.setPomodoro('autoStartNext', checked)}
          />
        </SettingRow>
      </SettingsCard>

      <SettingsCard title={t('settings.timer.presets')}>
        <div className="chip-row">
          {settings.timerPresets.map((minutes) => (
            <span key={minutes} className="chip preset-chip">
              {minutes} {t('settings.timer.minutes')}
              <IconButton
                label={t('settings.timer.removePreset')}
                className="preset-remove"
                onClick={() => timer.removePreset(minutes)}
              >
                <X size={11} aria-hidden="true" />
              </IconButton>
            </span>
          ))}
        </div>
        <div className="preset-add">
          <div className="clearable-field preset-input">
            <Input
              type="text"
              inputMode="numeric"
              autoComplete="off"
              maxLength={4}
              value={timer.presetDraft}
              placeholder={t('settings.timer.presetPlaceholder')}
              onInput={(event) => {
                const cleaned = event.currentTarget.value.replace(/[^0-9]/g, '').slice(0, 4);
                if (cleaned !== event.currentTarget.value) {
                  event.currentTarget.value = cleaned;
                }
              }}
              onChange={(event) =>
                timer.setPresetDraft(event.target.value.replace(/[^0-9]/g, '').slice(0, 4))
              }
              onKeyDown={(event) => {
                if (event.key === 'Enter') timer.addPreset();
              }}
            />
            {timer.presetDraft && (
              <ClearButton onClick={() => timer.setPresetDraft('')} />
            )}
          </div>
          <Button className="btn-control" variant="primary" onClick={timer.addPreset}>
            {t('settings.timer.addPreset')}
          </Button>
        </div>
      </SettingsCard>
    </>
  );
}
