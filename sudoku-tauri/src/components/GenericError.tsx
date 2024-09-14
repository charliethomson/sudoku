import { AppError, isAppError, isConfigError } from "../api/types/error";

export type GenericErrorProps = {
  error: any;
};

function AppErrorView(error: AppError<string>) {
  if (isConfigError(error))
    return (
      <div
        id="error"
        className="border border-red-500 bg-red-200 dark:bg-red-800 text-red-800 dark:text-red-200 p-4 rounded-md flex flex-col gap-4"
      >
        <h1 className="text-2xl text-bold">Something went wrong!</h1>
        <div id="detail">
          <p>Kind: {error.kind}</p>
          <p>Variant: {error.error.variant}</p>
          <p>Message: {error.error.message}</p>
        </div>
      </div>
    );

  return (
    <div
      id="error"
      className="border border-red-500 bg-red-200 dark:bg-red-800 text-red-800 dark:text-red-200 p-4 rounded-md flex flex-col gap-4"
    >
      <h1 className="text-2xl text-bold">Something went wrong!</h1>
      <div id="detail">
        {Object.entries(error).map(([k, v]) => (
          <p key={k}>
            {k}: {JSON.stringify(v)}
          </p>
        ))}
      </div>
    </div>
  );
}

function GenericError({ error }: GenericErrorProps) {
  if (isAppError(error)) return AppErrorView(error);
  return (
    <div
      id="error"
      className="border border-red-500 bg-red-200 dark:bg-red-800 text-red-800 dark:text-red-200 p-4 rounded-md flex flex-col gap-4"
    >
      <h1 className="text-2xl text-bold">Something went wrong!</h1>
      <div id="detail">
        {Object.entries(error).map(([k, v]) => (
          <p key={k}>
            {k}: {JSON.stringify(v)}
          </p>
        ))}
      </div>
    </div>
  );
}
