import { Button } from '../../../../components/ui/Button';
import { Modal } from '../../../../components/ui/Modal';
import { useAppIconImport } from '../../../../hooks/useAppIconImport';
import { t } from '../../../../lib/i18n';
import type { AppIconDialogProps } from '../../../../types';
import { AppIconFilePicker } from './AppIconFilePicker';
import { AppIconNameField } from './AppIconNameField';

export function AppIconDialog(props: AppIconDialogProps) {
  const form = useAppIconImport(props);

  return (
    <Modal
      open={props.open}
      title={t('settings.appearance.customIconTitle')}
      description={t('settings.appearance.customIconHint')}
      onClose={props.onClose}
      className="max-w-[480px]"
      footer={
        <>
          <Button onClick={props.onClose}>{t('common.cancel')}</Button>
          <Button
            variant="primary"
            loading={form.saving}
            disabled={form.saving || !form.canSave}
            onClick={() => void form.save()}
          >
            {t('common.save')}
          </Button>
        </>
      }
    >
      <AppIconFilePicker
        preview={form.preview}
        fileName={form.file?.name ?? ''}
        onChoose={form.chooseFile}
      />
      <AppIconNameField
        name={form.name}
        onChange={form.setName}
        onClear={form.clearName}
      />
      {form.error && <p className="form-error">{form.error}</p>}
    </Modal>
  );
}
