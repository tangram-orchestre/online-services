<script setup lang="ts">
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
</script>

<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>
