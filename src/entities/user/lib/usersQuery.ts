import type { UseQueryReturnType } from '@tanstack/vue-query';
import { useQuery } from '@tanstack/vue-query';
import { storeToRefs } from 'pinia';
import type { Ref } from 'vue';

import { useUserStore } from '../model/user.store';
import type { User } from '../model/user.types';

export function useGetUserQuery(): UseQueryReturnType<User, Error> & { user: Ref<User | null> } {
  const store = useUserStore();
  const { user } = storeToRefs(store);

  const query = useQuery<User>({
    queryKey: ['user'],

    queryFn: () => store.syncMe(),

    staleTime: 1000 * 60,
  });

  return {
    ...query,
    user,
  };
}
