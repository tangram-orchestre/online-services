import { match } from "ts-pattern";

export const errorMessageFromCheckViolation = (
  reason: CheckViolation,
): string => {
  console.log(reason);

  if (reason.kind) {
    return match(reason.kind)
      .with("SemestersDateOverlap", () => {
        return "Les dates des semestres ne peuvent pas se chevaucher.";
      })
      .exhaustive();
  }

  return reason.message;
};
