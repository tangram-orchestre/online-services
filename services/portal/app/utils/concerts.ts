/** The concert name, or « Concert du dim. 13 décembre » for unnamed ones. */
export const concertTitle = (concert: { name?: string | null; date: string }) =>
  concert.name || `Concert du ${formatLongDay(concert.date)}`;
