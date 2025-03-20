<script lang="ts" setup>
import { client } from "./client/client.gen";
import { getTest } from "~/client";

const runtimeConfig = useRuntimeConfig();

// configure internal service client
client.setConfig({
  // set default base url for requests
  baseURL: runtimeConfig.public.api_base_url,
  credentials: "include",
});

const { status, data, refresh } = getTest({
  composable: "useAsyncData",
});

const toggleDarkMode = () => {
  const html = document.querySelector("html")!;
  html.classList.toggle("tangram-dark");
};
</script>

<template>
  <div>
    <Button
      label="Toggle Dark Theme"
      icon="pi pi-moon"
      @click="toggleDarkMode()"
    />
    <div>{{ status }} toto {{ data }}</div>
    <Button label="Refresh" @click="() => refresh()" />
  </div>
</template>

<style lang="scss">
@use "~/assets/scss/main.scss";
</style>
