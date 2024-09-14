import { Marks } from "../../api/types/puzzle";

export type BoardCellContext = {
  value: number;
  centerMarks: Marks;
  pencilMarks: Marks;
};
export type BoardCellMode = keyof BoardCellContext;

export type BoardCellProps<TMode extends BoardCellMode> = {
  mode: TMode;
  context: BoardCellContext[TMode];
};

function BoardCellValue({ value }: { value: number }) {
  return (
    <div className="flex items-center justify-center size-full">
      <h1 className="text-xl">{value}</h1>
    </div>
  );
}
function BoardCellCenterMarks({ marks }: { marks: Marks }) {
  return (
    <div className="flex items-center justify-center size-full">
      <p>{marks.map((enabled, i) => (enabled ? i.toString() : "")).join("")}</p>
    </div>
  );
}
function BoardCellPencilMarks({ marks }: { marks: Marks }) {
  return (
    <div className="grid grid-cols-3 size-full justify-center items-center">
      {marks.map((enabled, i) =>
        enabled ? <span className="place-self-center">{i + 1}</span> : <span />,
      )}
    </div>
  );
}

function BoardCell<TMode extends BoardCellMode>({
  mode,
  context,
}: BoardCellProps<TMode>) {
  const content = {
    value: <BoardCellValue value={context as any} />,
    centerMarks: <BoardCellCenterMarks marks={context as any} />,
    pencilMarks: <BoardCellPencilMarks marks={context as any} />,
  }[mode];

  return <div className="size-20 relative bg-slate-500">{content}</div>;
}

export default BoardCell;
