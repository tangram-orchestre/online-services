/**
 * Semesters and the currently selected one.
 *
 * The selection is kept in the `semester` query parameter so it survives
 * reloads, can be shared, and follows the user from page to page.
 */
export const useSemesters = () => {
  const route = useRoute();
  const router = useRouter();

  const {
    data: semesters,
    refresh,
    status,
  } = getSemesters({
    composable: "useAsyncData",
    key: "getSemesters",
    // Reserved to the Orga group: not fetched for other members.
    asyncDataOptions: { immediate: useRoles().isOrga.value },
  });

  /** Most recent first. */
  const sortedSemesters = computed(() =>
    [...(semesters.value ?? [])].sort((a, b) =>
      b.start_date.localeCompare(a.start_date),
    ),
  );

  /** Semester containing today, or the most recent one. */
  const defaultSemester = () => {
    const today = new Date().toISOString().split("T")[0]!;
    return (
      sortedSemesters.value.find(
        (s) => s.start_date <= today && today <= s.end_date,
      ) ?? sortedSemesters.value[0]
    );
  };

  const selectedSemesterId = computed<number | undefined>({
    get: () => {
      const fromQuery = Number(route.query.semester);
      if (sortedSemesters.value.some((s) => s.id === fromQuery)) {
        return fromQuery;
      }
      return defaultSemester()?.id;
    },
    set: (id) => {
      router.replace({ query: { ...route.query, semester: id } });
    },
  });

  const selectedSemester = computed(() =>
    sortedSemesters.value.find((s) => s.id === selectedSemesterId.value),
  );

  /** A link to `path` that keeps the selected semester. */
  const pageLink = (
    path: string,
    query: Record<string, string | number | undefined> = {},
  ) => ({ path, query: { semester: selectedSemesterId.value, ...query } });

  return {
    pageLink,
    semesters,
    sortedSemesters,
    selectedSemesterId,
    selectedSemester,
    refresh,
    status,
  };
};
