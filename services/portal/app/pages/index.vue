<script setup lang="ts">
import { eachWeekOfInterval, isAfter, isBefore } from "date-fns";
import type { RouteLocationRaw } from "vue-router";

// The dashboard is reserved to the Orga group; others land on their profile.
const user = await currentUser();
if (!user.value.groups.includes("Orga")) {
  await navigateTo("/profile", { replace: true });
}

const { status: semestersStatus } = useSemesters();
const { semester, concerts, pieces, pieceById, loading } = useProgramme();
const { place, placeName } = useLocations();

const upcomingConcerts = computed(() =>
  concerts.value.filter((c) => daysUntil(c.date) >= 0),
);

/** « 14 sept. 2026 – 24 janv. 2027 · semaine 7 sur 19 » */
const period = computed(() => {
  const s = semester.value;
  if (!s) return "";
  const range = `${formatDay(s.start_date)} – ${formatDay(s.end_date)}`;
  const start = parseDay(s.start_date);
  const end = parseDay(s.end_date);
  const today = new Date();
  if (isBefore(today, start)) {
    return `${range} · commence dans ${daysUntil(s.start_date)} jours`;
  }
  if (isAfter(today, end)) return `${range} · terminé`;
  const weeks = eachWeekOfInterval({ start, end }, { weekStartsOn: 1 });
  const current = eachWeekOfInterval(
    { start, end: today },
    { weekStartsOn: 1 },
  );
  return `${range} · semaine ${current.length} sur ${weeks.length}`;
});

const stats = computed(() => {
  const next = upcomingConcerts.value[0];
  return [
    { value: concerts.value.length, label: "concerts" },
    { value: pieces.value.length, label: "morceaux" },
    ...(next
      ? [{ value: countdown(next.date), label: "prochain concert" }]
      : []),
  ];
});

/** A page of the semester, with extra query parameters. */
const pageLink = (
  path: string,
  query: Record<string, string | number> = {},
) => ({
  path,
  query: { semester: semester.value?.id, ...query },
});

type TodoItem = { label: string; icon: string; to: RouteLocationRaw };

/** A card: either a single action, or a list of points, each with its link. */
type Todo = {
  key: string;
  tag: string;
  title: string;
  detail: string;
} & (
  | { action: string; to: RouteLocationRaw; items?: undefined }
  | { items: TodoItem[] }
);

/** Concerts page, with a concert selected and one of its dialogs opened. */
const concertLink = (concertId: number, edit: "concert" | "programme") =>
  pageLink("/concerts", { concert: concertId, edit });

/** Things the organisers should look at, derived from the semester data. */
const todos = computed<Todo[]>(() => {
  if (loading.value) return [];
  const result: Todo[] = [];

  if (!concerts.value.length) {
    result.push({
      key: "no-concert",
      to: pageLink("/concerts", { edit: "new" }),
      tag: "Concerts",
      title: "Aucun concert prévu ce semestre",
      detail: "Ajoutez les dates pour pouvoir construire les programmes.",
      action: "Ajouter un concert",
    });
  }
  if (!pieces.value.length) {
    result.push({
      key: "no-piece",
      to: pageLink("/pieces", { edit: "new" }),
      tag: "Morceaux",
      title: "Aucun morceau pour ce semestre",
      detail: "Ajoutez les morceaux travaillés ce semestre.",
      action: "Ajouter des morceaux",
    });
  }

  // One card per upcoming concert, listing what it is missing.
  for (const concert of upcomingConcerts.value) {
    const edit = concertLink(concert.id, "concert");
    const items: TodoItem[] = [];
    if (concert.room_id == null) {
      items.push({
        label: "Pas encore de salle",
        icon: "i-lucide-map-pin",
        to: edit,
      });
    } else if (!concert.room_booked) {
      items.push({
        label: "Réservation de la salle à confirmer",
        icon: "i-lucide-clock",
        to: edit,
      });
    }
    if (!concert.starts_at) {
      items.push({
        label: "Heure de début à fixer",
        icon: "i-lucide-alarm-clock",
        to: edit,
      });
    }
    if (!concert.piece_ids.length) {
      items.push({
        label: "Programme vide",
        icon: "i-lucide-list-music",
        to: concertLink(concert.id, "programme"),
      });
    }
    if (!items.length) continue;

    result.push({
      key: `concert-${concert.id}`,
      tag: "Concert",
      title: concertTitle(concert),
      detail: [
        concert.name ? formatLongDay(concert.date) : undefined,
        placeName(concert.room_id),
        countdown(concert.date),
      ]
        .filter(Boolean)
        .join(" · "),
      items,
    });
  }

  const scheduled = new Set(concerts.value.flatMap((c) => c.piece_ids));
  const unscheduled = pieces.value.filter((p) => !scheduled.has(p.id));
  if (concerts.value.length && unscheduled.length) {
    const names = unscheduled.map((p) => p.name);
    result.push({
      key: "unscheduled",
      to: pageLink("/pieces", { filter: "unscheduled" }),
      tag: "Morceaux",
      title:
        unscheduled.length === 1
          ? `« ${names[0]} » n’est au programme d’aucun concert`
          : `${unscheduled.length} morceaux ne sont au programme d’aucun concert`,
      detail:
        unscheduled.length === 1
          ? "Ajoutez-le à un programme ou retirez-le du semestre."
          : names.join(", "),
      action: "Ajouter aux programmes",
    });
  }

  return result;
});

const todoCount = computed(() =>
  todos.value.reduce((n, todo) => n + (todo.items?.length ?? 1), 0),
);
</script>

<template>
  <div v-if="semester" class="flex flex-col gap-7">
    <header class="flex flex-wrap items-end justify-between gap-5">
      <div class="flex flex-col gap-1.5">
        <span class="text-sm font-semibold tracking-wider text-muted uppercase">
          Semestre {{ semester.short_name }}
        </span>
        <h1
          class="text-4xl leading-none font-bold tracking-tight text-highlighted sm:text-5xl"
        >
          {{ semester.long_name }}
        </h1>
        <p class="text-muted">{{ period }}</p>
      </div>
      <dl class="flex flex-wrap gap-3">
        <div
          v-for="stat in stats"
          :key="stat.label"
          class="flex min-w-23 flex-col-reverse gap-0.5 rounded-lg border border-default bg-default px-4 py-2"
        >
          <dt class="text-sm text-muted">{{ stat.label }}</dt>
          <dd class="font-display text-3xl leading-tight font-semibold">
            <USkeleton v-if="loading" class="my-1 h-7 w-10" />
            <template v-else>{{ stat.value }}</template>
          </dd>
        </div>
      </dl>
    </header>

    <section
      v-if="todos.length"
      aria-labelledby="todo-title"
      class="flex flex-col gap-3"
    >
      <h2 id="todo-title" class="text-lg font-semibold">
        À traiter
        <span class="font-sans text-base font-normal text-muted">
          · {{ todoCount }} {{ todoCount > 1 ? "points" : "point" }}
        </span>
      </h2>
      <ul
        class="grid grid-cols-[repeat(auto-fill,minmax(min(18rem,100%),1fr))] gap-3"
      >
        <li
          v-for="todo in todos"
          :key="todo.key"
          class="flex flex-col gap-2.5 rounded-lg border border-warning/30 bg-warning-soft p-4"
        >
          <span
            class="flex items-center gap-2 text-xs font-semibold tracking-wider uppercase"
          >
            <span class="size-2.5 rounded-full bg-warning" />
            {{ todo.tag }}
          </span>
          <span class="leading-snug font-semibold text-pretty">
            {{ todo.title }}
          </span>
          <span class="text-sm text-toned">{{ todo.detail }}</span>
          <ul
            v-if="todo.items"
            class="mt-auto flex flex-col divide-y divide-warning/20 overflow-hidden rounded-md border border-warning/30 bg-default"
          >
            <li v-for="item in todo.items" :key="item.label">
              <ULink
                :to="item.to"
                class="flex items-center gap-2 px-3 py-2 text-sm font-medium text-highlighted hover:bg-elevated"
              >
                <UIcon :name="item.icon" class="shrink-0 text-warning" />
                {{ item.label }}
                <UIcon
                  name="i-lucide-chevron-right"
                  class="ms-auto shrink-0 text-muted"
                />
              </ULink>
            </li>
          </ul>
          <UButton
            v-else
            :to="todo.to"
            :label="todo.action"
            color="primary"
            variant="outline"
            class="mt-auto self-start bg-default"
          />
        </li>
      </ul>
    </section>

    <SemesterTimeline
      :semester="semester"
      :concerts="concerts"
      :place-name="placeName"
    />

    <div
      class="grid grid-cols-[repeat(auto-fit,minmax(min(26rem,100%),1fr))] items-start gap-7"
    >
      <section aria-labelledby="concerts-title" class="flex flex-col gap-3">
        <div class="flex items-baseline justify-between gap-3">
          <h2 id="concerts-title" class="text-lg font-semibold">
            Concerts à venir
          </h2>
          <ULink
            :to="pageLink('/concerts')"
            class="text-sm font-medium text-secondary hover:text-highlighted"
          >
            Tous les concerts
          </ULink>
        </div>
        <USkeleton v-if="loading" class="h-56 rounded-lg" />
        <p
          v-else-if="!upcomingConcerts.length"
          class="rounded-lg border border-dashed border-default p-6 text-center text-muted"
        >
          {{
            concerts.length
              ? "Plus de concert à venir ce semestre"
              : "Aucun concert pour ce semestre"
          }}
        </p>
        <DashboardConcertCard
          v-for="concert in upcomingConcerts"
          :key="concert.id"
          :concert="concert"
          :place="place(concert.room_id)"
          :pieces="pieceById"
        />
      </section>

      <USkeleton v-if="loading" class="h-56 rounded-lg" />
      <DashboardPieces v-else :pieces="pieces" :concerts="concerts" />
    </div>
  </div>

  <UEmpty
    v-else-if="semestersStatus !== 'pending'"
    icon="i-lucide-calendar"
    title="Aucun semestre"
    description="Créez un semestre depuis le menu Semestre pour y ajouter des concerts et des morceaux."
  />
</template>
