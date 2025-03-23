<script lang="ts" setup>
import { NuxtPage } from "#components";
import { client } from "./client/client.gen";

import { useTheme } from "vuetify";

const runtimeConfig = useRuntimeConfig();

// configure internal service client
client.setConfig({
  // set default base url for requests
  baseURL: runtimeConfig.public.api_base_url,
  credentials: "include",
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
      <v-btn icon="mdi-account" to="/" />
    </v-app-bar>

    <v-navigation-drawer v-model="drawer">
      <v-list nav>
        <v-list-item prepend-icon="mdi-view-dashboard" to="/dashboard">
          Dashboard
        </v-list-item>

        <template v-if="user.groups.includes('Orga')">
          <v-divider />
          <v-list-item prepend-icon="mdi-account-group" to="/users">
            Membres
          </v-list-item>
        </template>
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <NuxtPage />
    </v-main>
  </v-app>
</template>

<style lang="scss">
@use "~/assets/scss/main.scss";
</style>
