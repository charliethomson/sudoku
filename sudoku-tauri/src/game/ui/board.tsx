import { useMemo } from "react";
import { PuzzleFile } from "../../api/types/puzzle";
import Region, { RegionId } from "./square";

export type BoardProps = {
  puzzle: PuzzleFile;
};

const REGION_IDS = new Array(9).fill(0).map((_, i) => i) as RegionId[];

function Board({ puzzle }: BoardProps) {
  return (
    <div className="grid grid-cols-3 aspect-square w-fit bg-black gap-2 p-1">
      {REGION_IDS.map((rid) => {
        return <Region id={rid} key={rid} />;
      })}
    </div>
  );
}

export default Board;
