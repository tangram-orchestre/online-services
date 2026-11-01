import type { Person } from "#hey-api/types.gen";

/** « Jeanne Moreau » */
export const personName = (person: { first_name: string; last_name: string }) =>
  `${person.first_name} ${person.last_name}`;

/**
 * Name on public Tangram material: the alias, else the full name. None for
 * anonymous people, who are never named there.
 */
export const publicName = (person: Person) =>
  person.anonymous ? undefined : (person.display_name ?? personName(person));

/** By last name, then first name. */
export const comparePeople = (a: Person, b: Person) =>
  a.last_name.localeCompare(b.last_name, "fr") ||
  a.first_name.localeCompare(b.first_name, "fr");

/** People of `ids`, skipping unknown ones. */
export const peopleOf = (ids: number[], byId: Map<number, Person>) =>
  ids.map((id) => byId.get(id)).filter((p): p is Person => !!p);

/** « Jeanne Moreau, Lucas Petit » */
export const peopleNames = (ids: number[], byId: Map<number, Person>) =>
  peopleOf(ids, byId).map(personName).join(", ");
