import { createLazyFileRoute, Link } from "@tanstack/react-router";
import { usePuzzles } from "../api/hooks/puzzles";
import GameView from "../game/main";

export const Route = createLazyFileRoute("/puzzle/$puzzleId")({
  component: Puzzle,
});

function NotFound() {
  return (
    <div>
      <p>Puzzle not found</p>
      <Link to="/"> Go Back </Link>
    </div>
  );
}

function Puzzle() {
  const params = Route.useParams();
  const puzzles = usePuzzles();
  if (!puzzles.isSuccess) return null;

  let puzzle = puzzles.data.find(
    (puzzle) => puzzle.meta.slug === params.puzzleId,
  );
  if (!puzzle) return NotFound();

  return <GameView puzzle={puzzle} />;
}
