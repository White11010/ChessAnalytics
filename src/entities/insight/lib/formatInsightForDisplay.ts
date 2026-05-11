// Localized insight titles, summaries, recommendations, and metric labels from payloads and i18n keys.
import type { Insight } from '../model/insight.types';

import { mergeInsightDisplayParams, parseInsightPayload } from './insightPayload';

type TFn = (key: string, ...args: unknown[]) => string;
type TeFn = (key: string) => boolean;

function bool(p: Record<string, string | number | boolean>, k: string): boolean {
  return Boolean(p[k]);
}

function localizeSpeedInParams(p: Record<string, string | number | boolean>, t: TFn): void {
  if (typeof p.speed_label === 'string') {
    const s = p.speed_label.toLowerCase();
    const k = s === 'ultrabullet' ? 'bullet' : s;
    if (k === 'bullet' || k === 'blitz' || k === 'rapid' || k === 'classical') {
      p.speed_label = t(`myGames.speed.${k}`);
    }
  }
  if (typeof p.label === 'string') {
    const s = p.label.toLowerCase();
    const k = s === 'ultrabullet' ? 'bullet' : s;
    if (k === 'bullet' || k === 'blitz' || k === 'rapid' || k === 'classical') {
      p.label = t(`myGames.speed.${k}`);
    }
  }
}

function baseParams(insight: Insight, t: TFn): Record<string, string | number | boolean> {
  const { subject_key, params } = parseInsightPayload(insight.payload_json);
  const merged = mergeInsightDisplayParams(subject_key, params);
  localizeSpeedInParams(merged, t);
  return merged;
}

function kindKey(kind: string, part: 'title' | 'summary' | 'recommendation'): string {
  return `insights.kinds.${kind}.${part}`;
}

export function getInsightTitle(insight: Insight, t: TFn, te: TeFn): string {
  const p = baseParams(insight, t);
  const k = insight.kind;
  if (k === 'tactics_side_performance') {
    const key = bool(p, 'white_stronger')
      ? 'insights.kinds.tactics_side_performance.titleWhiteStronger'
      : 'insights.kinds.tactics_side_performance.titleBlackStronger';
    if (te(key)) {
      return t(key);
    }
  }
  if (k === 'opening_dependency') {
    const strength = String(p.strength ?? 'neutral');
    const key =
      strength === 'strength'
        ? 'insights.kinds.opening_dependency.titleStrength'
        : strength === 'risk'
          ? 'insights.kinds.opening_dependency.titleRisk'
          : 'insights.kinds.opening_dependency.titleNeutral';
    if (te(key)) {
      return t(key, p);
    }
  }
  const key = kindKey(k, 'title');
  if (te(key)) {
    return t(key, p);
  }
  return t('insights.fallback.title', { kind: k });
}

export function getInsightSummary(insight: Insight, t: TFn, te: TeFn): string {
  const p = baseParams(insight, t);
  const k = insight.kind;
  if (k === 'time_morning_vs_evening') {
    const morningBetter = bool(p, 'morning_better');
    const better = morningBetter ? t('insights.slots.morning') : t('insights.slots.evening');
    const worse = morningBetter ? t('insights.slots.evening') : t('insights.slots.morning');
    p.better = better;
    p.worse = worse;
    const tz =
      typeof p.timezone_note === 'string' && p.timezone_note
        ? p.timezone_note
        : typeof p.timezone_offset_minutes === 'number'
          ? String(p.timezone_offset_minutes)
          : '';
    p.timezone_caption = tz ? t('insights.kinds.time_morning_vs_evening.timezoneCaption', { tz }) : '';
    const key = morningBetter
      ? 'insights.kinds.time_morning_vs_evening.summaryMorningBetter'
      : 'insights.kinds.time_morning_vs_evening.summaryEveningBetter';
    if (te(key)) {
      return t(key, p);
    }
  }
  if (k === 'tactics_conversion_advantage') {
    const split = typeof p.speed_split === 'string' ? p.speed_split.trim() : '';
    if (split && te('insights.kinds.tactics_conversion_advantage.summaryWithSpeeds')) {
      p.speed_split = split;
      return t('insights.kinds.tactics_conversion_advantage.summaryWithSpeeds', p);
    }
  }
  if (k === 'opening_dependency') {
    const strength = String(p.strength ?? 'neutral');
    const sk =
      strength === 'strength'
        ? 'insights.kinds.opening_dependency.summaryStrength'
        : strength === 'risk'
          ? 'insights.kinds.opening_dependency.summaryRisk'
          : 'insights.kinds.opening_dependency.summaryNeutral';
    if (te(sk)) {
      return t(sk, p);
    }
  }
  if (k === 'opening_color_split') {
    const sc = String(p.stronger_color ?? '');
    const wc = String(p.weaker_color ?? '');
    p.stronger_side = sc === 'white' ? t('myGames.toolbar.colors.white') : t('myGames.toolbar.colors.black');
    p.weaker_side = wc === 'white' ? t('myGames.toolbar.colors.white') : t('myGames.toolbar.colors.black');
    if (te('insights.kinds.opening_color_split.summary')) {
      return t('insights.kinds.opening_color_split.summary', p);
    }
  }
  if (k === 'opening_trend') {
    if (te('insights.kinds.opening_trend.summary')) {
      return t('insights.kinds.opening_trend.summary', p);
    }
  }
  const key = kindKey(k, 'summary');
  if (te(key)) {
    return t(key, p);
  }
  return t('insights.fallback.summary', { kind: k });
}

export function getInsightRecommendation(insight: Insight, t: TFn, te: TeFn): string | null {
  const p = baseParams(insight, t);
  const k = insight.kind;
  if (k === 'tactics_side_performance') {
    const key = bool(p, 'white_stronger')
      ? 'insights.kinds.tactics_side_performance.recommendationWhite'
      : 'insights.kinds.tactics_side_performance.recommendationBlack';
    if (te(key)) {
      return t(key, p);
    }
  }
  if (k === 'opening_dependency') {
    const strength = String(p.strength ?? 'neutral');
    const rk =
      strength === 'strength'
        ? 'insights.kinds.opening_dependency.recommendationStrength'
        : strength === 'risk'
          ? 'insights.kinds.opening_dependency.recommendationRisk'
          : 'insights.kinds.opening_dependency.recommendationNeutral';
    if (te(rk)) {
      return t(rk, p);
    }
  }
  const key = kindKey(k, 'recommendation');
  if (te(key)) {
    return t(key, p);
  }
  return null;
}

export function getInsightMetricLabel(insight: Insight, t: TFn, te: TeFn): string | null {
  const k = insight.kind;
  const map: Record<string, string> = {
    opening_best: 'insights.metrics.winrate',
    opening_worst_frequent: 'insights.metrics.winrate',
    opening_rare_gem: 'insights.metrics.winrate',
    opening_dependency: 'insights.metrics.gameShare',
    opening_color_split: 'insights.metrics.winrateGapPp',
    opening_trend: 'insights.metrics.trendDeltaPp',
    time_control_best: 'insights.metrics.winrate',
    time_control_worst: 'insights.metrics.winrate',
    time_rating_growth_30d: 'insights.metrics.ratingDelta',
    time_morning_vs_evening: 'insights.metrics.winrateBestSlot',
    tactics_late_game_losses: 'insights.metrics.shareOfGames',
    tactics_side_performance: 'insights.metrics.winrateGapPp',
    tactics_middlegame_vs_endgame: 'insights.metrics.endgameErrorShare',
    tactics_conversion_advantage: 'insights.metrics.failedToWinPct',
    tactics_accuracy_by_phase: 'insights.metrics.weakestPhaseAccuracy',
    psychology_tilt: 'insights.metrics.winrate',
    psychology_comeback: 'insights.metrics.winrate',
    psychology_rest_effect: 'insights.metrics.accuracyDropPp',
    opponent_rating_performance: 'insights.metrics.winrateVsStronger',
    time_games_per_day_pattern: 'insights.metrics.winrateLightDays',
  };
  const labelKey = map[k];
  if (labelKey && te(labelKey)) {
    return t(labelKey);
  }
  return null;
}


// Resolves stored title, summary, and recommendation with i18n fallbacks for compact cards.

export function resolveInsightDisplayTitle(insight: Insight, t: TFn, te: TeFn): string {
  const raw = insight.title?.trim();
  if (raw) {
    return raw;
  }
  return getInsightTitle(insight, t, te);
}

export function resolveInsightDisplaySummary(insight: Insight, t: TFn, te: TeFn): string {
  const raw = insight.summary?.trim();
  if (raw) {
    return raw;
  }
  return getInsightSummary(insight, t, te);
}

export function resolveInsightDisplayRecommendation(insight: Insight, t: TFn, te: TeFn): string | null {
  const raw = insight.recommendation?.trim();
  if (raw) {
    return raw;
  }
  return getInsightRecommendation(insight, t, te);
}
