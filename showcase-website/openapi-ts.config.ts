import { defineConfig } from "@hey-api/openapi-ts";

export default defineConfig({
  experimentalParser: true,
  input: "/opt/openapi/spec.json",
  output: "client",
  plugins: ["@hey-api/client-nuxt"],
});
