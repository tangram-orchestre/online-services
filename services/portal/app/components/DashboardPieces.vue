<script setup lang="ts">
const { pageLink } = useSemesters();
const { pieces, concertsOf } = useProgramme();
const { filter, filters, filtered } = usePieceFilter();
</script>

<template>
  <section aria-labelledby="pieces-title" class="flex flex-col gap-3">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <div class="flex items-baseline gap-3">
        <h2 id="pieces-title" class="text-lg font-semibold">
          Morceaux du semestre
        </h2>
        <ULink
          :to="pageLink('/pieces')"
          class="text-sm font-medium text-secondary hover:text-highlighted"
        >
          Tous les morceaux
        </ULink>
      </div>
      <FilterChips
        v-if="pieces.length"
        v-model="filter"
        :items="filters"
        label="Filtrer par concert"
      />
    </div>

    <ListCard
      header-class="sm:grid sm:grid-cols-[3fr_2fr]"
      :empty="filtered.length ? undefined : 'Aucun morceau pour ce semestre'"
    >
      <template #header>
        <span>Morceau</span>
        <span>Concerts</span>
      </template>
      <li
        v-for="piece in filtered"
        :key="piece.id"
        class="grid gap-2 px-5 py-3.5 sm:grid-cols-[3fr_2fr] sm:items-center sm:gap-4"
      >
        <div class="flex min-w-0 flex-col gap-0.5">
          <span class="font-display text-lg leading-snug font-semibold">
            {{ piece.name }}
          </span>
          <span v-if="pieceCredits(piece)" class="text-sm text-muted">
            {{ pieceCredits(piece) }}
          </span>
        </div>
        <div class="flex flex-wrap gap-1.5">
          <ConcertChip
            v-for="concert in concertsOf(piece.id)"
            :key="concert.id"
            :concert="concert"
          />
          <span v-if="!concertsOf(piece.id).length" class="text-sm text-dimmed">
            Hors programme
          </span>
        </div>
      </li>
    </ListCard>
  </section>
</template>
