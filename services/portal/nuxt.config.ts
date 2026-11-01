// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2024-11-01",
  devtools: { enabled: true },
  ssr: false,
  // Shown until the app loads, then set by `app.vue` and the pages.
  app: { head: { title: "Portail Tangram" } },

  modules: ["@vueuse/nuxt", "@nuxt/eslint", "@nuxt/image", "@nuxt/ui"],
  css: ["~/assets/css/main.css"],
  ui: {
    theme: {
      colors: [
        "primary",
        "secondary",
        "tertiary",
        "info",
        "success",
        "warning",
        "error",
      ],
    },
  },
  fonts: {
    defaults: {
      weights: [300, 400, 500, 600, 700],
    },
  },
  heyApi: {
    input: "/opt/openapi/private-spec.json",
    config: {
      plugins: [
        "@hey-api/client-nuxt",
        "@hey-api/schemas",
        "@hey-api/sdk",
        {
          name: "@hey-api/typescript",
          enums: "javascript",
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
