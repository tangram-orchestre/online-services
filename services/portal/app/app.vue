<script lang="ts" setup>
import { NuxtPage } from "#components";
import { client } from "#hey-api/client.gen";
import type { NavigationMenuItem } from "@nuxt/ui";
import { fr } from "@nuxt/ui/locale";

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

const navigation = computed<NavigationMenuItem[]>(() =>
  user.value.groups.includes("Orga")
    ? [
        { label: "Semestres", icon: "i-lucide-calendar", to: "/", exact: true },
        { label: "Lieux", icon: "i-lucide-map-pin", to: "/locations" },
      ]
    : [],
);
</script>

<template>
  <UApp :locale="fr" :toaster="{ position: 'top-right', duration: 3000 }">
    <UHeader title="Tangram" to="/">
      <UNavigationMenu :items="navigation" />

      <template #right>
        <UColorModeButton />
        <UButton
          icon="i-lucide-user"
          color="neutral"
          variant="ghost"
          to="/profile"
          aria-label="Mon profil"
        />
      </template>

      <template #body>
        <UNavigationMenu :items="navigation" orientation="vertical" />
      </template>
    </UHeader>

    <UMain>
      <UContainer class="py-6">
        <NuxtPage />
      </UContainer>
    </UMain>
  </UApp>
</template>
