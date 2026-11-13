import { SettingsHeader } from '../../components/layout/SettingsHeader';
import { SettingsSidebar } from '../../components/layout/SettingsSidebar';
import { ErrorPane } from '../../components/ui/ErrorPane';
import { LoadingPane } from '../../components/ui/LoadingPane';
import { Toast } from '../../components/ui/Toast';
import { useSettingsWindow } from '../../hooks/useSettingsWindow';
import { t } from '../../lib/i18n';
import { AboutPanel } from './panels/AboutPanel';
import { AppearancePanel } from './panels/AppearancePanel';
import { GeneralPanel } from './panels/GeneralPanel';
import { ReminderPanel } from './panels/ReminderPanel';
import { ShortcutsPanel } from './panels/ShortcutsPanel';
import { StatsPanel } from './panels/StatsPanel';
import { TimerPanel } from './panels/TimerPanel';
import { TodoPanel } from './panels/TodoPanel';
import { TABS } from './settingsTypes';
import './settings.css';

export function SettingsWindow() {
  const { settings, appInfo, activeTab, setActiveTab, toast, update, notify, loadError } =
    useSettingsWindow();

  if (loadError) {
    return <ErrorPane message={loadError} />;
  }

  if (!settings) {
    return <LoadingPane />;
  }

  const panelProps = { settings, appInfo, update, notify };

  return (
    <div className="settings-shell app-shell">
      <SettingsSidebar
        tabs={TABS}
        activeTab={activeTab}
        language={settings.language}
        appIcon={settings.appIcon}
        petName={settings.petName}
        onTabChange={setActiveTab}
      />

      <main className="settings-main">
        <SettingsHeader
          title={t(`settings.${activeTab}.title`)}
          description={t(`settings.${activeTab}.description`)}
        />

        <div className="settings-content">
          {activeTab === 'general' && <GeneralPanel {...panelProps} />}
          {activeTab === 'timer' && <TimerPanel {...panelProps} />}
          {activeTab === 'reminders' && <ReminderPanel {...panelProps} />}
          {activeTab === 'todos' && <TodoPanel {...panelProps} />}
          {activeTab === 'stats' && <StatsPanel {...panelProps} />}
          {activeTab === 'appearance' && <AppearancePanel {...panelProps} />}
          {activeTab === 'shortcuts' && <ShortcutsPanel {...panelProps} />}
          {activeTab === 'about' && <AboutPanel {...panelProps} />}
        </div>
      </main>

      {toast && <Toast message={toast} />}
    </div>
  );
}
