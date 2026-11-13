import { Button } from '../../../../components/ui/Button';
import { Modal } from '../../../../components/ui/Modal';
import { t } from '../../../../lib/i18n';
import { TodoForm } from './TodoForm';

interface TodoDialogProps {
  open: boolean;
  editing: boolean;
  title: string;
  note: string;
  due: string;
  remind: string;
  error: string;
  saving: boolean;
  onTitleChange: (value: string) => void;
  onNoteChange: (value: string) => void;
  onDueChange: (value: string) => void;
  onRemindChange: (value: string) => void;
  onSave: () => void;
  onCancel: () => void;
}

export function TodoDialog({
  open,
  editing,
  title,
  note,
  due,
  remind,
  error,
  saving,
  onTitleChange,
  onNoteChange,
  onDueChange,
  onRemindChange,
  onSave,
  onCancel,
}: TodoDialogProps) {
  return (
    <Modal
      open={open}
      title={editing ? t('settings.todos.editTitle') : t('settings.todos.add')}
      description={t('settings.todos.dialogHint')}
      onClose={onCancel}
      className="max-w-[560px]"
      footer={
        <>
          <Button onClick={onCancel}>{t('common.cancel')}</Button>
          <Button variant="primary" loading={saving} onClick={onSave}>
            {t('common.save')}
          </Button>
        </>
      }
    >
      <TodoForm
        title={title}
        note={note}
        due={due}
        remind={remind}
        error={error}
        onTitleChange={onTitleChange}
        onNoteChange={onNoteChange}
        onDueChange={onDueChange}
        onRemindChange={onRemindChange}
      />
    </Modal>
  );
}