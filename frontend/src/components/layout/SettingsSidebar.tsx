import { memo } from 'react';
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
          <img src={iconSrc} alt="" decoding="async" />
        </div>
        <div className="brand-text">
          <strong>{petName || t('app.name')}</strong>
          <span>{t('settings.subtitle')}</span>
        </div>
      </div>

      <nav className="settings-nav" aria-label={t('settings.title')}>
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

      <div className="sidebar-foot">{t('settings.p1Notice')}</div>
    </aside>
  );
});
