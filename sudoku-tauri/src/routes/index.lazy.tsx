import { createLazyFileRoute, Link } from "@tanstack/react-router";
import { usePuzzles } from "../api/hooks/puzzles";
import Spinner from "../components/Spinner";
import SourceBadge from "../components/SourceBadge";

import ChevronRight from "mingcute_icon/svg/arrow/right_line.svg?react";
import Badge from "../components/Badge";
import "../components/MingCuteIcon.css";
import Difficulty from "../components/Difficulty";

export const Route = createLazyFileRoute("/")({
  component: Index,
});

function Index() {
  const puzzles = usePuzzles();

  if (puzzles.isError && puzzles.error) {
    return (
      <div
        id="error"
        className="border border-red-500 bg-red-200 dark:bg-red-800 text-red-800 dark:text-red-200 p-4 rounded-md flex flex-col gap-4"
      >
        <h1 className="text-2xl text-bold">Something went wrong!</h1>
        <div id="detail">
          <p>Kind: {puzzles.error.kind}</p>
          <p>Variant: {puzzles.error.error.variant}</p>
          <p>Message: {puzzles.error.error.message}</p>
        </div>
      </div>
    );
  }

  if (puzzles.isFetching) {
    return (
      <div
        id="loader"
        className="flex w-full h-full items-center justify-center"
      >
        <div className="flex flex-col items-center gap-12">
          <span className="text-lg">Loading puzzles...</span>
          <Spinner />
        </div>
      </div>
    );
  }

  return (
    <div className="">
      <ul className="flex flex-col gap-16">
        {puzzles.data?.map((puzzle) => {
          return (
            <li key={puzzle.meta.slug} className="flex flex-col gap-12">
              <div className="flex gap-2 items-center justify-between">
                <span className="text-2xl">{puzzle.meta.name}</span>

                <Difficulty difficulty={puzzle.meta.difficulty} />
              </div>
              <div className="flex items-center gap-2 justify-between">
                <SourceBadge source={puzzle.meta.source} />

                <Link
                  to="/puzzle/$puzzleId"
                  params={{ puzzleId: puzzle.meta.slug }}
                  className="w-fit"
                >
                  <Badge
                    EndIcon={<ChevronRight className="_mgc" />}
                    label="Play"
                  />
                </Link>
              </div>
            </li>
          );
        })}
      </ul>
    </div>
  );
}
