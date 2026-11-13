/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{ts,tsx}', './*.html'],
  theme: {
    extend: {
      colors: {
        bg: 'var(--bg)',
        surface: 'var(--surface)',
        'surface-muted': 'var(--surface-muted)',
        ink: 'var(--text)',
        muted: 'var(--text-secondary)',
        line: 'var(--border)',
        accent: 'var(--accent)',
        'accent-soft': 'var(--accent-soft)',
        'accent-ink': 'var(--accent-ink)',
        success: 'var(--success)',
        warning: 'var(--warning)',
        danger: 'var(--danger)',
        ring: 'var(--focus-ring)',
      },
      borderRadius: {
        xs: 'var(--radius-xs)',
        sm: 'var(--radius-sm)',
        md: 'var(--radius-md)',
        lg: 'var(--radius-lg)',
        xl: 'var(--radius-xl)',
      },
      boxShadow: {
        soft: 'var(--shadow)',
        sm: 'var(--shadow-sm)',
        xs: 'var(--shadow-xs)',
      },
      fontFamily: {
        ui: 'var(--font-ui)',
      },
    },
  },
  plugins: [],
};
