import Aura from "@primeuix/themes/aura";
import { fr } from "primelocale/js/fr.js";

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
    "@primevue/nuxt-module",
    "@nuxtjs/tailwindcss",
  ],
  primevue: {
    options: {
      theme: {
        preset: Aura,
        options: {
          darkModeSelector: ".dark",
        },
      },
      locale: fr,
    },
  },
  runtimeConfig: {
    public: {
      api_base_url: "/api",
      sso_user_settings_url:
        "https://auth.tangram-orchestre.fr/if/user/#/settings",
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
  typescript: {
    typeCheck: true,
  },
});
