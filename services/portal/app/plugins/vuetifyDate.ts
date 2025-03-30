import { defineNuxtPlugin } from "#imports";
import { StringDateAdapter } from "vuetify/date/adapters/string";

export default defineNuxtPlugin({
  name: "vuetify:date:plugin",
  order: -25,
  parallel: true,
  setup(nuxtApp) {
    nuxtApp.hook("vuetify:configuration", ({ vuetifyOptions }) => {
      vuetifyOptions.date = {
        adapter: StringDateAdapter,
      };
    });
  },
});
