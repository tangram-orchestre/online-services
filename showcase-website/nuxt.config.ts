// https://nuxt.com/docs/api/configuration/nuxt-config

import svgLoader from "vite-svg-loader";

export default defineNuxtConfig({
  compatibilityDate: "2024-04-03",
  devtools: { enabled: true },
  typescript: { typeCheck: true },
  modules: [
    "@nuxt/eslint",
    "@vueuse/nuxt",
    "@nuxt/image",
    "@nuxt/icon",
    "@nuxt/fonts",
  ],
  css: ["~/assets/css/main.css"],
  postcss: {
    plugins: {
      tailwindcss: {},
      autoprefixer: {},
    },
  },
  vite: {
    plugins: [svgLoader()],
    css: {
      preprocessorOptions: {
        scss: {
          api: "modern-compiler", // or "modern"
        },
      },
    },
  },
  nitro: {
    compressPublicAssets: true,
  },
  fonts: {
    defaults: {
      weights: [400],
      styles: ["normal"],
      subsets: ["latin, latin-ext"],
    },
    families: [
      { name: "Roboto", weights: [100, 300, 400] },
      { name: "Oswald", weight: 300 },
      { name: "Raleway" },
      { name: "Lilita One" },
    ],
  },
});
