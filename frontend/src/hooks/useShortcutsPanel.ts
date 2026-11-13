import { useEffect, useState } from 'react';
import { api } from '../lib/ipc';
import { errorMessage } from '../lib/errorMessage';
import { t } from '../lib/i18n';
import type { Settings, ShortcutAction } from '../types';

export function useShortcutsPanel(settings: Settings) {
  const [focus, setFocus] = useState(settings.shortcuts.focus);
  const [clickThrough, setClickThrough] = useState(settings.shortcuts.clickThrough);
  const [toggleVisible, setToggleVisible] = useState(settings.shortcuts.toggleVisible);
  const [error, setError] = useState('');

  useEffect(() => {
    setFocus(settings.shortcuts.focus);
    setClickThrough(settings.shortcuts.clickThrough);
    setToggleVisible(settings.shortcuts.toggleVisible);
  }, [settings.shortcuts]);

  const applySaved = (saved: Settings) => {
    setFocus(saved.shortcuts.focus);
    setClickThrough(saved.shortcuts.clickThrough);
    setToggleVisible(saved.shortcuts.toggleVisible);
  };

  const resetLocal = () => {
    setFocus(settings.shortcuts.focus);
    setClickThrough(settings.shortcuts.clickThrough);
    setToggleVisible(settings.shortcuts.toggleVisible);
  };

  const updateShortcut = (
    action: ShortcutAction,
    accelerator: string,
    apply: (value: string) => void,
  ) => {
    setError('');
    apply(accelerator);
    void api
      .updateShortcut(action, accelerator)
      .then(applySaved)
      .catch((reason: unknown) => {
        setError(errorMessage(reason, t('settings.shortcuts.conflict')));
        resetLocal();
      });
  };

  const resetShortcuts = () => {
    setError('');
    void api
      .resetShortcuts()
      .then(applySaved)
      .catch((reason: unknown) =>
        setError(errorMessage(reason, t('settings.shortcuts.conflict'))),
      );
  };

  return {
    focus,
    clickThrough,
    toggleVisible,
    error,
    setError,
    setFocus,
    setClickThrough,
    setToggleVisible,
    updateShortcut,
    resetShortcuts,
  };
}
