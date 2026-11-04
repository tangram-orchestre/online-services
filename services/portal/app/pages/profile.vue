<script setup lang="ts">
useHead({ title: "Mon profil" });

const user = await currentUser();

const runtimeConfig = useRuntimeConfig();
</script>

<template>
  <h1 class="text-2xl font-semibold">Mon profil</h1>
  <div v-if="user" class="mt-4">
    <h2 class="mb-4 text-lg font-medium">Informations personnelles</h2>

    <div class="grid gap-4 sm:grid-cols-2">
      <UFormField label="Nom d'utilisateur">
        <UInput :model-value="user.username" disabled class="w-full" />
      </UFormField>
      <UFormField label="Email">
        <UInput :model-value="user.email" disabled class="w-full" />
      </UFormField>
    </div>

    <UButton
      class="mt-6"
      label="Modifier mes informations"
      icon="i-lucide-external-link"
      :to="runtimeConfig.public.sso_user_settings_url"
      external
    />
  </div>
</template>
