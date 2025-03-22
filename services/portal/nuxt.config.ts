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
  ],
  vuetify: {
    moduleOptions: {
      /* module specific options */
    },
    vuetifyOptions: {
      /* vuetify options */
    },
  },
  runtimeConfig: {
    public: {
      api_base_url: "/api",
    },
  },
  fonts: {
    defaults: {
      weights: [400],
      styles: ["normal"],
      subsets: ["latin, latin-ext"],
    },
    families: [{ name: "Roboto", weights: [400] }],
  },
});
