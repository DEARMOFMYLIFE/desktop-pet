import { useEffect, useState } from 'react';
import { errorMessage } from '../lib/errorMessage';
import { api } from '../lib/ipc';
import { preloadAllSkins, preloadSkinStates } from '../lib/character/preload';
import { resolveLocale, setLocale, t } from '../lib/i18n';
import type { Language, Settings } from '../types';

export const TOTAL_STEPS = 3;

export function useOnboarding() {
  const [step, setStep] = useState(0);
  const [settings, setSettings] = useState<Settings | null>(null);
  const [saving, setSaving] = useState(false);
  const [permission, setPermission] = useState('unknown');
  const [loadError, setLoadError] = useState('');

  useEffect(() => {
    preloadAllSkins();
  }, []);

  useEffect(() => {
    if (!settings) return;
    preloadSkinStates(settings.petSkin);
  }, [settings]);

  useEffect(() => {
    api
      .bootstrap()
      .then((bootstrap) => {
        setSettings(bootstrap.settings);
        setLocale(resolveLocale(bootstrap.settings.language));
      })
      .catch((error: unknown) => {
        console.error('bootstrap failed', error);
        setLoadError(errorMessage(error, t('error.generic')));
      });
  }, []);

  useEffect(() => {
    api
      .notificationStatus()
      .then(setPermission)
      .catch((error: unknown) => console.error('notification status failed', error));
  }, []);

  const patchSettings = (patch: Partial<Settings>) => {
    setSettings((previous) => (previous ? { ...previous, ...patch } : previous));
  };

  const setLanguage = (language: Language) => {
    patchSettings({ language });
    setLocale(resolveLocale(language));
  };

  const requestPermission = () => {
    void api
      .requestNotificationPermission()
      .then(setPermission)
      .catch((error: unknown) => console.error('notification permission failed', error));
  };

  const finish = async () => {
    if (!settings || saving) return;
    setSaving(true);
    try {
      await api.updateSettings({
        ...settings,
        petName: settings.petName.trim() || '小柚',
        firstRun: false,
      });
      await api.closeWindow('onboarding');
    } catch (error) {
      console.error('onboarding finish failed', error);
      setSaving(false);
    }
  };

  return {
    step,
    settings,
    saving,
    permission,
    loadError,
    patchSettings,
    setLanguage,
    requestPermission,
    finish,
    next: () => setStep((value) => value + 1),
    back: () => setStep((value) => value - 1),
  };
}