import { match } from "ts-pattern";

const errorMessageFromCheckViolation = (reason: CheckViolation): string => {
  if (reason.kind) {
    return match(reason.kind)
      .with("SemestersDateOverlap", () => {
        return "Les dates des semestres ne peuvent pas se chevaucher.";
      })
      .with("ConcertDateOutsideSemester", () => {
        return "La date du concert n'est pas dans la période du semestre.";
      })
      .with("ConcertPieceOutsideSemester", () => {
        return "Un morceau du programme n'appartient pas au semestre du concert.";
      })
      .with("ConcertRoomRehearsalOnly", () => {
        return "Cette salle n'accueille pas de public : elle ne peut servir qu'aux répétitions.";
      })
      .with("ConcertRoomBookedWithoutRoom", () => {
        return "Choisissez une salle avant de confirmer sa réservation.";
      })
      .with("RoomHostsConcerts", () => {
        return "Une salle qui accueille des concerts doit avoir une capacité public.";
      })
      .with("RoomTotalCapacityWithoutAudience", () => {
        return "Une salle sans public ne peut pas avoir de capacité totale.";
      })
      .with("PersonAnonymousWithDisplayName", () => {
        return "Une personne anonyme ne peut pas avoir d'alias.";
      })
      .with("RoomTotalCapacityTooSmall", () => {
        return "La capacité totale d'une salle ne peut pas être inférieure à sa capacité musiciens ou public.";
      })
      .exhaustive();
  }

  return reason.message;
};

export type ErrorResponseHandler = (e: {
  response: { status: number; _data: BadRequestReason };
}) => void;

export const onResponseErrorHandler = (
  entityName: string,
  isFeminine: boolean,
  func: (errorMsg: string) => void,
  /** Replaces the default message when the entity is still referenced. */
  foreignKeyMessage?: string,
): ErrorResponseHandler => {
  const f = (feminine: string, masculine: string = "") => {
    if (isFeminine) {
      return feminine;
    }
    return masculine || "";
  };
  return (e: { response: { status: number; _data: BadRequestReason } }) => {
    let errorMsg = "Erreur inconnue";
    if (e.response._data && e.response.status === 400) {
      const d = e.response._data as GetDummyErrors[400];
      if (d.type === "UniqueViolation") {
        errorMsg = `Un${f("e")} ${entityName} avec ce nom existe déjà`;
      } else if (d.type === "CheckViolation") {
        errorMsg = errorMessageFromCheckViolation(d);
      } else if (d.type === "NotNullViolation") {
        errorMsg = `Un champ requis n'est pas rempli`;
      } else if (d.type === "ForeignKeyViolation") {
        errorMsg =
          foreignKeyMessage ??
          `Impossible de supprimer ce${f("tte")} ${entityName} car ${f("elle", "il")} est encore référencé${f("e")}`;
      }
    } else {
      errorMsg = `Erreur ${e.response.status}`;
    }
    func(errorMsg);
  };
};
