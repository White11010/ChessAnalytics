import type { PentagonDto } from '@/entities/player-profile-chart';

export type { PentagonDto };

export interface VersusOpeningCard {
  name: string;
  wins: number;
  draws: number;
  losses: number;
  total: number;
  /** Versus frequent openings: (wins + 0.5 * draws) / total as a percentage; field name is historical. */
  winRatePct: number;
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

export interface VersusPlanEntry {
  title: string;
  subtitle: string;
  tier: string;
}

export interface VersusPlanSide {
  attack: VersusPlanEntry[];
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

