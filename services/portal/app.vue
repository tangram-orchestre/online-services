<script lang="ts" setup>
import { client } from "./client/client.gen";
import { getUsersMe } from "~/client";

import { useTheme } from "vuetify";

const runtimeConfig = useRuntimeConfig();

// configure internal service client
client.setConfig({
  // set default base url for requests
  baseURL: runtimeConfig.public.api_base_url,
  credentials: "include",
});

const isDark = useDark();
const toggleDark = useToggle(isDark);
const theme = useTheme();

watch(isDark, (value) => {
  theme.global.name.value = value ? "dark" : "light";
});

const { data, refresh } = getUsersMe({
  composable: "useAsyncData",
});
</script>

<template>
  <v-app>
    <v-navigation-drawer>
      <v-list>
        <v-list-item title="My Application" subtitle="Vuetify" />
        <v-divider />
        <v-list-item title="Navigation drawer" />
      </v-list>
    </v-navigation-drawer>

    <v-app-bar title="Tangram">
      <v-btn
        :icon="isDark ? 'mdi-weather-night' : 'mdi-brightness-5'"
        @click="toggleDark()"
      />
    </v-app-bar>

    <v-main>
      <v-container>
        <div v-if="data">
          Hello {{ data.first_name }} ({{ data.username }})
          {{ data.last_name }} {{ data.phone_number }}, how are you?
        </div>
        <v-btn @click="() => refresh()">Refresh</v-btn>
      </v-container>
    </v-main>
  </v-app>
</template>

<style lang="scss">
@use "~/assets/scss/main.scss";
</style>
