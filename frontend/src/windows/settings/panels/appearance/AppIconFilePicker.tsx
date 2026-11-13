import { ImagePlus } from 'lucide-react';
import { useRef } from 'react';
import { Button } from '../../../../components/ui/Button';
import { t } from '../../../../lib/i18n';

interface AppIconFilePickerProps {
  preview: string;
  fileName: string;
  onChoose: (file: File | undefined) => void;
}

export function AppIconFilePicker({ preview, fileName, onChoose }: AppIconFilePickerProps) {
  const inputRef = useRef<HTMLInputElement>(null);

  const choose = () => inputRef.current?.click();

  return (
    <div className="icon-upload">
      <button
        type="button"
        className="icon-upload-preview"
        aria-label={t('settings.appearance.choosePng')}
        onClick={choose}
      >
        {preview ? (
          <img src={preview} alt="" />
        ) : (
          <ImagePlus size={26} aria-hidden="true" />
        )}
      </button>
      <div className="icon-upload-info">
        <p>{t('settings.appearance.iconUploadHint')}</p>
        <Button onClick={choose}>{t('settings.appearance.choosePng')}</Button>
        <span className="icon-upload-name">
          {fileName || t('settings.appearance.iconNoFile')}
        </span>
      </div>
      <input
        ref={inputRef}
        type="file"
        accept="image/png"
        className="hidden"
        onChange={(event) => {
          onChoose(event.target.files?.[0]);
          event.target.value = '';
        }}
      />
    </div>
  );
}
