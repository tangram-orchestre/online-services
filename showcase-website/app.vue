<script setup lang="ts">
import { client } from "./client/client.gen";

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

useHead({
  htmlAttrs: {
    lang: "fr",
  },
});

useSeoMeta({
  title: "Tangram Orchestre",
  ogTitle: "Tangram Orchestre",
  ogUrl: "https://www.tangram-orchestre.fr",
  description: "Orchestre amateur pour Chef⸱fe⸱s en herbe",
  ogDescription: "Orchestre amateur pour Chef⸱fe⸱s en herbe",
  ogImage: "https://www.tangram-orchestre.fr/images/partoches.jpg",
  twitterCard: "summary_large_image",
});

const runtimeConfig = useRuntimeConfig();

// configure internal service client
client.setConfig({
  // set default base url for requests
  baseURL: import.meta.server
    ? runtimeConfig.private_api_base_url
    : runtimeConfig.public.api_base_url,
});
</script>

<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>
