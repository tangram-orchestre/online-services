<script setup lang="ts">
import type { ConcertWithPieces, Semester } from "#hey-api/types.gen";
import {
  addDays,
  differenceInCalendarDays,
  eachDayOfInterval,
  eachWeekOfInterval,
  endOfDay,
  isSameDay,
  isSameMonth,
  isWithinInterval,
} from "date-fns";

const props = defineProps<{
  semester: Semester;
  concerts: ConcertWithPieces[];
  placeName: (roomId?: number | null) => string | undefined;
}>();

const dayMonth = (date: Date, month: "short" | "long") =>
  date.toLocaleDateString("fr", { day: "numeric", month });

/** « du 5 au 11 octobre », « du 28 sept. au 4 oct. » */
const weekRange = (monday: Date, sunday: Date) =>
  isSameMonth(monday, sunday)
    ? `du ${monday.getDate()} au ${dayMonth(sunday, "long")}`
    : `du ${dayMonth(monday, "short")} au ${dayMonth(sunday, "short")}`;

/** « dim. 11 » */
const concertDay = (concert: ConcertWithPieces) =>
  parseDay(concert.date).toLocaleDateString("fr", {
    weekday: "short",
    day: "numeric",
  });

/** Position of a day in its week, from 0 (Monday start) to 1 (Sunday end). */
const dayOffset = (monday: Date, day: Date, middle = false) =>
  ((differenceInCalendarDays(day, monday) + (middle ? 0.5 : 0)) / 7) * 100;

const weeks = computed(() => {
  const today = new Date();
  const semesterStart = parseDay(props.semester.start_date);
  const semesterEnd = parseDay(props.semester.end_date);
  return eachWeekOfInterval(
    { start: semesterStart, end: semesterEnd },
    { weekStartsOn: 1 },
  ).map((monday, index) => {
    const sunday = addDays(monday, 6);
    const week = { start: monday, end: endOfDay(sunday) };
    // Months starting this week (the first one at the semester start), placed
    // on their first day.
    const months = eachDayOfInterval(week)
      .filter(
        (day) =>
          isWithinInterval(day, { start: semesterStart, end: semesterEnd }) &&
          (day.getDate() === 1 || isSameDay(day, semesterStart)),
      )
      .map((day) => ({
        key: day.toISOString(),
        label: day.toLocaleDateString("fr", { month: "short" }),
        offset: dayOffset(monday, day),
      }));
    const current = isWithinInterval(today, week);
    return {
      key: monday.toISOString(),
      number: `S${index + 1}`,
      title: `${weekRange(monday, sunday)}`,
      months,
      // Middle of today, to mark it inside the week.
      today: current ? dayOffset(monday, today, true) : undefined,
      current,
      past: week.end < today,
      concerts: props.concerts.filter((c) =>
        isWithinInterval(parseDay(c.date), week),
      ),
    };
  });
});

/** Weeks get wider with each concert, so that their dates stay apart. */
const columns = computed(() => {
  const widths = weeks.value.map((week) => {
    const n = week.concerts.length;
    return n === 0 ? { min: 1.75, fr: 1 } : { min: n * 2.75, fr: n };
  });
  return {
    template: widths.map((w) => `minmax(${w.min}rem, ${w.fr}fr)`).join(" "),
    // Below that, the timeline scrolls horizontally.
    minWidth: `calc(${widths.reduce((sum, w) => sum + w.min, 0)}rem + ${widths.length - 1} * 3px)`,
  };
});

// Start the timeline at the week before today rather than at the semester
// start, once the columns have their final widths (after the concerts load).
const scroller = useTemplateRef<HTMLElement>("scroller");
watch(
  [scroller, () => columns.value.template],
  () => {
    const el = scroller.value;
    if (!el) return;
    const current = weeks.value.findIndex((w) => w.current);
    const week =
      el.querySelectorAll<HTMLElement>("li")[Math.max(current - 1, 0)];
    el.scrollLeft = current < 0 || !week ? 0 : week.offsetLeft;
  },
  { flush: "post" },
);

const { pageLink } = useSemesters();
/** Concerts page, with the concert selected. */
const concertLink = (concert: ConcertWithPieces) =>
  pageLink("/concerts", { concert: concert.id });

const programmeLabel = (concert: ConcertWithPieces) => {
  const n = concert.piece_ids.length;
  if (!n) return "Programme vide";
  return n === 1 ? "1 morceau au programme" : `${n} morceaux au programme`;
};

// The week tooltip stays closed while a concert card is open, including
// while the pointer moves from the concert to its card.
const hoveredWeek = ref<string>();
const openConcert = ref<number>();
// Moving to the next week opens it before the previous one closes: a closing
// week only clears itself.
const onWeekOpen = (weekKey: string, open: boolean) => {
  if (open) hoveredWeek.value = weekKey;
  else if (hoveredWeek.value === weekKey) hoveredWeek.value = undefined;
};
const onConcertOpen = (concertId: number, open: boolean) => {
  if (open) openConcert.value = concertId;
  else if (openConcert.value === concertId) openConcert.value = undefined;
};

const concertLabel = (concert: ConcertWithPieces) =>
  [
    concert.name ?? "Concert",
    formatLongDay(concert.date),
    props.placeName(concert.room_id),
  ]
    .filter(Boolean)
    .join(" · ");
</script>

<template>
  <section aria-labelledby="timeline-title" class="flex flex-col gap-3">
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

    <div
      ref="scroller"
      class="relative overflow-x-auto rounded-lg border border-default bg-default p-5 pb-4"
    >
      <ol
        class="grid gap-x-0.75"
        :style="{
          gridTemplateColumns: columns.template,
          minWidth: columns.minWidth,
        }"
      >
        <li v-for="week in weeks" :key="week.key" class="flex flex-col">
          <!-- Month starts and today, placed on their day within the week. -->
          <div class="relative h-5">
            <span
              v-for="month in week.months"
              :key="month.key"
              class="absolute inset-y-0 border-s-2 border-default ps-1.5 text-xs font-semibold whitespace-nowrap text-toned"
              :style="{ left: `${month.offset}%` }"
            >
              {{ month.label }}
            </span>
          </div>
          <div class="relative h-5">
            <span
              v-if="week.today !== undefined"
              class="absolute bottom-0 flex -translate-x-1/2 flex-col items-center text-[11px] leading-none font-semibold text-secondary"
              :style="{ left: `${week.today}%` }"
            >
              Auj.
              <span class="mt-0.5 h-1.5 w-0.5 bg-secondary" />
            </span>
          </div>
          <UTooltip
            :text="week.title"
            :content="{ side: 'bottom' }"
            :open="hoveredWeek === week.key && openConcert === undefined"
            @update:open="(open) => onWeekOpen(week.key, open)"
          >
            <div
              class="relative flex flex-1 flex-col items-center gap-1 rounded-md pt-3 pb-2"
              :class="week.current ? 'bg-secondary/20' : 'bg-muted'"
              :aria-current="week.current ? 'date' : undefined"
            >
              <span
                v-if="week.today !== undefined"
                class="absolute inset-y-0 w-0.5 -translate-x-1/2 bg-secondary/50"
                :style="{ left: `${week.today}%` }"
              />
              <!-- Only concerts get a date: a date under a week would read
                   as the date of its concert. -->
              <div class="relative flex h-12 items-start justify-center gap-2">
                <UPopover
                  v-for="concert in week.concerts"
                  :key="concert.id"
                  mode="hover"
                  :open-delay="150"
                  :close-delay="200"
                  :content="{ side: 'top' }"
                  @update:open="(open) => onConcertOpen(concert.id, open)"
                >
                  <ULink
                    :to="concertLink(concert)"
                    :aria-label="concertLabel(concert)"
                    class="flex w-10 flex-col items-center gap-2 rounded-sm pt-1 hover:bg-default/60"
                  >
                    <span class="concert-marker size-3.5" />
                    <span
                      class="text-[11px] font-semibold whitespace-nowrap text-highlighted"
                    >
                      {{ concertDay(concert) }}
                    </span>
                  </ULink>
                  <template #content>
                    <div class="flex w-64 flex-col gap-2 p-3 text-sm">
                      <div class="flex flex-col">
                        <span class="font-semibold text-highlighted">
                          {{ concertTitle(concert) }}
                        </span>
                        <span
                          v-if="concert.name"
                          class="text-muted first-letter:uppercase"
                        >
                          {{ formatLongDay(concert.date) }}
                        </span>
                      </div>
                      <div class="flex flex-col gap-1 text-toned">
                        <span class="flex items-center gap-2">
                          <UIcon
                            name="i-lucide-map-pin"
                            class="shrink-0 text-muted"
                          />
                          {{ placeName(concert.room_id) ?? "Salle à définir" }}
                        </span>
                        <span class="flex items-center gap-2">
                          <UIcon
                            name="i-lucide-clock"
                            class="shrink-0 text-muted"
                          />
                          {{
                            concert.starts_at
                              ? `Début à ${formatTime(concert.starts_at)}`
                              : "Horaire à définir"
                          }}
                        </span>
                        <span class="flex items-center gap-2">
                          <UIcon
                            name="i-lucide-list-music"
                            class="shrink-0 text-muted"
                          />
                          {{ programmeLabel(concert) }}
                        </span>
                      </div>
                      <UBadge
                        v-if="concert.room_id != null && !concert.room_booked"
                        label="Réservation à confirmer"
                        icon="i-lucide-clock"
                        color="warning"
                        variant="subtle"
                        class="self-start"
                      />
                      <UButton
                        :to="concertLink(concert)"
                        label="Voir le concert"
                        trailing-icon="i-lucide-arrow-right"
                        color="primary"
                        variant="outline"
                        size="sm"
                        class="self-start"
                      />
                    </div>
                  </template>
                </UPopover>
              </div>
              <span
                class="relative text-xs font-semibold"
                :class="week.past ? 'text-dimmed' : 'text-highlighted'"
              >
                {{ week.number }}
              </span>
            </div>
          </UTooltip>
        </li>
      </ol>
    </div>
  </section>
</template>
