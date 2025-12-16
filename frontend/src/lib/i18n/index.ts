import { browser } from '$app/environment';
import { init, register, getLocaleFromNavigator, locale } from 'svelte-i18n';

const defaultLocale = 'en';

register('en', () => import('./locales/en.json'));
register('th', () => import('./locales/th.json'));

init({
  fallbackLocale: defaultLocale,
  initialLocale: browser ? getLocaleFromNavigator() : defaultLocale,
});

export { locale };
