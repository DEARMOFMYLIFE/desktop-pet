import { ImagePlus } from 'lucide-react';
import { useEffect, useRef, useState } from 'react';
import { Button } from '../../../../components/ui/Button';
import { ClearButton } from '../../../../components/ui/ClearButton';
import { Input } from '../../../../components/ui/Input';
import { Modal } from '../../../../components/ui/Modal';
import { errorMessage } from '../../../../lib/errorMessage';
import { RESERVED_APP_ICON_NAMES } from '../../../../lib/appIcons';
import { t } from '../../../../lib/i18n';
import type { CustomIcon } from '../../../../types';

const MAX_BYTES = 5 * 1024 * 1024;

interface AppIconDialogProps {
  open: boolean;
  existingNames: string[];
  onClose: () => void;
  onImported: (icon: CustomIcon) => void;
  onImport: (name: string, bytes: number[]) => Promise<CustomIcon>;
}

function duplicateName(name: string, existingNames: string[]): boolean {
  const normalized = name.trim().toLowerCase();
  return (
    RESERVED_APP_ICON_NAMES.some((reserved) => reserved.toLowerCase() === normalized) ||
    existingNames.some((existing) => existing.trim().toLowerCase() === normalized)
  );
}

export function AppIconDialog({
  open,
  existingNames,
  onClose,
  onImported,
  onImport,
}: AppIconDialogProps) {
  const inputRef = useRef<HTMLInputElement>(null);
  const [name, setName] = useState('');
  const [file, setFile] = useState<File | null>(null);
  const [preview, setPreview] = useState('');
  const [error, setError] = useState('');
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (!open) return;
    setName('');
    setFile(null);
    setPreview('');
    setError('');
    setSaving(false);
  }, [open]);

  useEffect(() => {
    if (!preview) return;
    return () => URL.revokeObjectURL(preview);
  }, [preview]);

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

  return (
    <Modal
      open={open}
      title={t('settings.appearance.customIconTitle')}
      description={t('settings.appearance.customIconHint')}
      onClose={onClose}
      className="max-w-[480px]"
      footer={
        <>
          <Button onClick={onClose}>{t('common.cancel')}</Button>
          <Button
            variant="primary"
            loading={saving}
            disabled={saving || !file || !name.trim()}
            onClick={() => void save()}
          >
            {t('common.save')}
          </Button>
        </>
      }
    >
      <div className="icon-upload">
        <button
          type="button"
          className="icon-upload-preview"
          aria-label={t('settings.appearance.choosePng')}
          onClick={() => inputRef.current?.click()}
        >
          {preview ? (
            <img src={preview} alt="" />
          ) : (
            <ImagePlus size={26} aria-hidden="true" />
          )}
        </button>
        <div className="icon-upload-info">
          <p>{t('settings.appearance.iconUploadHint')}</p>
          <Button onClick={() => inputRef.current?.click()}>
            {t('settings.appearance.choosePng')}
          </Button>
          <span className="icon-upload-name">
            {file?.name ?? t('settings.appearance.iconNoFile')}
          </span>
        </div>
        <input
          ref={inputRef}
          type="file"
          accept="image/png"
          className="hidden"
          onChange={(event) => {
            chooseFile(event.target.files?.[0]);
            event.target.value = '';
          }}
        />
      </div>

      <label className="icon-name-field">
        <span>{t('settings.appearance.iconName')}</span>
        <div className="clearable-field">
          <Input
            autoFocus
            maxLength={32}
            value={name}
            placeholder={t('settings.appearance.iconNamePlaceholder')}
            onChange={(event) => {
              setName(event.target.value);
              setError('');
            }}
          />
          {name && (
            <ClearButton
              onClick={() => {
                setName('');
                setError('');
              }}
            />
          )}
        </div>
      </label>

      {error && <p className="form-error">{error}</p>}
    </Modal>
  );
}