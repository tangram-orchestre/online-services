import type { TypedSchema, TypedSchemaError } from "vee-validate";
import type { z } from "zod";

/**
 * Validate a vee-validate form with a Zod schema (`@vee-validate/zod` only
 * supports Zod 3).
 */
export const toTypedSchema = <S extends z.ZodType>(
  schema: S,
): TypedSchema<Partial<z.input<S>>, z.output<S>> => ({
  __type: "VVTypedSchema",
  async parse(values) {
    const result = await schema.safeParseAsync(values);
    if (result.success) {
      return { value: result.data, errors: [] };
    }
    const errors = new Map<string, TypedSchemaError>();
    for (const issue of result.error.issues) {
      // vee-validate paths index arrays with brackets: `contacts[0].name`.
      const path = issue.path
        .map((key) =>
          typeof key === "number" ? `[${key}]` : `.${String(key)}`,
        )
        .join("")
        .replace(/^\./, "");
      const error = errors.get(path) ?? { path, errors: [] };
      error.errors.push(issue.message);
      errors.set(path, error);
    }
    return { errors: [...errors.values()] };
  },
});
