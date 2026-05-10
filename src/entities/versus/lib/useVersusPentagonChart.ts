import { nextTick, ref, watch, type Ref } from 'vue';

import type { VersusSpeedSlice } from '../model/versus.types';

import { buildVersusPentagonSvg } from './versusPentagonSvg';

export function useVersusPentagonChart(activeSliceRef: Ref<VersusSpeedSlice | null>) {
  const pentagonSvgHost = ref<HTMLElement | null>(null);

  watch(
    () => activeSliceRef.value,
    async () => {
      await nextTick();
      const host = pentagonSvgHost.value;
      const slice = activeSliceRef.value;
      if (!host || !slice) {
        return;
      }
      const py = slice.selfSide.pentagon;
      const po = slice.opponentSide.pentagon;
      const yv = py
        ? [py.accuracy, py.stability, py.conversion ?? 50, py.openings, py.endgame]
        : [0, 0, 0, 0, 0];
      const ov = po
        ? [po.accuracy, po.stability, po.conversion ?? 50, po.openings, po.endgame]
        : [0, 0, 0, 0, 0];
      host.innerHTML = buildVersusPentagonSvg(yv, ov);
    },
    { immediate: true, deep: true },
  );

  return { pentagonSvgHost };
}
