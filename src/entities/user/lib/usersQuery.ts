import { invoke } from '@tauri-apps/api/core';
import type { UseQueryReturnType } from '@tanstack/vue-query';
import { useQuery } from '@tanstack/vue-query';
import { storeToRefs } from 'pinia';
import type { Ref } from 'vue';

import { useUserStore } from '../model/user.store';
import type { User } from '../model/user.types';

/** Loads active user from local SQLite (`get_me`) — no Lichess round-trip. */
export function useGetUserQuery(): UseQueryReturnType<User | null, Error> & {
  user: Ref<User | null>;
} {
  const store = useUserStore();
  const { user } = storeToRefs(store);

  const query = useQuery<User | null>({
    queryKey: ['user'],

    queryFn: async () => {
      const me = await invoke<User | null>('get_me');
      store.setUser(me);
      return me;
    },

    staleTime: 1000 * 60 * 60,
  });

  return {
    ...query,
    user,
  };
}
