import type { PublicConcert } from "~/client";

/** « 27 janvier 2027 » from « 2027-01-27 ». */
export const formatConcertDate = (date: string) => {
  const [year, month, day] = date.split("-").map(Number);
  return new Date(year!, month! - 1, day).toLocaleDateString("fr-FR", {
    day: "numeric",
    month: "long",
    year: "numeric",
  });
};

/** Today, as YYYY-MM-DD. */
const today = () => {
  const now = new Date();
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${now.getFullYear()}-${pad(now.getMonth() + 1)}-${pad(now.getDate())}`;
};

/** Shown for concerts without an image of their own, of their room or of their venue. */
const defaultImage = "/images/partoches.jpg";

const escapeHtml = (text: string) =>
  text
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");

/**
 * A concert of the database, as shown on a card (without programme). Its
 * image is served by the public API at `apiBaseUrl`.
 */
export const fromPublicConcert = (
  concert: PublicConcert,
  apiBaseUrl: string,
): Concert => {
  const upcoming = concert.date >= today();
  return {
    date: concert.date,
    location: escapeHtml(concert.location_name ?? "Lieu à venir"),
    image: concert.image_id
      ? `${apiBaseUrl}/public/images/${concert.image_id}`
      : defaultImage,
    url: upcoming
      ? concert.tickets_url
        ? { kind: "Tickets", target: concert.tickets_url }
        : { kind: "Tickets Unavailable", target: undefined }
      : concert.video_url
        ? { kind: "Video", target: concert.video_url }
        : undefined,
  };
};
