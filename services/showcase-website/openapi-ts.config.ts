import { defaultPlugins, defineConfig } from "@hey-api/openapi-ts";

export default defineConfig({
  input: "/opt/openapi/public-spec.json",
  output: {
    path: "client",
    // The client runtime, copied from openapi-ts (files without a `file`),
    // does not type-check against Nuxt >= 4.5 (`cache: false`).
    header: ({ defaultValue, file }) =>
      file ? defaultValue : ["// @ts-nocheck", ...defaultValue],
  },
  plugins: [...defaultPlugins, "@hey-api/client-nuxt", "zod"],
});
