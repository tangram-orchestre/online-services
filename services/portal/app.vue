<script lang="ts" setup>
import { client } from "./client/client.gen";
import { getUsersMe } from "~/client";

const runtimeConfig = useRuntimeConfig();

// configure internal service client
client.setConfig({
  // set default base url for requests
  baseURL: runtimeConfig.public.api_base_url,
  credentials: "include",
});

const isDark = useDark({
  selector: "html",
  attribute: "class",
  valueDark: "tangram-dark",
});
const toggleDark = useToggle(isDark);

const { data, refresh } = getUsersMe({
  composable: "useAsyncData",
});
</script>

<template>
  <div>
    <Button label="Toggle Dark Theme" icon="pi pi-moon" @click="toggleDark()" />
    <div v-if="data">
      Hello {{ data.first_name }} {{ data.last_name }}, how are you?
    </div>
    <Button label="Refresh" @click="() => refresh()" />
  </div>
</template>

<style lang="scss">
@use "~/assets/scss/main.scss";
</style>
