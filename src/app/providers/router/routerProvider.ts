import { createMemoryHistory, createRouter } from 'vue-router';

import { AnalizeBoardPage } from '@/pages/AnalizeBoardPage';
import { HomePage } from '@/pages/HomePage';
import { InsightsPage } from '@/pages/InsightsPage';
import { MyGamesPage } from '@/pages/MyGamesPage';
import { SettingsPage } from '@/pages/SettingsPage';

const routes = [
  { path: '/', name: 'Home', component: HomePage },
  { path: '/my-games', name: 'MyGames', component: MyGamesPage },
  { path: '/analize-board', name: 'AnalizeBoard', component: AnalizeBoardPage },
  { path: '/insights', name: 'Insights', component: InsightsPage },
  { path: '/settings', name: 'Settings', component: SettingsPage },
];

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
});
