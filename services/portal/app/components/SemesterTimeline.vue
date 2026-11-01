<script setup lang="ts">
import type { ConcertWithPieces, Semester } from "#hey-api/types.gen";
import {
  addDays,
  eachWeekOfInterval,
  endOfDay,
  isWithinInterval,
} from "date-fns";

const props = defineProps<{
  semester: Semester;
  concerts: ConcertWithPieces[];
  placeName: (roomId: number) => string | undefined;
}>();

const weeks = computed(() => {
  const today = new Date();
  return eachWeekOfInterval(
    {
      start: parseDay(props.semester.start_date),
      end: parseDay(props.semester.end_date),
    },
    { weekStartsOn: 1 },
  ).map((monday, index) => {
    const week = { start: monday, end: endOfDay(addDays(monday, 6)) };
    return {
      key: monday.toISOString(),
      number: `S${index + 1}`,
      label: `${monday.getDate()}/${monday.getMonth() + 1}`,
      current: isWithinInterval(today, week),
      past: week.end < today,
      concerts: props.concerts.filter((c) =>
        isWithinInterval(parseDay(c.date), week),
      ),
    };
  });
});

const concertLabel = (concert: ConcertWithPieces) =>
  [`Concert · ${formatLongDay(concert.date)}`, props.placeName(concert.room_id)]
    .filter(Boolean)
    .join(" · ");
</script>

<template>
  <section
    aria-labelledby="timeline-title"
    class="flex flex-col gap-3 rounded-lg border border-default bg-default p-5 pb-4"
  >
    <div class="flex flex-wrap items-baseline justify-between gap-x-6 gap-y-2">
      <h2 id="timeline-title" class="text-lg font-semibold">
        Le semestre en un coup d’œil
      </h2>
      <div class="flex flex-wrap gap-x-5 gap-y-2 text-sm text-muted">
        <span class="flex items-center gap-1.5">
          <span class="concert-marker size-2.5" />
          Concert
        </span>
        <span class="flex items-center gap-1.5">
          <span class="size-3 rounded-sm bg-secondary/20" />
          Semaine en cours
        </span>
      </div>
    </div>

    <div class="overflow-x-auto pb-1">
      <ol
        class="grid min-w-max gap-0.75"
        :style="{
          gridTemplateColumns: `repeat(${weeks.length}, minmax(2.25rem, 1fr))`,
        }"
      >
        <li
          v-for="week in weeks"
          :key="week.key"
          class="flex flex-col items-center gap-1 rounded-md py-2"
          :class="week.current ? 'bg-secondary/20' : 'bg-muted'"
          :aria-current="week.current ? 'date' : undefined"
        >
          <span class="h-4 text-[11px] font-semibold text-secondary">
            {{ week.current ? "Auj." : "" }}
          </span>
          <div class="flex h-7 items-center justify-center gap-2.5">
            <UTooltip
              v-for="concert in week.concerts"
              :key="concert.id"
              :text="concertLabel(concert)"
            >
              <span
                class="concert-marker size-3.5"
                role="img"
                :aria-label="concertLabel(concert)"
              />
            </UTooltip>
          </div>
          <span
            class="text-xs font-semibold"
            :class="week.past ? 'text-dimmed' : 'text-highlighted'"
          >
            {{ week.number }}
          </span>
          <span class="text-[11px] text-muted">{{ week.label }}</span>
        </li>
      </ol>
    </div>
  </section>
</template>

<style scoped>
@reference "../assets/css/main.css";

.concert-marker {
  @apply inline-block shrink-0 rotate-45 border-2 border-inverted bg-tertiary;
}
</style>
