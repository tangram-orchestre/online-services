<script setup lang="ts">
import { match } from "ts-pattern";

defineProps<{
  concert: Concert;
}>();
</script>

<template>
  <div
    class="flex flex-col rounded-3xl bg-slate-900 p-4 text-center text-white shadow-xl"
  >
    <div class="relative">
      <div
        class="absolute right-3 top-3 z-20 flex size-12 flex-col items-center justify-center rounded-full bg-slate-900/80 text-4xl hover:opacity-90"
      >
        <Icon name="ic:round-plus" />
      </div>
      <NuxtPicture
        :src="concert.image"
        :alt="`${concert.date} - ${concert.location}`"
        class="aspect-video"
        quality="70"
        sizes="90vw sm:45vw lg:30vw xl:25vw"
        :img-attrs="{ class: 'aspect-video rounded-2xl border-2 w-full' }"
      />
    </div>
    <div class="flex grow flex-col justify-center">
      <div class="py-4">
        <div
          class="lilita-one-regular text-2xl tracking-wide"
          v-html="concert.location"
        />
        <div class="raleway">{{ concert.date }}</div>
      </div>

      <ConcertLink
        v-if="concert.url"
        :target="concert.url.target"
        :text="
          match(concert.url.kind)
            .with('Video', () => 'Voir la vidéo')
            .with('Tickets', () => 'Acheter mes billets')
            .with('Tickets Unavailable', () => 'Billets bientôt disponibles')
            .exhaustive()
        "
        class="text-slate-950"
        :class="[
          {
            'pointer-events-none': concert.url.kind === 'Tickets Unavailable',
          },
          ...match(concert.url.kind)
            .with('Video', () => ['bg-[#ff5b62]', 'hover:bg-[#f86168]'])
            .with('Tickets', () => ['bg-[#81ccb5]', 'hover:bg-[#8adac2]'])
            .with('Tickets Unavailable', () => [
              'bg-[#81ccb5]',
              'brightness-75',
            ])
            .exhaustive(),
        ]"
      />
    </div>
  </div>
</template>
