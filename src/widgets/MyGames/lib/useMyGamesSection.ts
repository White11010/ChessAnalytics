import { computed, watch } from 'vue';

import {
  filterMyGames,
  patternTagOptions,
  uniqueOpeningOptions,
  useGamesStore,
  useMyGamesFiltersStore,
  useSyncGamesQuery,
} from '@/entities/game';

export function useMyGamesSection() {
  useSyncGamesQuery();

  const gamesStore = useGamesStore();
  const filtersStore = useMyGamesFiltersStore();

  watch(
    () => filtersStore.$state,
    () => {
      filtersStore.persist();
    },
    { deep: true },
  );

  const patternOptions = computed(() => patternTagOptions(gamesStore.games));
  const openingOptions = computed(() => uniqueOpeningOptions(gamesStore.games));

  const displayedGames = computed(() => {
    const filtered = filterMyGames(gamesStore.games, {
      searchText: filtersStore.searchText,
      results: filtersStore.results,
      speeds: filtersStore.speeds,
      periods: filtersStore.periods,
      patternTag: filtersStore.patternTag,
      openingValue: filtersStore.openingValue,
      openingNameExact: filtersStore.openingNameExact,
      playerColors: filtersStore.playerColors,
    });
    return filtered;
  });

  const showResultCount = computed(() => {
    const total = gamesStore.games.length;
    if (!total) {
      return false;
    }
    const filteredLen = displayedGames.value.length;
    const hasSearch = filtersStore.searchText.trim() !== '';
    const hasChipFilters =
      filtersStore.results.length > 0 ||
      filtersStore.speeds.length > 0 ||
      filtersStore.periods.length > 0 ||
      filtersStore.patternTag != null ||
      filtersStore.openingValue != null ||
      filtersStore.openingNameExact != null ||
      filtersStore.playerColors.length > 0;
    return hasSearch || hasChipFilters || filteredLen !== total;
  });

  return {
    gamesStore,
    patternOptions,
    openingOptions,
    displayedGames,
    showResultCount,
  };
}
