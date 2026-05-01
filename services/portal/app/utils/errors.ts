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
        errorMsg = `Impossible de supprimer ce${f("tte")} ${entityName} car ${f("elle", "il")} est encore référencé${f("e")}`;
      }
    } else {
      errorMsg = `Erreur ${e.response.status}`;
    }
    func(errorMsg);
  };
};
