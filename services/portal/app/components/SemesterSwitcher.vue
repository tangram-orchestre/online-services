<script setup lang="ts">
import type { DropdownMenuItem } from "@nuxt/ui";

const { sortedSemesters, selectedSemesterId, selectedSemester } =
  useSemesters();

const selectedIndex = computed(() =>
  sortedSemesters.value.findIndex((s) => s.id === selectedSemesterId.value),
);

// Semesters are sorted most recent first.
const previous = computed(() => sortedSemesters.value[selectedIndex.value + 1]);
const next = computed(() =>
  selectedIndex.value > 0
    ? sortedSemesters.value[selectedIndex.value - 1]
    : undefined,
);

const items = computed<DropdownMenuItem[]>(() =>
  sortedSemesters.value.map((s) => ({
    label: `${s.short_name} — ${s.long_name}`,
    type: "checkbox" as const,
    checked: s.id === selectedSemesterId.value,
    onSelect: () => {
      selectedSemesterId.value = s.id;
    },
  })),
);
</script>

<template>
  <div v-if="selectedSemester" class="flex flex-col gap-2">
    <span
      id="semester-switcher-label"
      class="text-xs font-semibold tracking-widest text-muted uppercase"
    >
      Semestre
    </span>
    <div
      role="group"
      aria-labelledby="semester-switcher-label"
      class="flex items-stretch rounded-xl border border-default bg-muted"
    >
      <UButton
        icon="i-lucide-chevron-left"
        color="neutral"
        variant="link"
        aria-label="Semestre précédent"
        :disabled="!previous"
        class="px-3"
        @click="selectedSemesterId = previous?.id"
      />
      <UDropdownMenu :items="items">
        <button
          type="button"
          class="flex flex-1 flex-col items-center rounded-lg py-2 leading-tight hover:bg-elevated focus-visible:outline-2 focus-visible:outline-primary"
          aria-label="Choisir un semestre"
        >
          <span class="font-display text-xl font-bold text-highlighted">
            {{ selectedSemester.short_name }}
          </span>
          <span class="text-xs text-muted">
            {{ selectedSemester.long_name }}
          </span>
        </button>
      </UDropdownMenu>
      <UButton
        icon="i-lucide-chevron-right"
        color="neutral"
        variant="link"
        aria-label="Semestre suivant"
        :disabled="!next"
        class="px-3"
        @click="selectedSemesterId = next?.id"
      />
    </div>
  </div>
</template>
