import { useState } from 'react';
import type { KeyboardEvent } from 'react';
import { formatShortcut } from '../../lib/format';
import { t } from '../../lib/i18n';

interface ShortcutRecorderProps {
  value: string;
  label: string;
  onChange: (accelerator: string) => void;
  onError: (message: string) => void;
}

const MODIFIER_KEYS = new Set(['Control', 'Shift', 'Alt', 'Meta']);

function normalizeKey(event: KeyboardEvent<HTMLButtonElement>): string | null {
  const { code } = event;
  if (code.startsWith('Key')) return code.slice(3);
  if (code.startsWith('Digit')) return code.slice(5);
  if (/^F([1-9]|1[0-9]|2[0-4])$/.test(code)) return code;
  if (['Space', 'Enter', 'Tab', 'Backspace', 'Delete', 'Home', 'End'].includes(code)) {
    return code;
  }
  return null;
}

export function ShortcutRecorder({
  value,
  label,
  onChange,
  onError,
}: ShortcutRecorderProps) {
  const [recording, setRecording] = useState(false);

  const handleKeyDown = (event: KeyboardEvent<HTMLButtonElement>) => {
    if (!recording) return;
    event.preventDefault();
    event.stopPropagation();

    if (event.key === 'Escape') {
      setRecording(false);
      return;
    }
    if (MODIFIER_KEYS.has(event.key)) return;

    const key = normalizeKey(event);
    if (!key) {
      onError(t('settings.shortcuts.unsupported'));
      return;
    }

    const modifiers: string[] = [];
    if (event.ctrlKey || event.metaKey) modifiers.push('CmdOrCtrl');
    if (event.altKey) modifiers.push('Alt');
    if (event.shiftKey) modifiers.push('Shift');
    if (modifiers.length === 0) {
      onError(t('settings.shortcuts.needModifier'));
      return;
    }

    onChange([...modifiers, key].join('+'));
    setRecording(false);
  };

  return (
    <button
      type="button"
      className={`shortcut-recorder${recording ? ' is-recording' : ''}`}
      aria-label={label}
      onClick={() => setRecording(true)}
      onKeyDown={handleKeyDown}
      onBlur={() => setRecording(false)}
    >
      {recording ? t('settings.shortcuts.recording') : formatShortcut(value)}
    </button>
  );
}
