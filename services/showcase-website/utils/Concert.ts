export interface Url {
  target: string | undefined;
  kind: "Tickets Unavailable" | "Tickets" | "No Tickets" | "Video";
}

export interface Piece {
  name: string;
  direction?: string;
}

export interface Concert {
  /** YYYY-MM-DD */
  date: string;
  /** HTML (e.g. « Mairie du XIV<sup>ème</sup> »). */
  location: string;
  /** Local path (bundled image) or URL of an image uploaded to the API. */
  image: string;
  url?: Url;
  program?: Array<Piece>;
}
