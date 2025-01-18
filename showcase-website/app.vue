<script setup lang="ts">
import { client } from "./client";

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
  ogImage: "http://www.tangram-orchestre.fr/images/partoches.jpg",
  twitterCard: "summary_large_image",
});

// configure internal service client
client.setConfig({
  // set default base url for requests
  baseURL: import.meta.server ? "http://backend:3000" : window.location.origin,
});
</script>

<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>
