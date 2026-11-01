/** The concert name, or « Concert du dim. 13 décembre » for unnamed ones. */
export const concertTitle = (concert: { name?: string | null; date: string }) =>
  concert.name || `Concert du ${formatLongDay(concert.date)}`;

/** A short label for chips: the concert name, or « 13 déc. ». */
export const concertShortName = (concert: {
  name?: string | null;
  date: string;
}) => concert.name || formatShortDay(concert.date);

/** « 1 morceau », « 3 morceaux » */
export const piecesCount = (n: number) =>
  n === 1 ? "1 morceau" : `${n} morceaux`;
