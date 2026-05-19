/** Tauri `get_player_profile_chart` payload shape; used by home chart, insights KPIs, etc. */
export interface PentagonDto {
  accuracy: number;
  stability: number;
  conversion: number | null;
  openings: number;
  endgame: number;
}

export interface PlayerProfileChartResponse {
  speed: string;
  ratingUsed: number | null;
  bucketLabel: string;
  benchmark: PentagonDto;
  benchmarkAcplAvg: number;
  player: PentagonDto | null;
  sampleSize: number;
}
