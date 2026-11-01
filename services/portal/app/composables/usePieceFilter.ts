/** A concert id (pieces on its programme), or the special filters. */
export type PieceFilter = number | "all" | "unscheduled";

/** Filter of the selected semester's pieces by concert. */
export const usePieceFilter = (initial: PieceFilter = "all") => {
  const { pieces, concerts, concertsOf, loading } = useProgramme();

  const filter = ref<PieceFilter>(initial);

  const unscheduled = computed(() =>
    pieces.value.filter((p) => !concertsOf(p.id).length),
  );

  const filters = computed(() => [
    { value: "all" as PieceFilter, label: `Tous (${pieces.value.length})` },
    ...concerts.value.map((c) => ({
      value: c.id as PieceFilter,
      label: concertShortName(c),
    })),
    ...(unscheduled.value.length
      ? [
          {
            value: "unscheduled" as PieceFilter,
            label: `Hors programme (${unscheduled.value.length})`,
          },
        ]
      : []),
  ]);

  // Back to all pieces when the filtered concert disappears (e.g. semester
  // change), once loaded.
  watch(filters, (items) => {
    if (!loading.value && !items.some((f) => f.value === filter.value)) {
      filter.value = "all";
    }
  });

  /** The pieces matching the filter, by name. */
  const filtered = computed(() => {
    const f = filter.value;
    if (f === "all") return pieces.value;
    if (f === "unscheduled") return unscheduled.value;
    return pieces.value.filter((p) => concertsOf(p.id).some((c) => c.id === f));
  });

  return { filter, filters, filtered };
};
