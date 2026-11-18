import { memo } from 'react';
import { AppIconImage } from '../ui/AppIconImage';
import { ScrollArea } from '../ui/ScrollArea';
import { useAppIconSrc } from '../../hooks/useAppIconSrc';
import { t } from '../../lib/i18n';
import type { AppIcon, SettingsTab, TabId } from '../../types';

interface SettingsSidebarProps {
  tabs: SettingsTab[];
  activeTab: TabId;
  language: string;
  appIcon: AppIcon;
  petName: string;
  onTabChange: (tab: TabId) => void;
}

export const SettingsSidebar = memo(function SettingsSidebar({
  tabs,
  activeTab,
  language,
  appIcon,
  petName,
  onTabChange,
}: SettingsSidebarProps) {
  const iconSrc = useAppIconSrc(appIcon);

  return (
    <aside className="settings-sidebar" data-language={language}>
      <div className="brand">
        <div className="brand-mark">
          <AppIconImage src={iconSrc} />
        </div>
        <div className="brand-text">
          <strong>{petName || t('app.name')}</strong>
          <span>{t('settings.subtitle')}</span>
        </div>
      </div>

      <ScrollArea className="settings-nav-scroll" label={t('settings.title')} compact>
        <nav className="settings-nav">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              type="button"
              className={activeTab === tab.id ? 'is-active' : ''}
              aria-current={activeTab === tab.id ? 'page' : undefined}
              onClick={() => onTabChange(tab.id)}
            >
              {t(tab.labelKey)}
            </button>
          ))}
        </nav>
      </ScrollArea>

      <div className="sidebar-foot">{t('settings.p1Notice')}</div>
    </aside>
  );
});
