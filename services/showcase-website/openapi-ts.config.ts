import { defaultPlugins, defineConfig } from "@hey-api/openapi-ts";

export default defineConfig({
  experimentalParser: true,
  input: "/opt/openapi/public-spec.json",
  output: "client",
  plugins: [...defaultPlugins, "@hey-api/client-nuxt", "zod"],
});
