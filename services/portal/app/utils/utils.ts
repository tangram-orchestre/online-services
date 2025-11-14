import type { UnwrapRef } from "vue";

export type ElementOfArray<T> = T extends Array<infer U> ? U : never;

export type ElementOfRefArray<T> = ElementOfArray<UnwrapRef<T>>;

export const updateById = <T extends { id: number | string }>(
  arr: T[] | null,
  element: T,
) => {
  if (!arr) {
    return;
  }

  const index = arr.findIndex((e) => e.id == element.id);
  arr[index] = element;
};
