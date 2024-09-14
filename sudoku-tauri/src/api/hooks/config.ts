import { useMutation, useQuery } from "@tanstack/react-query";
import CoreApi from "../core/api";
import { createNamespacedKeyFactory } from "../common";

const keyFactory = createNamespacedKeyFactory("config");

export const configQueryKeys = {
  get: keyFactory("get"),
};

export function useConfig() {
  return useQuery({
    queryKey: [configQueryKeys.get()],
    queryFn: CoreApi.config.get,
  });
}
