import type { KeyMoment } from '../model/gameAnalysis.types';

type TFn = (key: string, ...args: unknown[]) => string;
type TeFn = (key: string) => boolean;

export function formatSwingCp(value: number): string {
  const sign = value > 0 ? '+' : '';
  return `${sign}${value}`;
}

function kindLabel(moment: KeyMoment, t: TFn, te: TeFn): string {
  const kindKey = `analysis.moveKind.${moment.kind}`;
  return te(kindKey) ? t(kindKey) : moment.kind;
}

function pawnsFromSwing(swingCp: number): string {
  return (Math.abs(swingCp) / 100).toFixed(1);
}

function formatEvalPawns(cp: number): string {
  const pawns = cp / 100;
  const sign = pawns > 0 ? '+' : '';
  return `${sign}${pawns.toFixed(1)}`;
}

export function getKeyMomentHeadline(moment: KeyMoment, t: TFn, te: TeFn): string {
  const key = 'analysis.keyMoment.headlineWithSwing';
  if (te(key)) {
    return t(key, {
      move: moment.move_number,
      kind: kindLabel(moment, t, te),
      swing: formatSwingCp(moment.swing_cp),
    });
  }
  return `${moment.move_number} — ${kindLabel(moment, t, te)}`;
}

export function getKeyMomentDescription(moment: KeyMoment, t: TFn, te: TeFn): string {
  const pawns = pawnsFromSwing(moment.swing_cp);

  if (moment.swing_cp <= -200) {
    const k = 'analysis.keyMoment.descThrewWinning';
    if (te(k)) {
      return t(k, { pawns });
    }
  }
  if (moment.swing_cp < -30) {
    const k = 'analysis.keyMoment.descLostPawns';
    if (te(k)) {
      return t(k, { pawns });
    }
  }
  if (moment.kind === 'brilliant') {
    const k = 'analysis.keyMoment.descBrilliant';
    if (te(k)) {
      return t(k);
    }
  }
  if (moment.swing_cp !== 0) {
    const k = 'analysis.keyMoment.descSwingCp';
    if (te(k)) {
      return t(k, { swing: formatSwingCp(moment.swing_cp), pawns });
    }
  }
  return '';
}

export function getKeyMomentMovesLine(moment: KeyMoment, t: TFn, te: TeFn): string {
  if (moment.best_move_san) {
    const k = 'analysis.keyMoment.movesLine';
    if (te(k)) {
      return t(k, { played: moment.move_san, best: moment.best_move_san });
    }
  }
  const k = 'analysis.keyMoment.movesLinePlayedOnly';
  if (te(k)) {
    return t(k, { played: moment.move_san });
  }
  return moment.move_san;
}

export function getKeyMomentEvalLine(moment: KeyMoment, t: TFn, te: TeFn): string | null {
  if (!Number.isFinite(moment.eval_before) || !Number.isFinite(moment.eval_after)) {
    return null;
  }
  const k = 'analysis.keyMoment.evalBeforeToAfter';
  if (!te(k)) {
    return null;
  }
  return t(k, {
    before: formatEvalPawns(moment.eval_before),
    after: formatEvalPawns(moment.eval_after),
  });
}
