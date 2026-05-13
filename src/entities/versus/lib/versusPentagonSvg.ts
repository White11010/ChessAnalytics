/** Inline SVG radar for Versus pentagon comparison (same vertex order as Home profile chart). */
function escapeSvgText(s: string): string {
  return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}

export function buildVersusPentagonSvg(
  yVals: number[],
  oVals: number[],
  axisLabels?: readonly string[],
): string {
  const vb = 400;
  const cx = 200;
  const cy = 200;
  const R = 108;
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
  inner += `<path d="${path(yp)}" fill="rgba(var(--v-theme-info), 0.15)" stroke="rgb(var(--v-theme-info))" stroke-width="2"/>`;
  for (const pt of yp) {
    inner += `<circle cx="${pt[0]}" cy="${pt[1]}" r="3" fill="rgb(var(--v-theme-info))" stroke="rgb(var(--v-theme-surface))" stroke-width="1.5"/>`;
  }
  for (const pt of op) {
    inner += `<circle cx="${pt[0]}" cy="${pt[1]}" r="3" fill="rgb(var(--v-theme-warning))" stroke="rgb(var(--v-theme-surface))" stroke-width="1.5"/>`;
  }

  if (axisLabels?.length === n) {
    const labelR = R + 22;
    const fill = 'rgba(var(--v-theme-on-surface), 0.75)';
    for (let i = 0; i < n; i++) {
      const a = (Math.PI * 2 * i) / n - Math.PI / 2;
      const lx = cx + labelR * Math.cos(a);
      const ly = cy + labelR * Math.sin(a);
      inner += `<text x="${lx.toFixed(1)}" y="${ly.toFixed(1)}" text-anchor="middle" dominant-baseline="middle" fill="${fill}" font-size="14" font-weight="500" font-family="inherit">${escapeSvgText(axisLabels[i] ?? '')}</text>`;
    }
  }

  return `<svg width="100%" height="100%" viewBox="0 0 ${vb} ${vb}" preserveAspectRatio="xMidYMid meet">${inner}</svg>`;
}
