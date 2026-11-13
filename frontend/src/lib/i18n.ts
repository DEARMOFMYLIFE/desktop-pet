import enUS from '@locales/en-US.json';
import zhCN from '@locales/zh-CN.json';

export type Locale = 'zh-CN' | 'en-US';

type FlatDictionary = Record<string, string>;

function flatten(source: Record<string, unknown>, prefix = ''): FlatDictionary {
  const result: FlatDictionary = {};
  for (const [key, value] of Object.entries(source)) {
    const path = prefix ? `${prefix}.${key}` : key;
    if (typeof value === 'string') {
      result[path] = value;
    } else if (value && typeof value === 'object' && !Array.isArray(value)) {
      Object.assign(result, flatten(value as Record<string, unknown>, path));
    }
  }
  return result;
}

const dictionaries: Record<Locale, FlatDictionary> = {
  'zh-CN': flatten(zhCN as Record<string, unknown>),
  'en-US': flatten(enUS as Record<string, unknown>),
};

let currentLocale: Locale = 'zh-CN';

export function resolveLocale(language: string): Locale {
  if (language === 'zh-CN' || language === 'en-US') return language;
  const system = typeof navigator !== 'undefined' ? navigator.language : 'en-US';
  return system.toLowerCase().startsWith('zh') ? 'zh-CN' : 'en-US';
}

export function setLocale(locale: Locale): void {
  currentLocale = locale;
  if (typeof document !== 'undefined') {
    document.documentElement.lang = locale;
  }
}

export function getLocale(): Locale {
  return currentLocale;
}

export function t(key: string, params?: Record<string, string | number>): string {
  const template =
    dictionaries[currentLocale][key] ?? dictionaries['en-US'][key] ?? key;
  if (!params) return template;
  return template.replace(/\{(\w+)\}/g, (_, name: string) =>
    params[name] === undefined ? `{${name}}` : String(params[name]),
  );
}
