export function pad(value: number): string {
  return String(value).padStart(2, '0');
}

export function ymd(date: Date): string {
  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}`;
}

export function addDays(date: Date, amount: number): Date {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate() + amount);
}

export function startOfMonth(date: Date): Date {
  return new Date(date.getFullYear(), date.getMonth(), 1);
}

export function parseDate(value: string): Date | null {
  if (!value) return null;
  const date = new Date(value);
  return Number.isNaN(date.getTime()) ? null : date;
}

export function formatDateTime(value: string, locale: string): string {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return value;
  return new Intl.DateTimeFormat(locale, {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  }).format(date);
}

export function monthLabel(locale: string, date: Date): string {
  return new Intl.DateTimeFormat(locale, {
    year: 'numeric',
    month: 'long',
  }).format(date);
}

export function monthLabels(locale: string): string[] {
  return Array.from({ length: 12 }, (_, index) =>
    new Intl.DateTimeFormat(locale, { month: 'short' }).format(new Date(2020, index, 1)),
  );
}

export function weekdayLabels(locale: string): string[] {
  return locale === 'zh-CN'
    ? ['一', '二', '三', '四', '五', '六', '日']
    : ['Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa', 'Su'];
}

export function firstYearOfGrid(date: Date): number {
  return Math.floor(date.getFullYear() / 12) * 12;
}

export function yearGrid(date: Date): number[] {
  const first = firstYearOfGrid(date);
  return Array.from({ length: 12 }, (_, index) => first + index);
}
