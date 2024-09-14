export type ConfigError = {
  variant: string;
  message: string;
};

type ErrorKinds = {
  config: ConfigError;
  ipc: {
    message: "Unable to communicate with host process";
  };
  unknown: Record<any, any>;
};

export type ErrorKind = keyof ErrorKinds;

export type AppError<TErrorKind extends string> = {
  kind: TErrorKind;
  error: TErrorKind extends ErrorKind ? ErrorKinds[TErrorKind] : never;
};

export function isAppError(error: any): error is AppError<string> {
  return !!error && "kind" in error && "error" in error;
}

type OkResult<TData> = {
  ok: true;
  data: TData;
};

type ErrResult<
  TError extends AppError<TErrorKind>,
  TErrorKind extends string,
> = {
  ok: false;
  error: TError;
};

export type AppResult<
  TData,
  TError extends AppError<TErrorKind>,
  TErrorKind extends string = string,
> = OkResult<TData> | ErrResult<TError, TErrorKind>;

export function ok<TData>(data: TData): AppResult<TData, any> {
  return {
    ok: true,
    data,
  };
}

export function err<
  TError extends AppError<TErrorKind>,
  TErrorKind extends string = string,
>(error: TError): AppResult<any, TError, TErrorKind> {
  return {
    ok: false,
    error,
  };
}

export function unwrap<
  TResult extends AppResult<TData, TError, TErrorKind>,
  TData,
  TError extends AppError<TErrorKind>,
  TErrorKind extends string = string,
>(result: TResult): TData {
  if (!result.ok) throw result.error;
  return result.data;
}

export function expect<
  TResult extends AppResult<TData, TError, TErrorKind>,
  TData,
  TError extends AppError<TErrorKind>,
  TErrorKind extends string = string,
>(result: TResult, message: string): TData {
  if (!result.ok) throw [message, result.error];
  return result.data;
}

export function match<
  TResult extends AppResult<TData, TError, TErrorKind>,
  TData,
  TError extends AppError<TErrorKind>,
  TErrorKind extends string = string,
>(result: TResult, ok: (data: TData) => void, err: (err: TError) => void) {
  if (!result.ok) err(result.error);
  else ok(result.data);
}

export function map<
  TResult extends AppResult<TData, TError, TErrorKind>,
  TData,
  TRData,
  TError extends AppError<TErrorKind>,
  TErrorKind extends string = string,
>(
  result: TResult,
  mapper: (data: TData) => TRData,
): AppResult<TRData, TError, TErrorKind> {
  if (!result.ok) return result;
  return ok(mapper(result.data));
}

export function isConfigError(
  error: AppError<string>,
): error is AppError<"config"> {
  return error.kind === "config";
}
