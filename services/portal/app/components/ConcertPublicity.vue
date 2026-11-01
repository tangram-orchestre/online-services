<script setup lang="ts">
import type { Concert } from "#hey-api/types.gen";

/** Public or private, and the ticketing and video links. */
defineProps<{
  concert: Concert;
}>();
</script>

<template>
  <UBadge
    :label="concert.public ? 'Public' : 'Privé'"
    :icon="concert.public ? 'i-lucide-globe' : 'i-lucide-lock'"
    :color="concert.public ? 'success' : 'neutral'"
    variant="subtle"
    class="rounded-full"
  />
  <UButton
    v-if="concert.tickets_url"
    :to="concert.tickets_url"
    target="_blank"
    label="Billetterie"
    icon="i-lucide-ticket"
    color="neutral"
    variant="link"
    size="sm"
  />
  <UButton
    v-if="concert.video_url"
    :to="concert.video_url"
    target="_blank"
    label="Vidéo"
    icon="i-lucide-circle-play"
    color="neutral"
    variant="link"
    size="sm"
  />
</template>
