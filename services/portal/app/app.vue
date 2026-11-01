<script lang="ts" setup>
import { NuxtPage } from "#components";
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

useHead({
  htmlAttrs: {
    lang: "fr",
  },
});

const user = await currentUser();
const isOrga = computed(() => user.value.groups.includes("Orga"));

const route = useRoute();
const { pageLink } = useSemesters();

type NavItem = { label: string; icon: string; path: string };

const navigation = computed<{ title: string; items: NavItem[] }[]>(() =>
  isOrga.value
    ? [
        {
          title: "Ce semestre",
          items: [
            {
              label: "Tableau de bord",
              icon: "i-lucide-layout-dashboard",
              path: "/",
            },
            { label: "Concerts", icon: "i-lucide-ticket", path: "/concerts" },
            { label: "Morceaux", icon: "i-lucide-music", path: "/pieces" },
          ],
        },
        {
          title: "Tous semestres",
          items: [
            { label: "Lieux", icon: "i-lucide-map-pin", path: "/locations" },
            { label: "Personnes", icon: "i-lucide-users", path: "/people" },
          ],
        },
      ]
    : [],
);
</script>

<template>
  <UApp :locale="fr" :toaster="{ position: 'top-right', duration: 3000 }">
    <UDashboardGroup unit="rem">
      <UDashboardSidebar
        :default-size="16"
        :min-size="16"
        :max-size="16"
        class="surface-inverted bg-canvas"
        :ui="{
          content: 'surface-inverted bg-canvas max-w-xs',
          header: 'h-auto pt-7 pb-2',
          body: 'gap-8 py-4',
          footer: 'pb-4',
        }"
      >
        <template #header>
          <NuxtLink :to="pageLink('/')" class="rounded-md">
            <AppLogo
              :subtitle="isOrga ? 'Espace organisation' : 'Espace membre'"
            />
          </NuxtLink>
        </template>

        <SemesterSwitcher v-if="isOrga" />

        <nav aria-label="Navigation principale" class="flex flex-col gap-6">
          <div
            v-for="group in navigation"
            :key="group.title"
            class="flex flex-col gap-1.5"
          >
            <span
              class="text-xs font-semibold tracking-widest text-muted uppercase"
            >
              {{ group.title }}
            </span>
            <NuxtLink
              v-for="item in group.items"
              :key="item.path"
              :to="pageLink(item.path)"
              :aria-current="route.path === item.path ? 'page' : undefined"
              class="flex min-h-11 items-center gap-3 rounded-lg px-3.5 text-[15px] transition-colors hover:bg-elevated aria-[current=page]:bg-inverted aria-[current=page]:font-semibold aria-[current=page]:text-inverted"
            >
              <UIcon :name="item.icon" class="size-4.5 shrink-0" />
              {{ item.label }}
            </NuxtLink>
          </div>
        </nav>

        <SidebarShapes class="mt-auto" />

        <template #footer>
          <UButton
            :label="`${user.first_name} ${user.last_name}`"
            icon="i-lucide-circle-user"
            color="neutral"
            variant="ghost"
            to="/profile"
            class="min-w-0 flex-1"
          />
          <UColorModeButton />
        </template>
      </UDashboardSidebar>

      <UDashboardPanel :ui="{ body: 'p-0 sm:p-0' }">
        <template #header>
          <UDashboardNavbar class="lg:hidden" :ui="{ title: 'font-normal' }">
            <template #title>
              <AppLogo />
            </template>
          </UDashboardNavbar>
        </template>

        <template #body>
          <UContainer class="w-full py-6 lg:py-8">
            <NuxtPage />
          </UContainer>
        </template>
      </UDashboardPanel>
    </UDashboardGroup>
  </UApp>
</template>
