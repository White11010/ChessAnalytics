const THEME_KEY = 'app-theme';

type ThemeName = 'light' | 'dark';

export function initTheme(vuetify: any): void {
  const saved = localStorage.getItem(THEME_KEY) as ThemeName | null;

  const initial: ThemeName = saved === 'light' || saved === 'dark' ? saved : 'dark';

  vuetify.theme.change(initial);
}
