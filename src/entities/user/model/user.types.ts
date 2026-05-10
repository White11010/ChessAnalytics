export interface User {
  username: string;
  id: string;
  /** API field names match Tauri / Lichess-backed `User` (snake_case). */
  platform?: string;

  bullet_rating?: number | null;
  bullet_games?: number | null;
  blitz_rating?: number | null;
  blitz_games?: number | null;
  rapid_rating?: number | null;
  rapid_games?: number | null;
  classical_rating?: number | null;
  classical_games?: number | null;

  /** Lichess export cursor; optional for older clients. */
  lichess_since_cursor_ms?: number | null;
  /** When games sync last completed successfully (epoch ms). */
  last_sync_completed_at_ms?: number | null;
}
