<script setup lang="ts">
import type { ConcertWithPieces, Piece } from "#hey-api/types.gen";
import type { Place } from "~/composables/useLocations";

const props = defineProps<{
  concert: ConcertWithPieces;
  place?: Place;
  pieces: Map<number, Piece>;
}>();

const programme = computed(() =>
  props.concert.piece_ids
    .map((id) => props.pieces.get(id))
    .filter((p): p is Piece => !!p),
);
const remaining = computed(() => countdown(props.concert.date));
</script>

<template>
  <article
    class="flex flex-col gap-4 rounded-lg border border-default bg-default p-5"
  >
    <div class="flex flex-wrap items-start justify-between gap-4">
      <div class="flex min-w-0 items-center gap-4">
        <DateTile :date="concert.date" />
        <div class="flex min-w-0 flex-col gap-0.5">
          <h3 class="text-2xl leading-tight font-bold text-highlighted">
            {{ concert.name || place?.location.name || "Salle à définir" }}
          </h3>
          <span class="text-sm text-muted">
            {{
              concert.starts_at
                ? formatTime(concert.starts_at)
                : "Horaire à définir"
            }}
            <template v-if="concert.name">
              · {{ place?.location.name ?? "Salle à définir" }}
            </template>
            <template v-if="place">
              · {{ place.room.name }} · {{ place.location.city }}
            </template>
          </span>
        </div>
      </div>
      <span
        v-if="remaining"
        class="font-display text-3xl leading-none font-bold text-highlighted"
      >
        {{ remaining }}
      </span>
      <span v-else class="text-sm text-muted">Passé</span>
    </div>

    <div class="flex flex-col gap-1 border-t border-muted pt-3.5">
      <span class="text-xs font-semibold tracking-wider text-muted uppercase">
        Programme
      </span>
      <ol v-if="programme.length" class="flex flex-col gap-0.5">
        <li v-for="piece in programme" :key="piece.id" class="font-medium">
          {{ piece.name }}
          <span v-if="piece.composer" class="text-sm font-normal text-muted">
            · {{ piece.composer }}
          </span>
        </li>
      </ol>
      <p v-else class="text-muted">Programme vide</p>
    </div>

    <div class="flex flex-wrap items-center gap-2">
      <UBadge
        :label="concert.public ? 'Public' : 'Privé'"
        :icon="concert.public ? 'i-lucide-globe' : 'i-lucide-lock'"
        :color="concert.public ? 'success' : 'neutral'"
        variant="subtle"
        class="rounded-full"
      />
      <UBadge
        v-if="place && !concert.room_booked"
        label="Salle non confirmée"
        icon="i-lucide-clock"
        color="warning"
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
    </div>
  </article>
</template>
