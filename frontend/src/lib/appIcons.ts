export const BUILTIN_APP_ICONS = [
  'clock',
  'star',
  'moon',
  'coffee',
  'sun',
  'leaf',
  'heart',
  'cloud',
  'book',
  'flower',
] as const;

export const RESERVED_APP_ICON_NAMES = [
  'clock',
  'star',
  'moon',
  'coffee',
  'sun',
  'leaf',
  'heart',
  'cloud',
  'book',
  'flower',
  '时钟',
  '星星',
  '月亮',
  '咖啡',
  '太阳',
  '叶子',
  '爱心',
  '云朵',
  '书本',
  '花朵',
  'Clock',
  'Star',
  'Moon',
  'Coffee',
  'Sun',
  'Leaf',
  'Heart',
  'Cloud',
  'Book',
  'Flower',
] as const;

export function isBuiltinAppIcon(value: string): boolean {
  return (BUILTIN_APP_ICONS as readonly string[]).includes(value);
}
