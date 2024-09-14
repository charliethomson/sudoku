import { PuzzleFile } from "../api/types/puzzle";
import Board from "./ui/board";

export type GameViewProps = {
  puzzle: PuzzleFile;
};

function GameView({ puzzle }: GameViewProps) {
  return (
    <div>
      <Board puzzle={puzzle} />
    </div>
  );
}

export default GameView;
