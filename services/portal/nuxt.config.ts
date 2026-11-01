// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2024-11-01",
  devtools: { enabled: true },
  ssr: false,

  modules: [
    "@vueuse/nuxt",
    "@nuxt/eslint",
    "@nuxt/image",
    "@nuxt/ui",
    "@hey-api/nuxt",
  ],
  css: ["~/assets/css/main.css"],
  heyApi: {
    config: {
      input: "/opt/openapi/private-spec.json",
      plugins: [
        "@hey-api/schemas",
        {
          name: "@hey-api/sdk",
        },
        {
          enums: "javascript",
          name: "@hey-api/typescript",
        },
        "zod",
      ],
    },
  },
  watch: ["/opt/openapi/private-spec.json"],
  runtimeConfig: {
    public: {
      api_base_url: "/api",
      sso_user_settings_url:
        "https://auth.tangram-orchestre.fr/if/user/#/settings",
    },
  },
  typescript: {
    typeCheck: true,
  },
});
