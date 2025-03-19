export interface Url {
  target: string | undefined;
  kind: "Tickets Unavailable" | "Tickets" | "Video";
}

export interface Piece {
  name: string;
  direction?: string;
}

export interface Concert {
  date: string;
  location: string;
  image: string;
  url?: Url;
  program?: Array<Piece>;
}
