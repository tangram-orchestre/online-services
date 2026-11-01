<script setup lang="ts">
defineProps<{
  concert: { name?: string | null; date: string };
}>();
</script>

<template>
  <span
    class="rounded-full bg-tertiary-200 px-2.5 py-0.5 text-sm font-medium text-on-tertiary"
  >
    <slot>{{ concertShortName(concert) }}</slot>
  </span>
</template>
