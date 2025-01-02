import eslintPluginPrettierRecommended from "eslint-plugin-prettier/recommended";
import tailwind from "eslint-plugin-tailwindcss";
// @ts-check
import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt(
  eslintPluginPrettierRecommended,
  ...tailwind.configs["flat/recommended"],
  {
    files: ["**/*.ts", "**/*.vue", "**/*.mjs"],
    rules: {
      "vue/no-multiple-template-root": "off",
      "vue/no-v-html": "off",
      "tailwindcss/no-custom-classname": "off",
      "vue/html-self-closing": "error",
    },
  },
);
