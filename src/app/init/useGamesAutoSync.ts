import { storeToRefs } from 'pinia';
import { ref, watch } from 'vue';
import { useRoute } from 'vuetify/lib/composables/router.mjs';

import { useGamesSyncStore } from '@/entities/games-sync';
import { useUserStore } from '@/entities/user';

/**
 * Auto games sync: app launch (15 min), Home / MyGames navigation (30 min).
 * First paint: only 15‑minute rule (avoid conflicting with 30‑minute rule on same screen).
 */
export function useGamesAutoSync(): void {
  const userStore = useUserStore();
  const { user } = storeToRefs(userStore);
  const syncStore = useGamesSyncStore();
  const route = useRoute();

  /** Skip one route-based auto check right after user becomes available (launch rule handles it). */
  const suppressRouteAutoOnce = ref(true);

  watch(
    user,
    (u) => {
      if (!u) {
        syncStore.reset();
        suppressRouteAutoOnce.value = true;
        return;
      }
      syncStore.hydrateFromUser(u.last_sync_completed_at_ms ?? null);
      syncStore.scheduleAutoSync(15, 'app_launch');
      suppressRouteAutoOnce.value = true;
    },
    { immediate: true },
  );

  watch(
    () => [user.value, route.value?.name] as const,
    () => {
      if (!user.value) {
        return;
      }
      if (suppressRouteAutoOnce.value) {
        suppressRouteAutoOnce.value = false;
        return;
      }
      const name = route.value?.name;
      if (name === 'Home' || name === 'MyGames') {
        syncStore.scheduleAutoSync(30, `route:${String(name)}`);
      }
    },
  );
}
