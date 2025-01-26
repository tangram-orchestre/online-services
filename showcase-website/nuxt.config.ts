// https://nuxt.com/docs/api/configuration/nuxt-config

import svgLoader from "vite-svg-loader";

const get_env_var = (s: string): string => {
  const val = process.env[s];
  if (!val) {
    throw new Error(`Missing ${s} env var`);
  }
  return val;
};

export default defineNuxtConfig({
  compatibilityDate: "2024-04-03",
  devtools: { enabled: true },
  typescript: { typeCheck: true },
  runtimeConfig: {
    private_api_base_url: get_env_var("PRIVATE_API_BASE_URL"),
    public: {
      api_base_url: get_env_var("PUBLIC_API_BASE_URL"),
    },
  },
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
