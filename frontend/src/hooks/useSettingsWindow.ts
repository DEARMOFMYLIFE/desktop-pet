import { useEffect, useState } from 'react';
import { errorMessage } from '../lib/errorMessage';
import { api } from '../lib/ipc';
import { resolveLocale, setLocale, t } from '../lib/i18n';
import type { AppInfo, Settings, TabId } from '../types';

export function useSettingsWindow() {
  const [settings, setSettings] = useState<Settings | null>(null);
  const [appInfo, setAppInfo] = useState<AppInfo | null>(null);
  const [activeTab, setActiveTab] = useState<TabId>('general');
  const [toast, setToast] = useState('');
  const [loadError, setLoadError] = useState('');

  useEffect(() => {
    let disposed = false;
    api
      .bootstrap()
      .then((bootstrap) => {
        if (disposed) return;
        setSettings(bootstrap.settings);
        setAppInfo(bootstrap.app);
        setLocale(resolveLocale(bootstrap.settings.language));
      })
      .catch((error: unknown) => {
        console.error('bootstrap failed', error);
        if (!disposed) setLoadError(errorMessage(error, t('error.generic')));
      });
    return () => {
      disposed = true;
    };
  }, []);

  useEffect(() => {
    if (!toast) return;
    const timer = window.setTimeout(() => setToast(''), 1800);
    return () => window.clearTimeout(timer);
  }, [toast]);

  useEffect(() => {
    let disposed = false;
    const unlisteners: Array<() => void> = [];
    api
      .on<Settings>('settings:changed', (next) => {
        setSettings(next);
        setLocale(resolveLocale(next.language));
      })
      .then((unlisten) => {
        if (disposed) {
          unlisten();
        } else {
          unlisteners.push(unlisten);
        }
      });
    return () => {
      disposed = true;
      unlisteners.forEach((unlisten) => unlisten());
    };
  }, []);

  async function update(patch: Partial<Settings>) {
    if (!settings) return;
    const previous = settings;
    const next = { ...settings, ...patch };
    setSettings(next);
    if (patch.language) {
      setLocale(resolveLocale(next.language));
    }
    try {
      const saved = await api.updateSettings(next);
      setSettings(saved);
      setLocale(resolveLocale(saved.language));
      setToast(t('common.saved'));
    } catch (error) {
      console.error('settings update failed', error);
      setSettings(previous);
      setToast(t('error.generic'));
    }
  }

  return {
    settings,
    appInfo,
    activeTab,
    setActiveTab,
    toast,
    update,
    notify: setToast,
    loadError,
  };
}
