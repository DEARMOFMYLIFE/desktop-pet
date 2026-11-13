import { describe, expect, it } from 'vitest';
import { formatClock } from './format';

describe('formatClock', () => {
  it('formats minutes and seconds', () => {
    expect(formatClock(0)).toBe('00:00');
    expect(formatClock(65)).toBe('01:05');
    expect(formatClock(1500)).toBe('25:00');
  });

  it('formats hours when needed', () => {
    expect(formatClock(3661)).toBe('1:01:01');
  });

  it('clamps negative values', () => {
    expect(formatClock(-5)).toBe('00:00');
  });
});
