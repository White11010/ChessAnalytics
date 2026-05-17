import type { PentagonDto } from '../model/versus.types';

export function pentagonAxisNumber(p: PentagonDto | null, axis: keyof PentagonDto): number {
  if (!p) return 0;
  const v = p[axis];
  return typeof v === 'number' && Number.isFinite(v) ? v : 0;
}
