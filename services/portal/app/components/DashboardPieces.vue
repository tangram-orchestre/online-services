<script setup lang="ts">
import type { ConcertWithPieces, Piece } from "#hey-api/types.gen";

const props = defineProps<{
  pieces: Piece[];
  concerts: ConcertWithPieces[];
}>();

/** A concert id, or the special filters. */
type Filter = number | "all" | "unscheduled";
const filter = ref<Filter>("all");

const concertsOf = (piece: Piece) =>
  props.concerts.filter((c) => c.piece_ids.includes(piece.id));

const unscheduled = computed(() =>
  props.pieces.filter((p) => concertsOf(p).length === 0),
);

const filters = computed(() => [
  { value: "all" as Filter, label: `Tous (${props.pieces.length})` },
  ...props.concerts.map((c) => ({
    value: c.id as Filter,
    label: formatShortDay(c.date),
  })),
  ...(unscheduled.value.length
    ? [
        {
          value: "unscheduled" as Filter,
          label: `Hors programme (${unscheduled.value.length})`,
        },
      ]
    : []),
]);

// Reset when the selected concert disappears (e.g. semester change).
watch(filters, (items) => {
  if (!items.some((f) => f.value === filter.value)) filter.value = "all";
});

const rows = computed(() => {
  const f = filter.value;
  const pieces =
    f === "all"
      ? props.pieces
      : f === "unscheduled"
        ? unscheduled.value
        : props.pieces.filter((p) =>
            props.concerts.find((c) => c.id === f)?.piece_ids.includes(p.id),
          );
  return [...pieces]
    .sort((a, b) => a.name.localeCompare(b.name, "fr"))
    .map((piece) => ({ piece, concerts: concertsOf(piece) }));
});
</script>

<template>
  <section aria-labelledby="pieces-title" class="flex flex-col gap-3">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <h2 id="pieces-title" class="text-lg font-semibold">
        Morceaux du semestre
      </h2>
      <div
        v-if="pieces.length"
        role="group"
        aria-label="Filtrer par concert"
        class="flex flex-wrap gap-2"
      >
        <UButton
          v-for="f in filters"
          :key="f.value"
          :label="f.label"
          :aria-pressed="filter === f.value"
          :color="filter === f.value ? 'primary' : 'neutral'"
          :variant="filter === f.value ? 'solid' : 'outline'"
          class="rounded-full"
          @click="filter = f.value"
        />
      </div>
    </div>

    <div class="rounded-lg border border-default bg-default">
      <div
        class="hidden gap-4 border-b border-default px-5 py-3 text-xs font-semibold tracking-wider text-muted uppercase sm:grid sm:grid-cols-[3fr_2fr]"
      >
        <span>Morceau</span>
        <span>Concerts</span>
      </div>
      <ul>
        <li
          v-for="{ piece, concerts: pieceConcerts } in rows"
          :key="piece.id"
          class="grid gap-2 border-b border-muted px-5 py-3.5 last:border-b-0 sm:grid-cols-[3fr_2fr] sm:items-center sm:gap-4"
        >
          <div class="flex min-w-0 flex-col gap-0.5">
            <span class="font-display text-lg leading-snug font-semibold">
              {{ piece.name }}
            </span>
            <span
              v-if="piece.composer || piece.arranger"
              class="text-sm text-muted"
            >
              {{ piece.composer }}
              <template v-if="piece.arranger">
                <template v-if="piece.composer">·</template>
                arr. {{ piece.arranger }}
              </template>
            </span>
          </div>
          <div class="flex flex-wrap gap-1.5">
            <span
              v-for="concert in pieceConcerts"
              :key="concert.id"
              class="rounded-full bg-tertiary-200 px-2.5 py-0.5 text-sm font-medium text-on-tertiary"
            >
              {{ formatShortDay(concert.date) }}
            </span>
            <span v-if="!pieceConcerts.length" class="text-sm text-dimmed">
              Hors programme
            </span>
          </div>
        </li>
        <li v-if="!rows.length" class="px-5 py-6 text-center text-muted">
          Aucun morceau pour ce semestre
        </li>
      </ul>
    </div>
  </section>
</template>
