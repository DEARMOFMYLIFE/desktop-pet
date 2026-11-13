import type { CustomIcon } from './settings';

export interface AppIconDialogProps {
  open: boolean;
  existingNames: string[];
  onClose: () => void;
  onImported: (icon: CustomIcon) => void;
  onImport: (name: string, bytes: number[]) => Promise<CustomIcon>;
}
