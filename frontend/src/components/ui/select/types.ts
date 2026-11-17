export interface SelectOption<T extends string> {
  value: T;
  label: string;
}

export interface SelectMenuPosition {
  left: number;
  top: number;
  width: number;
  maxHeight: number;
}
