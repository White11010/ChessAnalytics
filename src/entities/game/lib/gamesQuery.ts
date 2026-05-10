import type { UseQueryReturnType } from '@tanstack/vue-query';
import { useQuery } from '@tanstack/vue-query';
import { storeToRefs } from 'pinia';
import type { Ref } from 'vue';

import { useGamesStore } from '../model/game.store';
import type { Game } from '../model/games.types';

export function useSyncGamesQuery(): UseQueryReturnType<Game[], Error> & { games: Ref<Game[]> } {
  const store = useGamesStore();
  const { games } = storeToRefs(store);

  const query = useQuery<Game[]>({
    queryKey: ['games'],

    queryFn: async () => {
      await store.loadFromDb();
      return [...store.games];
    },

    staleTime: 1000 * 60 * 60,
  });

  return {
    ...query,
    games,
  };
}
