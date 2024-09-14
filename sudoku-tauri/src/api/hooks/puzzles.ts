import { useQuery } from "@tanstack/react-query";
import CoreApi from "../core/api";
import { createNamespacedKeyFactory } from "../common";
import { PuzzleFile } from "../types/puzzle";
import { AppError } from "../types/error";

const keyFactory = createNamespacedKeyFactory("puzzles");

export const puzzleQueryKeys = {
  get: keyFactory("get"),
};

export function usePuzzles() {
  return useQuery<PuzzleFile[], AppError<"config">>({
    queryKey: [puzzleQueryKeys.get()],
    queryFn: () => CoreApi.puzzles.list(undefined),
  });
}
