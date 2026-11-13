import { useEffect, useState } from 'react';
import { RESERVED_APP_ICON_NAMES } from '../lib/appIcons';
import { errorMessage } from '../lib/errorMessage';
import { t } from '../lib/i18n';
import type { AppIconDialogProps } from '../types';

const MAX_BYTES = 5 * 1024 * 1024;

function duplicateName(name: string, existingNames: string[]): boolean {
  const normalized = name.trim().toLowerCase();
  return (
    RESERVED_APP_ICON_NAMES.some((reserved) => reserved.toLowerCase() === normalized) ||
    existingNames.some((existing) => existing.trim().toLowerCase() === normalized)
  );
}

export function useAppIconImport({
  open,
  existingNames,
  onClose,
  onImported,
  onImport,
}: AppIconDialogProps) {
  const [name, setNameValue] = useState('');
  const [file, setFile] = useState<File | null>(null);
  const [preview, setPreview] = useState('');
  const [error, setError] = useState('');
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (!open) return;
    setNameValue('');
    setFile(null);
    setPreview('');
    setError('');
    setSaving(false);
  }, [open]);

  useEffect(() => {
    if (!preview) return;
    return () => URL.revokeObjectURL(preview);
  }, [preview]);

  const setName = (value: string) => {
    setNameValue(value);
    setError('');
  };

  const clearName = () => setName('');

  const chooseFile = (next: File | undefined) => {
    if (!next) return;
    if (next.type !== 'image/png' && !next.name.toLowerCase().endsWith('.png')) {
      setError(t('settings.appearance.iconPngOnly'));
      return;
    }
    if (next.size > MAX_BYTES) {
      setError(t('settings.appearance.iconTooLarge'));
      return;
    }
    setFile(next);
    setPreview(URL.createObjectURL(next));
    setError('');
  };

  const save = async () => {
    const trimmed = name.trim();
    if (!trimmed) {
      setError(t('settings.appearance.iconNameRequired'));
      return;
    }
    if (duplicateName(trimmed, existingNames)) {
      setError(t('settings.appearance.iconNameDuplicate'));
      return;
    }
    if (!file) {
      setError(t('settings.appearance.iconFileRequired'));
      return;
    }
    setSaving(true);
    setError('');
    try {
      const bytes = Array.from(new Uint8Array(await file.arrayBuffer()));
      const icon = await onImport(trimmed, bytes);
      onImported(icon);
      onClose();
    } catch (reason) {
      setError(errorMessage(reason, t('settings.appearance.iconImportFailed')));
    } finally {
      setSaving(false);
    }
  };

  return {
    name,
    setName,
    clearName,
    file,
    preview,
    error,
    saving,
    chooseFile,
    save,
    canSave: Boolean(file && name.trim()),
  };
}
