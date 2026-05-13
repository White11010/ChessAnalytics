import { nextTick, ref, watch, type Ref } from 'vue';

import type { VersusSpeedSlice } from '../model/versus.types';

import { buildVersusPentagonSvg } from './versusPentagonSvg';

export function useVersusPentagonChart(
  activeSliceRef: Ref<VersusSpeedSlice | null>,
  axisLabelsRef: Ref<readonly string[]>,
) {
  const pentagonSvgHost = ref<HTMLElement | null>(null);

  watch(
    () => [activeSliceRef.value, axisLabelsRef.value] as const,
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
      const labels = axisLabelsRef.value;
      host.innerHTML = buildVersusPentagonSvg(yv, ov, labels.length === 5 ? labels : undefined);
    },
    { immediate: true, deep: true },
  );

  return { pentagonSvgHost };
}
