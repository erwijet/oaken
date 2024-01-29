import { deep } from "@tsly/deep";
import { clsx, type ClassValue } from "clsx";
import { useEffect } from "react";
import { twMerge } from "tailwind-merge";
import { create } from "zustand";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

type GlobalStateStore = {
  dict: { [_ in string]?: unknown };
  setAtKey: <T>(key: string, value: T) => void;
};

const useGlobalStateStore = create<GlobalStateStore>((set, _get) => ({
  dict: {},
  setAtKey(key, value) {
    set((prev) => deep(prev).replaceAt(`dict.${key}`, value).take());
  },
}));

export function useGlobalState<T>(key: string, opts?: { default: T }) {
  const cur = useGlobalStateStore((s) => s.dict[key]);
  const setAtKey = useGlobalStateStore.getState().setAtKey;

  useEffect(() => {
    if (typeof opts?.default != "undefined" && typeof cur == "undefined") setAtKey(key, opts?.default);
  }, []);

  if (typeof cur == "undefined" && typeof opts?.default != "undefined")
    return [opts.default, (value: T | undefined) => setAtKey(key, value)] as const;
  else return [cur as T | undefined, (value: T | undefined) => setAtKey(key, value)] as const;
}