import { defineConfig } from "@hey-api/openapi-ts";

export default defineConfig({
  client: "@hey-api/client-nuxt",
  experimentalParser: true,
  input: "/opt/openapi/spec.json",
  output: "client",
});
