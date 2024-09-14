import { PuzzleDifficulty } from "../api/types/puzzle";

import StarFill from "mingcute_icon/svg/other/star_2_fill.svg?react";
import StarLine from "mingcute_icon/svg/other/star_2_line.svg?react";

export type DifficultyProps = {
  difficulty: PuzzleDifficulty;
};

function Difficulty({ difficulty }: DifficultyProps) {
  const i = (
    {
      Easy: 1,
      Medium: 2,
      Hard: 3,
    } as const
  )[difficulty];

  const j = 3 - i;

  return (
    <div className="flex gap-1">
      {new Array(i).fill(0).map((_, k) => (
        <StarFill
          key={k}
          className="_mgc text-yellow-600 dark:text-yellow-400"
        />
      ))}
      {new Array(j).fill(0).map((_, k) => (
        <StarLine
          key={k}
          className="_mgc text-yellow-600 dark:text-yellow-400"
        />
      ))}
    </div>
  );
}

export default Difficulty;
