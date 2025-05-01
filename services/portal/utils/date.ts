import { format } from "date-fns";

export type Converted<U, T, K extends keyof T & string> = {
  [P in K]: U;
} & {
  [P in keyof T as Exclude<P, K>]: T[P];
};

export function convert_fields_to_dates<T, K extends keyof T & string>(
  data: T,
  date_fields: K[],
): Converted<Date, T, K> {
  let instance = {};
  for (const key in data) {
    if (
      data[key] &&
      typeof data[key] === "string" &&
      date_fields.includes(key as unknown as K)
    ) {
      instance = { [key]: new Date(data[key]), ...instance };
    } else {
      instance = { [key]: data[key], ...instance };
    }
  }
  return instance as Converted<Date, T, K>;
}

export function convert_fields_from_dates<T, K extends keyof T & string>(
  data: Converted<Date, T, K>,
): T {
  let instance = {};
  let key: keyof Converted<Date, T, K>;
  for (key in data) {
    const field = data[key];
    if (field instanceof Date) {
      instance = { [key]: format(data[key], "yyyy-LL-dd"), ...instance };
    } else {
      instance = { [key]: data[key], ...instance };
    }
  }
  return instance as T;
}
