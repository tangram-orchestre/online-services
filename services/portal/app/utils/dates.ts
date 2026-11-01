import { differenceInCalendarDays, parseISO } from "date-fns";

/** Parses an API date (`YYYY-MM-DD`) as a local day. */
export const parseDay = (value: string) => parseISO(value);

const format = (value: string, options: Intl.DateTimeFormatOptions) =>
  parseDay(value).toLocaleDateString("fr", options);

/** « 13 déc. » */
export const formatShortDay = (value: string) =>
  format(value, { day: "numeric", month: "short" });

/** « 13 déc. 2026 » */
export const formatDay = (value: string) =>
  format(value, { day: "numeric", month: "short", year: "numeric" });

/** « dim. 13 décembre » */
export const formatLongDay = (value: string) =>
  format(value, { weekday: "short", day: "numeric", month: "long" });

/** « 16:00:00 » → « 16:00 » */
export const formatTime = (value: string) => value.slice(0, 5);

/** Calendar days from today to `value` (negative when in the past). */
export const daysUntil = (value: string) =>
  differenceInCalendarDays(parseDay(value), new Date());

/** « Aujourd’hui », « Demain », « J-12 », or undefined once past. */
export const countdown = (value: string) => {
  const days = daysUntil(value);
  if (days < 0) return undefined;
  if (days === 0) return "Aujourd’hui";
  if (days === 1) return "Demain";
  return `J-${days}`;
};
