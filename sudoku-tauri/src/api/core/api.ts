import { invoke, InvokeArgs } from "@tauri-apps/api/tauri";
import { AppError, AppResult, err, unwrap } from "../types/error";
import { Config } from "../types/config";
import { PuzzleFile } from "../types/puzzle";

function command<
  TArgs extends InvokeArgs | undefined,
  TData,
  TErrorKind extends string = string,
>(
  command: string,
): (
  args: TArgs,
) => Promise<
  AppResult<TData, AppError<TErrorKind | "ipc">, TErrorKind | "ipc">
> {
  return async function (
    args: TArgs,
  ): Promise<
    AppResult<TData, AppError<TErrorKind | "ipc">, TErrorKind | "ipc">
  > {
    try {
      const result = await invoke<
        AppResult<TData, AppError<TErrorKind>, TErrorKind>
      >(command, args);

      console.log(result);

      return result;
    } catch {
      return err({
        kind: "ipc",
        error: { message: "Unable to communicate with host process" },
      });
    }
  };
}

function unwrapCommand<
  TArgs extends InvokeArgs | undefined,
  TData,
  TErrorKind extends string = string,
>(commandName: string): (args: TArgs) => Promise<TData> {
  const cmd = command<TArgs, TData, TErrorKind>(commandName);
  return async function (args: TArgs): Promise<TData> {
    let result = await cmd(args);
    // await new Promise((res) => setTimeout(res, 4000));
    return unwrap(result);
  };
}

const CoreApi = {
  config: {
    get: unwrapCommand<undefined, Config, "config">("get_config"),
  },
  puzzles: {
    list: unwrapCommand<undefined, PuzzleFile[], "config">("get_puzzles"),
    get: unwrapCommand<{ puzzle_id: number }, PuzzleFile, "config">(
      "get_puzzle",
    ),
  },
} as const;

export default CoreApi;
