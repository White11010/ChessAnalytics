import type { PentagonDto, VersusSideSummary } from '../model/versus.types';

export type ConclusionLine = { good: boolean; text: string };

type TFn = (key: string, params?: Record<string, unknown>) => string;

export function buildConclusionsYou(
  sy: VersusSideSummary,
  so: VersusSideSummary,
  t: TFn,
): ConclusionLine[] {
  const pentY = sy.pentagon;
  const pentO = so.pentagon;
  const lines: ConclusionLine[] = [];
  const ya = sy.avgAccuracyPct;
  const oa = so.avgAccuracyPct;
  if (ya != null && oa != null) {
    if (ya >= oa) {
      lines.push({
        good: true,
        text: String(
          t('versusPage.conclusionAccHigh', { you: ya.toFixed(0), opp: oa.toFixed(0) }),
        ),
      });
    } else {
      lines.push({
        good: false,
        text: String(
          t('versusPage.conclusionAccLow', { you: ya.toFixed(0), opp: oa.toFixed(0) }),
        ),
      });
    }
  }
  const yAc = sy.avgAcpl;
  const oAc = so.avgAcpl;
  if (yAc != null && oAc != null) {
    if (yAc <= oAc) {
      lines.push({
        good: true,
        text: String(
          t('versusPage.conclusionAcplLow', { you: yAc.toFixed(1), opp: oAc.toFixed(1) }),
        ),
      });
    } else {
      lines.push({
        good: false,
        text: String(
          t('versusPage.conclusionAcplHigh', { you: yAc.toFixed(1), opp: oAc.toFixed(1) }),
        ),
      });
    }
  }
  const yb = sy.blundersPerGame;
  const ob = so.blundersPerGame;
  if (yb != null && ob != null) {
    if (yb <= ob) {
      lines.push({
        good: true,
        text: String(
          t('versusPage.conclusionBlundersLow', { you: yb.toFixed(1), opp: ob.toFixed(1) }),
        ),
      });
    } else {
      lines.push({
        good: false,
        text: String(
          t('versusPage.conclusionBlundersHigh', {
            you: yb.toFixed(1),
            opp: ob.toFixed(1),
          }),
        ),
      });
    }
  }
  if (pentY && pentO && pentY.endgame > pentO.endgame + 3) {
    lines.push({
      good: true,
      text: String(
        t('versusPage.conclusionEndHigh', {
          you: pentY.endgame.toFixed(0),
          opp: pentO.endgame.toFixed(0),
        }),
      ),
    });
  } else if (pentY && pentO && pentY.endgame + 3 < pentO.endgame) {
    lines.push({
      good: false,
      text: String(
        t('versusPage.conclusionEndHigh', {
          you: pentY.endgame.toFixed(0),
          opp: pentO.endgame.toFixed(0),
        }),
      ),
    });
  }
  if (pentY && pentO && pentY.openings > pentO.openings + 3) {
    lines.push({
      good: true,
      text: String(
        t('versusPage.conclusionOpenHigh', {
          you: pentY.openings.toFixed(0),
          opp: pentO.openings.toFixed(0),
        }),
      ),
    });
  }
  return lines.slice(0, 6);
}

export function buildConclusionsOpp(
  sy: VersusSideSummary,
  so: VersusSideSummary,
  t: TFn,
): ConclusionLine[] {
  const pentY = sy.pentagon;
  const pentO = so.pentagon;
  const lines: ConclusionLine[] = [];
  const ya = sy.avgAccuracyPct;
  const oa = so.avgAccuracyPct;
  if (ya != null && oa != null) {
    if (oa >= ya) {
      lines.push({
        good: true,
        text: String(
          t('versusPage.conclusionAccHigh', { you: oa.toFixed(0), opp: ya.toFixed(0) }),
        ),
      });
    } else {
      lines.push({
        good: false,
        text: String(
          t('versusPage.conclusionAccLow', { you: oa.toFixed(0), opp: ya.toFixed(0) }),
        ),
      });
    }
  }
  const yAc = sy.avgAcpl;
  const oAc = so.avgAcpl;
  if (yAc != null && oAc != null) {
    if (oAc <= yAc) {
      lines.push({
        good: true,
        text: String(
          t('versusPage.conclusionAcplLow', { you: oAc.toFixed(1), opp: yAc.toFixed(1) }),
        ),
      });
    } else {
      lines.push({
        good: false,
        text: String(
          t('versusPage.conclusionAcplHigh', { you: oAc.toFixed(1), opp: yAc.toFixed(1) }),
        ),
      });
    }
  }
  const yb = sy.blundersPerGame;
  const ob = so.blundersPerGame;
  if (yb != null && ob != null) {
    if (ob <= yb) {
      lines.push({
        good: true,
        text: String(
          t('versusPage.conclusionBlundersLow', { you: ob.toFixed(1), opp: yb.toFixed(1) }),
        ),
      });
    } else {
      lines.push({
        good: false,
        text: String(
          t('versusPage.conclusionBlundersHigh', {
            you: ob.toFixed(1),
            opp: yb.toFixed(1),
          }),
        ),
      });
    }
  }
  if (pentY && pentO && pentO.endgame > pentY.endgame + 3) {
    lines.push({
      good: true,
      text: String(
        t('versusPage.conclusionEndHigh', {
          you: pentO.endgame.toFixed(0),
          opp: pentY.endgame.toFixed(0),
        }),
      ),
    });
  } else if (pentY && pentO && pentO.endgame + 3 < pentY.endgame) {
    lines.push({
      good: false,
      text: String(
        t('versusPage.conclusionEndHigh', {
          you: pentO.endgame.toFixed(0),
          opp: pentY.endgame.toFixed(0),
        }),
      ),
    });
  }
  if (pentY && pentO && pentO.openings > pentY.openings + 3) {
    lines.push({
      good: true,
      text: String(
        t('versusPage.conclusionOpenHigh', {
          you: pentO.openings.toFixed(0),
          opp: pentY.openings.toFixed(0),
        }),
      ),
    });
  }
  return lines.slice(0, 6);
}

export function pentagonAxisNumber(
  p: PentagonDto | null,
  axis: keyof PentagonDto,
): number {
  if (!p) return 0;
  const v = p[axis];
  return typeof v === 'number' && Number.isFinite(v) ? v : 0;
}
