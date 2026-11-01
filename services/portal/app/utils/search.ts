/** Lowercase, without accents: « Dvořák » matches « dvorak ». */
const normalize = (value: string) =>
  value
    .normalize("NFD")
    .replace(/\p{Diacritic}/gu, "")
    .toLowerCase();

/** Whether one of `fields` contains `query`, ignoring case and accents. */
export const matchesSearch = (
  query: string,
  fields: (string | null | undefined)[],
) => {
  const q = normalize(query.trim());
  return !q || fields.some((field) => field && normalize(field).includes(q));
};
