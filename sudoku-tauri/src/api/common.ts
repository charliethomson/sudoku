export type NamespacedKey<
  TNamespace extends string,
  TKey extends string,
  TContext,
> = (cx?: TContext) => {
  ns: TNamespace;
  ky: TKey;
  id: `${TNamespace}:${TKey}`;
  cx?: TContext;
};

export type NamespacedKeyFactory<TNamespace extends string> = <
  TContext,
  TKey extends string = string,
>(
  key: TKey,
) => NamespacedKey<TNamespace, TKey, TContext>;

export function createNamespacedKeyFactory<TNamespace extends string>(
  namespace: TNamespace,
): NamespacedKeyFactory<TNamespace> {
  return function <TContext, TKey extends string>(key: TKey) {
    return (cx?: TContext) => ({
      ns: namespace,
      ky: key,
      id: `${namespace}:${key}`,
      cx,
    });
  };
}
