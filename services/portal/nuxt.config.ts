import Aura from "@primeuix/themes/aura";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2024-11-01",
  devtools: { enabled: true },
  ssr: false,

  modules: [
    "@primevue/nuxt-module",
    "@nuxt/eslint",
    "@nuxt/image",
    "@nuxt/fonts",
  ],
  primevue: {
    options: {
      theme: {
        preset: Aura,
        options: {
          darkModeSelector: ".tangram-dark",
        },
      },
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
