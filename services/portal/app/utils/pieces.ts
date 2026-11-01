import type { Person } from "#hey-api/types.gen";

/** « Beethoven · arr. Jeanne Moreau » */
export const pieceCredits = (
  piece: { composer?: string | null; arranger_ids: number[] },
  people: Map<number, Person>,
) => {
  const arrangers = peopleNames(piece.arranger_ids, people);
  return [piece.composer, arrangers && `arr. ${arrangers}`]
    .filter(Boolean)
    .join(" · ");
};
