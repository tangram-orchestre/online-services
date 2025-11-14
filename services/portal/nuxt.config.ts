import { fr } from "vuetify/locale";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2024-11-01",
  devtools: { enabled: true },
  ssr: false,

  modules: [
    "@vueuse/nuxt",
    "@nuxt/eslint",
    "@nuxt/image",
    "@nuxt/fonts",
    "vuetify-nuxt-module",
    "@hey-api/nuxt",
  ],
  vuetify: {
    moduleOptions: {
      /* module specific options */
    },
    vuetifyOptions: {
      locale: {
        locale: "fr",
        messages: { fr },
      },
      labComponents: true,
    },
  },
  heyApi: {
    config: {
      input: "/opt/openapi/private-spec.json",
      plugins: [
        "@hey-api/schemas",
        {
          name: "@hey-api/sdk",
          transformer: true,
          validator: true,
        },
        {
          enums: "javascript",
          name: "@hey-api/typescript",
        },
        "@hey-api/transformers",
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
  // fonts: {
  //   defaults: {
  //     weights: [400],
  //     styles: ["normal"],
  //     subsets: ["latin, latin-ext"],
  //   },
  //   families: [{ name: "Roboto", weights: [400] }],
  // },
  typescript: {
    typeCheck: true,
  },
});
