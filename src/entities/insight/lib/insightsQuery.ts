import type { UseQueryReturnType } from '@tanstack/vue-query';
import { useQuery } from '@tanstack/vue-query';
import { storeToRefs } from 'pinia';
import { computed, type MaybeRef, type Ref, toValue } from 'vue';

import { useInsightsStore } from '../model/insight.store';
import type { Insight } from '../model/insight.types';

export function useRegenerateInsightsQuery(enabled: MaybeRef<boolean>): UseQueryReturnType<
  Insight[],
  Error
> & {
  insights: Ref<Insight[]>;
} {
  const store = useInsightsStore();
  const { items } = storeToRefs(store);

  const query = useQuery<Insight[]>({
    queryKey: ['insights'],

    queryFn: () => store.regenerate(),

    staleTime: 1000 * 60,

    enabled: computed(() => toValue(enabled)),
  });

  return {
    ...query,
    insights: items,
  };
}
