<script lang="ts" setup>
import { NuxtPage } from "#components";
import { client } from "#hey-api/client.gen";

import i18next from "i18next";
import { z } from "zod";
import { zodI18nMap } from "zod-i18n-map";
import translation from "zod-i18n-map/locales/fr/zod.json";

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
  theme.global.name.value = isDark.value ? "dark" : "light";
};

onMounted(() => {
  setTheme();
});

watch(isDark, () => {
  setTheme();
});

const drawer = ref(false);
</script>

<template>
  <v-app>
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
        <v-list-item prepend-icon="mdi-view-dashboard" to="/dashboard">
          Dashboard
        </v-list-item>

        <template v-if="user.groups.includes('Orga')">
          <v-divider />
          <v-list-item prepend-icon="mdi-account-group" to="/members">
            Membres
          </v-list-item>
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
