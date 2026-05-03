export interface Game {
  id: string;

  username: string;
  platform: 'Lichess' | 'Chess.com';

  rated: boolean;
  speed: string;
  time_control: string;
  created_at: number;

  player_name: string;
  player_id: string;

  opponent_name: string;
  opponent_id: string;

  white_name: string;
  white_id: string;

  black_name: string;
  black_id: string;

  white_rating: number | null;
  black_rating: number | null;

  player_rating: number | null;
  opponent_rating: number | null;

  winner: 'white' | 'black' | null;
  player_color: 'white' | 'black';
  player_result: 'win' | 'loss' | 'draw';

  opening_eco: string | null;
  opening_name: string | null;

  moves: string | null;
  last_fen: string | null;

  pgn: string;
}
