import eslintPluginPrettierRecommended from "eslint-plugin-prettier/recommended";
// @ts-check
import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt(eslintPluginPrettierRecommended, {
  files: ["**/*.ts", "**/*.vue", "**/*.mjs"],
  rules: {
    "vue/no-multiple-template-root": "off",
    "vue/no-v-html": "off",
    "vue/html-self-closing": [
      "error",
      {
        html: {
          void: "always",
          normal: "always",
          component: "always",
        },
        svg: "always",
        math: "always",
      },
    ],
    "vue/valid-v-slot": [
      "error",
      {
        allowModifiers: true,
      },
    ],
  },
});
