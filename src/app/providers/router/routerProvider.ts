import { createMemoryHistory, createRouter } from 'vue-router';

import { AnalizeBoardPage } from '@/pages/AnalizeBoardPage';
import { HomePage } from '@/pages/HomePage';
import { InsightsPage } from '@/pages/InsightsPage';
import { MyGamesPage } from '@/pages/MyGamesPage';
import { SettingsPage } from '@/pages/SettingsPage';
import { GameDetailsPage } from '@/pages/GameDetailsPage';
import { VersusPage } from '@/pages/VersusPage';

const routes = [
  { path: '/', name: 'Home', component: HomePage },
  { path: '/versus', name: 'Versus', component: VersusPage },
  { path: '/my-games', name: 'MyGames', component: MyGamesPage },
  { path: '/analize-board', name: 'AnalizeBoard', component: AnalizeBoardPage },
  { path: '/insights', name: 'Insights', component: InsightsPage },
  { path: '/settings', name: 'Settings', component: SettingsPage },
  { path: '/game-details/:id', name: 'GameDetails', component: GameDetailsPage },
];

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
});
