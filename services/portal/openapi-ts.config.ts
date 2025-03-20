import { defaultPlugins, defineConfig } from "@hey-api/openapi-ts";

export default defineConfig({
  experimentalParser: true,
  input: "/opt/openapi/private-spec.json",
  output: "client",
  plugins: [...defaultPlugins, "@hey-api/client-nuxt", "zod"],
});
