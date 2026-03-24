<script lang="ts" setup>
import { NuxtPage } from "#components";
import { client } from "#hey-api/client.gen";

import i18next from "i18next";
import { z } from "zod";
import { zodI18nMap } from "zod-i18n-map";
import translation from "zod-i18n-map/locales/fr/zod.json";
import ConfirmDialog from "./components/ConfirmDialog.vue";

import { VSonner } from "vuetify-sonner";
import "vuetify-sonner/style.css";

i18next.init({
  lng: "fr",
  resources: {
    fr: { zod: translation },
  },
});
z.setErrorMap(zodI18nMap);

const runtimeConfig = useRuntimeConfig();

// configure internal service client
client.setConfig({
  // set default base url for requests
  baseURL: runtimeConfig.public.api_base_url,
  credentials: "include",
});

useHead({
  htmlAttrs: {
    lang: "fr",
  },
});

const user = await currentUser();

const isDark = useDark();
const toggleDark = useToggle(isDark);
const theme = useTheme();

const setTheme = () => {
  theme.change(isDark.value ? "dark" : "light");
};

onMounted(() => {
  setTheme();
});

watch(isDark, () => {
  setTheme();
});

const drawer = ref(false);

const confirmDialogRef =
  useTemplateRef<InstanceType<typeof ConfirmDialog>>("confirm");

onMounted(() => {
  if (confirmDialogRef.value) {
    setConfirmDialogInstance(confirmDialogRef.value);
  } else {
    console.error("ConfirmDialog component not found");
  }
});
</script>

<template>
  <v-app>
    <confirm-dialog ref="confirm" />

    <v-sonner position="top-right" :duration="3000" />

    <v-app-bar color="primary">
      <v-app-bar-nav-icon variant="text" @click.stop="drawer = !drawer" />

      <v-toolbar-title>Tangram</v-toolbar-title>

      <v-spacer />

      <v-btn
        :icon="isDark ? 'mdi-weather-night' : 'mdi-brightness-5'"
        @click="toggleDark()"
      />
      <v-btn class="ma-2" icon="mdi-account" to="/" />
    </v-app-bar>

    <v-navigation-drawer v-model="drawer">
      <v-list nav>
        <template v-if="user.groups.includes('Orga')">
          <v-list-item prepend-icon="mdi-calendar" to="/semesters">
            Semestres
          </v-list-item>
        </template>
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <v-container class="pa-4">
        <NuxtPage />
      </v-container>
    </v-main>
  </v-app>
</template>

<style lang="scss">
@use "~/assets/scss/main.scss";
</style>
