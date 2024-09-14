import BoardCell from "./cell";

export type RegionId = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9;

export type RegionProps = {
  id: RegionId;
};

const OFFSETS = [0, 1, 2, 9, 10, 11, 18, 19, 20];
const BASES = [0, 3, 6, 27, 30, 33, 54, 57, 60];

function Region({ id }: RegionProps) {
  const base = BASES[id];

  return (
    <div className="grid grid-cols-3 aspect-square w-fit gap-1">
      {OFFSETS.map((offset) => {
        const cellId = base + offset;

        const mode = ["value", "pencilMarks", "centerMarks"][cellId % 3];
        const value = [
          offset,
          new Array(9).fill(1).map((_, i) => i % 2 === 0),
          new Array(9).fill(1).map((_, i) => i % 2 === 1),
        ][cellId % 3];

        return (
          <BoardCell key={cellId} mode={mode as any} context={value as any} />
        );
      })}
    </div>
  );
}

export default Region;
