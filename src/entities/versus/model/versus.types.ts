import type { PentagonDto } from '@/entities/player-profile-chart';

export type { PentagonDto };

/** Full Lichess opening line within a merged family; frequencies sum to the family card `total`. */
export interface VersusOpeningLineShare {
  name: string;
  total: number;
  frequencyPct: number;
}

export interface VersusOpeningCard {
  name: string;
  wins: number;
  draws: number;
  losses: number;
  total: number;
  /** Versus frequent openings: (wins + 0.5 * draws) / total as a percentage; field name is historical. */
  winRatePct: number;
  /** Lines grouped under this family (from Tauri); empty if missing in older payloads. */
  lines?: VersusOpeningLineShare[];
}

export interface VersusSideSummary {
  username: string;
  rating: number | null;
  sampleSizeMetrics: number;
  pentagon: PentagonDto | null;
  benchmark: PentagonDto;
  avgAccuracyPct: number | null;
  avgAcpl: number | null;
  winRatePct: number | null;
  blundersPerGame: number | null;
  conversionPct: number | null;
  openingsAsWhite: VersusOpeningCard[];
  openingsAsBlack: VersusOpeningCard[];
}

export type VersusPlanReasonKind =
  | 'tier1GapPlay'
  | 'tier1GapAvoid'
  | 'tier2SelfBest'
  | 'tier2SelfWorst'
  | 'tier2OppWeak'
  | 'tier2OppStrong'
  | 'tier3SelfTop'
  | 'tier3SelfBottom';

export interface VersusPlanEntry {
  title: string;
  selectionTier: 1 | 2 | 3;
  reasonKind: VersusPlanReasonKind;
  reasonParams: Record<string, string | number>;
  tier: 'play' | 'avoid';
  selfWinRatePct?: number;
  oppWinRatePct?: number;
  selfGames: number;
  oppGames: number;
}

export interface VersusPlanSide {
  play: VersusPlanEntry[];
  avoid: VersusPlanEntry[];
}

export interface VersusGamePlan {
  oppGamesInOpeningSlice: number;
  asWhite: VersusPlanSide;
  asBlack: VersusPlanSide;
}

export interface VersusDiagnostics {
  opponentGamesMatchingSpeed: number;
  opponentAnalysesAttempted: number;
  opponentAnalysesSucceeded: number;
  sampleLichessSpeedsWhenNoMatch?: string[];
  firstAnalysisError?: string;
}

export interface VersusSpeedSlice {
  diagnostic: VersusDiagnostics;
  selfSide: VersusSideSummary;
  opponentSide: VersusSideSummary;
  gamePlan?: VersusGamePlan | null;
}

export interface VersusSlices {
  bullet: VersusSpeedSlice;
  blitz: VersusSpeedSlice;
  rapid: VersusSpeedSlice;
}

export interface VersusCompareResponse {
  opponentGamesInApiSample: number;
  slices: VersusSlices;
}

export interface VersusProgressPayload {
  phase: string;
  current: number;
  total: number;
}

