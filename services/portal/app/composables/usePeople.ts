import type { Person } from "#hey-api/types.gen";

/**
 * People credited on pieces, shared by every page and dialog (same async
 * data key): updating `allPeople` after a save updates them everywhere.
 */
export const usePeople = () => {
  const {
    data: allPeople,
    refresh,
    status,
  } = getPeople({ composable: "useAsyncData", key: "getPeople" });

  /** By last name, then first name. */
  const people = computed<Person[]>(() =>
    [...(allPeople.value ?? [])].sort(comparePeople),
  );
  const personById = computed(
    () => new Map<number, Person>(people.value.map((p) => [p.id, p])),
  );

  return { allPeople, people, personById, refresh, status };
};
