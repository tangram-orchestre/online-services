/**
 * The item of a list selected in the `key` query parameter, so that it
 * survives reloads and can be linked to (from the dashboard, from the other
 * pages), and the item shown in the details panel.
 */
export const useListSelection = <T extends { id: number }>(
  key: string,
  items: Ref<T[]>,
  /** Shown on large screens until an item is selected. */
  fallback: () => T | undefined,
) => {
  const route = useRoute();
  const router = useRouter();

  const selectedId = computed<number | undefined>({
    get: () => Number(route.query[key]) || undefined,
    set: (id) => {
      router.replace({ query: { ...route.query, [key]: id } });
    },
  });
  const selected = computed(() =>
    items.value.find((item) => item.id === selectedId.value),
  );
  const shown = computed(() => selected.value ?? fallback());

  /** Whether an item was explicitly selected: opens the small screen panel. */
  const detailOpen = computed({
    get: () => !!selected.value,
    set: (open) => {
      if (!open) selectedId.value = undefined;
    },
  });

  /** State of a list row, for `ListRow`. */
  const rowState = (id: number) =>
    selected.value?.id === id
      ? ("selected" as const)
      : shown.value?.id === id
        ? ("shown" as const)
        : undefined;

  /** Clears the selection when its item is deleted. */
  const onDeleted = (id: number) => {
    if (selectedId.value === id) selectedId.value = undefined;
  };

  return { selectedId, selected, shown, detailOpen, rowState, onDeleted };
};
