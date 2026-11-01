/** « Beethoven · arr. Jean Dupont » */
export const pieceCredits = (piece: {
  composer?: string | null;
  arranger?: string | null;
}) =>
  [piece.composer, piece.arranger && `arr. ${piece.arranger}`]
    .filter(Boolean)
    .join(" · ");
