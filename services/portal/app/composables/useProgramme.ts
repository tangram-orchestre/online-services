import type { ConcertWithPieces, Piece } from "#hey-api/types.gen";

/**
 * Concerts and pieces of the selected semester.
 *
 * Every page and dialog shares the same cached lists (same async data keys):
 * updating `allConcerts` / `allPieces` after a save updates them everywhere.
 */
export const useProgramme = () => {
  const { selectedSemester: semester } = useSemesters();

  const {
    data: allConcerts,
    refresh: refreshConcerts,
    status: concertsStatus,
  } = getConcerts({ composable: "useAsyncData", key: "getConcerts" });
  const {
    data: allPieces,
    refresh: refreshPieces,
    status: piecesStatus,
  } = getPieces({ composable: "useAsyncData", key: "getPieces" });

  const loading = computed(
    () =>
      concertsStatus.value === "pending" || piecesStatus.value === "pending",
  );

  // TODO: filter server side once the API supports it.
  /** By date. */
  const concerts = computed<ConcertWithPieces[]>(() =>
    (allConcerts.value ?? [])
      .filter((c) => c.semester_id === semester.value?.id)
      .sort((a, b) => a.date.localeCompare(b.date)),
  );
  /** By name. */
  const pieces = computed<Piece[]>(() =>
    (allPieces.value ?? [])
      .filter((p) => p.semester_id === semester.value?.id)
      .sort((a, b) => a.name.localeCompare(b.name, "fr")),
  );
  const pieceById = computed(
    () => new Map<number, Piece>(pieces.value.map((p) => [p.id, p])),
  );

  /** Concerts whose programme contains the piece, by date. */
  const concertsOf = (pieceId: number) =>
    concerts.value.filter((c) => c.piece_ids.includes(pieceId));

  return {
    semester,
    allConcerts,
    allPieces,
    concerts,
    pieces,
    pieceById,
    concertsOf,
    loading,
    refreshConcerts,
    refreshPieces,
  };
};

/**
 * The id of the item selected in the `key` query parameter, so that it
 * survives reloads and can be linked to (from the dashboard, from the other
 * page).
 */
export const useSelectedId = (key: string) => {
  const route = useRoute();
  const router = useRouter();
  return computed<number | undefined>({
    get: () => Number(route.query[key]) || undefined,
    set: (id) => {
      router.replace({ query: { ...route.query, [key]: id } });
    },
  });
};
