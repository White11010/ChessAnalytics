export interface PentagonDto {
  accuracy: number;
  stability: number;
  conversion: number | null;
  openings: number;
  endgame: number;
}

export interface VersusOpeningCard {
  name: string;
  wins: number;
  total: number;
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
  openings: VersusOpeningCard[];
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
