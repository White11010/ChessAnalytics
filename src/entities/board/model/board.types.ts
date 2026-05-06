export interface MoveItem {
  san: string;
  fen: string;
}
export interface Board {
  history: MoveItem[];
  currentIndex: number;
}
