import { createApp } from 'vue';

import App from '@/app/App.vue';
import { initTheme, removePreloader } from '@/app/init';
import { VueApexCharts } from '@/app/providers/charts';
import { queryClient, VueQueryPlugin } from '@/app/providers/query';
import { router } from '@/app/providers/router';
import { pinia } from '@/app/providers/store';
import { vuetify } from '@/app/providers/vuetify';
import 'chessground/assets/chessground.base.css';
import 'chessground/assets/chessground.brown.css';
import 'chessground/assets/chessground.cburnett.css';

const app = createApp(App);

app
  .use(pinia)
  .use(router)
  .use(vuetify)
  .use(VueQueryPlugin, {
    queryClient,
  })
  .use(VueApexCharts);

initTheme(vuetify);

app.mount('#app');

removePreloader();
