/** Inline SVG radar for Versus pentagon comparison (same layout as legacy VersusPage). */
export function buildVersusPentagonSvg(yVals: number[], oVals: number[]): string {
  const cx = 90;
  const cy = 90;
  const R = 72;
  const n = 5;
  const gridStroke = 'rgba(var(--v-theme-on-surface), 0.2)';
  const ringInner = 'rgba(var(--v-theme-on-surface), 0.12)';
  function pts(vals: number[]): [number, number][] {
    return vals.map((v, i) => {
      const a = (Math.PI * 2 * i) / n - Math.PI / 2;
      const rr = (v / 100) * R;
      return [cx + rr * Math.cos(a), cy + rr * Math.sin(a)];
    });
  }
  function path(ps: [number, number][]): string {
    return (
      ps
        .map((p, i) => `${i === 0 ? 'M' : 'L'}${p[0].toFixed(1)},${p[1].toFixed(1)}`)
        .join(' ') + ' Z'
    );
  }

  let inner = '';
  for (const pct of [25, 50, 75, 100]) {
    const gp = Array.from({ length: n }, (_, i) => {
      const a = (Math.PI * 2 * i) / n - Math.PI / 2;
      const rr = (pct / 100) * R;
      return [cx + rr * Math.cos(a), cy + rr * Math.sin(a)] as [number, number];
    });
    const stroke = pct === 100 ? gridStroke : ringInner;
    inner += `<path d="${path(gp)}" fill="none" stroke="${stroke}" stroke-width="0.5"/>`;
  }
  for (let i = 0; i < n; i++) {
    const a = (Math.PI * 2 * i) / n - Math.PI / 2;
    inner += `<line x1="${cx}" y1="${cy}" x2="${cx + R * Math.cos(a)}" y2="${cy + R * Math.sin(a)}" stroke="${ringInner}" stroke-width="0.5"/>`;
  }
  const op = pts(oVals);
  inner += `<path d="${path(op)}" fill="rgba(var(--v-theme-warning), 0.12)" stroke="rgb(var(--v-theme-warning))" stroke-width="1.5" stroke-dasharray="4,3"/>`;
  const yp = pts(yVals);
  inner += `<path d="${path(yp)}" fill="rgba(var(--v-theme-primary), 0.15)" stroke="rgb(var(--v-theme-primary))" stroke-width="2"/>`;
  for (const pt of yp) {
    inner += `<circle cx="${pt[0]}" cy="${pt[1]}" r="3" fill="rgb(var(--v-theme-primary))" stroke="rgb(var(--v-theme-surface))" stroke-width="1.5"/>`;
  }
  for (const pt of op) {
    inner += `<circle cx="${pt[0]}" cy="${pt[1]}" r="3" fill="rgb(var(--v-theme-warning))" stroke="rgb(var(--v-theme-surface))" stroke-width="1.5"/>`;
  }
  return `<svg width="180" height="180" viewBox="0 0 180 180">${inner}</svg>`;
}
