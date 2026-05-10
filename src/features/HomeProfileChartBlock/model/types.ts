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
  player: PentagonDto | null;
  sampleSize: number;
}
