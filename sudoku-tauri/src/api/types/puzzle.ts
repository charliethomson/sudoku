import { DateString } from ".";

export type Marks = [
  boolean,
  boolean,
  boolean,
  boolean,
  boolean,
  boolean,
  boolean,
  boolean,
  boolean,
];

export type PuzzleBoard = number[];
export type PuzzleMarks = Marks[];

export type PuzzleSchemaSource = "Nyt";
export type PuzzleDifficulty = "Easy" | "Medium" | "Hard";
export type PuzzleVersion = number;
export type PuzzleId = { id_type: "Unknown" } | { id_type: "Id"; id: number };
export type PuzzleMetaSchema = {
  Nil: {};
  v1: {
    source: PuzzleSchemaSource;
    publish_date: DateString;
    load_time: DateString;
    id: PuzzleId;
    difficulty: PuzzleDifficulty;
    puzzle_version: PuzzleVersion;
    name: string;
    slug: string;
  };
};

export type PuzzleMetaVersion = keyof PuzzleMetaSchema;

export type PuzzleMeta<Version extends PuzzleMetaVersion = "Nil"> = {
  version: Version;
} & PuzzleMetaSchema[Version];

export type PuzzleFile<Version extends PuzzleMetaVersion = "v1"> = {
  header: [number, number];
  initial_state: PuzzleBoard;
  solved_state: PuzzleBoard;
  center_marks: PuzzleMarks;
  pencil_marks: PuzzleMarks;
  meta: PuzzleMeta<Version>;
};
