import { afterEach, describe, expect, it } from 'vitest';
import { setLocale, t } from './i18n';

afterEach(() => setLocale('zh-CN'));

describe('i18n', () => {
  it('translates both locales', () => {
    setLocale('zh-CN');
    expect(t('app.name')).toBe('小柚');
    expect(t('settings.general.alwaysOnTop')).toBe('始终置顶');

    setLocale('en-US');
    expect(t('app.name')).toBe('Yuzu');
    expect(t('settings.general.alwaysOnTop')).toBe('Always on top');
  });

  it('interpolates parameters', () => {
    setLocale('en-US');
    expect(t('onboarding.step', { n: 2 })).toBe('Step 2 of 3');
  });

  it('falls back to the key when missing', () => {
    setLocale('en-US');
    expect(t('missing.key')).toBe('missing.key');
  });
});
