import { defaultPlugins, defineConfig } from "@hey-api/openapi-ts";

export default defineConfig({
  input: "/opt/openapi/public-spec.json",
  output: "client",
  plugins: [...defaultPlugins, "@hey-api/client-nuxt", "zod"],
});
