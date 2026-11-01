<script setup lang="ts">
import type { LocationWithDetails as Location } from "#hey-api/types.gen";

const route = useRoute();
const router = useRouter();
const toast = useToast();

const { locations, refresh, status, place } = useLocations();

const { data: allConcerts } = getConcerts({
  composable: "useAsyncData",
  key: "getConcerts",
});

const { selectedSemester: semester } = useSemesters();

const search = ref("");

const normalize = (value: string) =>
  value
    .normalize("NFD")
    .replace(/\p{Diacritic}/gu, "")
    .toLowerCase();

const filteredLocations = computed(() => {
  const query = normalize(search.value.trim());
  return [...(locations.value ?? [])]
    .filter(
      (l) =>
        !query ||
        normalize(
          [
            l.name,
            l.city,
            l.street,
            l.zipcode,
            ...l.rooms.map((r) => r.name),
          ].join(" "),
        ).includes(query),
    )
    .sort((a, b) => a.name.localeCompare(b.name, "fr"));
});

/** Concerts of the selected semester, by location. */
const semesterConcerts = computed(() => {
  const concerts = (allConcerts.value ?? [])
    .filter((c) => c.semester_id === semester.value?.id)
    .sort((a, b) => a.date.localeCompare(b.date));
  const byLocation = new Map<number, typeof concerts>();
  for (const concert of concerts) {
    const locationId = place(concert.room_id)?.location.id;
    if (locationId === undefined) continue;
    byLocation.set(locationId, [
      ...(byLocation.get(locationId) ?? []),
      concert,
    ]);
  }
  return byLocation;
});

/** Concerts count by room, all semesters: these rooms can't be removed. */
const roomConcerts = computed(() => {
  const counts = new Map<number, number>();
  for (const concert of allConcerts.value ?? []) {
    if (concert.room_id == null) continue;
    counts.set(concert.room_id, (counts.get(concert.room_id) ?? 0) + 1);
  }
  return counts;
});
const concertsAt = (location: Location) =>
  semesterConcerts.value.get(location.id) ?? [];

const concertCount = (count: number) =>
  count === 0
    ? "Aucun concert ce semestre"
    : `${count} concert${count > 1 ? "s" : ""} ce semestre`;

// Selected location (or "new") is kept in the URL, like the semester.
const selection = computed<number | "new" | undefined>({
  get: () => {
    if (route.query.location === "new") return "new";
    const id = Number(route.query.location);
    return locations.value?.some((l) => l.id === id) ? id : undefined;
  },
  set: (value) => {
    router.replace({ query: { ...route.query, location: value } });
  },
});

const selectedLocation = computed(() =>
  locations.value?.find((l) => l.id === selection.value),
);

const onSaved = (location: Location) => {
  if (locations.value) {
    const exists = locations.value.some((l) => l.id === location.id);
    locations.value = exists
      ? locations.value.map((l) => (l.id === location.id ? location : l))
      : [...locations.value, location];
  }
  selection.value = location.id;
};

const onCancel = () => {
  if (selection.value === "new") selection.value = undefined;
};

const deleteLocation = (location: Location) => {
  confirmDialog(
    "Confirmer la suppression",
    `Êtes-vous sûr de vouloir supprimer le lieu « ${location.name} » ?`,
  ).then((confirmed) => {
    if (!confirmed) return;
    deleteLocationByLocationId({
      composable: "$fetch",
      path: { location_id: location.id },
      onResponseError: onResponseErrorHandler("lieu", false, (errorMsg) => {
        toast.add({ title: errorMsg, color: "error" });
        refresh();
      }),
    }).then(() => {
      if (locations.value) {
        locations.value = locations.value.filter((l) => l.id !== location.id);
      }
      selection.value = undefined;
      toast.add({ title: "Lieu supprimé avec succès", color: "success" });
    });
  });
};
</script>

<template>
  <div class="flex flex-col gap-7">
    <header class="flex flex-wrap items-end justify-between gap-5">
      <div class="flex flex-col gap-1.5">
        <span class="text-sm font-semibold tracking-wider text-muted uppercase">
          Tous semestres
        </span>
        <h1
          class="text-4xl leading-none font-bold tracking-tight text-highlighted sm:text-5xl"
        >
          Lieux
        </h1>
        <p class="text-muted">
          Les lieux de répétition et de concert de Tangram, partagés par tous
          les semestres.
        </p>
      </div>
      <UButton
        label="Ajouter un lieu"
        icon="i-lucide-plus"
        size="xl"
        @click="selection = 'new'"
      />
    </header>

    <div
      class="grid items-start gap-6 lg:grid-cols-[minmax(16rem,20rem)_minmax(0,1fr)]"
    >
      <!-- On small screens, the list and the detail are shown one at a time. -->
      <section
        aria-label="Liste des lieux"
        class="flex-col gap-3"
        :class="selection === undefined ? 'flex' : 'hidden lg:flex'"
      >
        <UInput
          v-model="search"
          icon="i-lucide-search"
          placeholder="Rechercher un lieu"
          size="xl"
          class="w-full"
          :ui="{ base: 'rounded-xl' }"
        />

        <template v-if="status === 'pending' && !locations">
          <USkeleton v-for="i in 3" :key="i" class="h-24 rounded-lg" />
        </template>
        <p
          v-else-if="!filteredLocations.length"
          class="rounded-lg border border-dashed border-default p-6 text-center text-muted"
        >
          {{ search ? "Aucun lieu ne correspond" : "Aucun lieu" }}
        </p>

        <ul class="flex flex-col gap-3">
          <li v-for="location in filteredLocations" :key="location.id">
            <button
              type="button"
              class="flex w-full flex-col gap-1 rounded-lg border bg-default p-4 text-start transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary"
              :class="
                selection === location.id
                  ? 'border-inverted ring-1 ring-inverted'
                  : 'border-default hover:border-accented'
              "
              :aria-current="selection === location.id ? 'true' : undefined"
              @click="selection = location.id"
            >
              <span class="font-display text-lg font-bold text-highlighted">
                {{ location.name }}
              </span>
              <span class="text-sm text-toned">
                {{ location.zipcode }} {{ location.city }}
              </span>
              <span class="flex flex-wrap gap-1.5 py-1">
                <span
                  v-for="room in location.rooms"
                  :key="room.id"
                  class="rounded-full px-2.5 py-0.5 text-xs font-medium"
                  :class="
                    isRehearsalOnly(room)
                      ? 'bg-secondary/15'
                      : 'bg-tertiary-200 text-on-tertiary'
                  "
                >
                  {{ room.name }}
                </span>
              </span>
              <span class="text-sm text-muted">
                {{ concertCount(concertsAt(location).length) }}
              </span>
            </button>
          </li>
        </ul>
      </section>

      <div
        class="flex-col gap-5"
        :class="selection === undefined ? 'hidden lg:flex' : 'flex'"
      >
        <UButton
          label="Tous les lieux"
          icon="i-lucide-arrow-left"
          color="neutral"
          variant="link"
          class="self-start px-0 lg:hidden"
          @click="selection = undefined"
        />

        <template v-if="selection === 'new' || selectedLocation">
          <LocationEditor
            :key="selection"
            :location="selectedLocation"
            :room-concerts="roomConcerts"
            @saved="onSaved"
            @cancel="onCancel"
          />

          <section
            v-if="selectedLocation"
            aria-labelledby="usage-title"
            class="flex flex-col gap-1 rounded-lg border border-default bg-default p-5 sm:p-6"
          >
            <div
              class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1 border-b border-default pb-3"
            >
              <h2 id="usage-title" class="text-lg font-semibold">
                Utilisation
                <template v-if="semester">
                  · {{ semester.short_name }}
                </template>
              </h2>
              <span class="text-sm text-muted">
                {{ concertCount(concertsAt(selectedLocation).length) }}
              </span>
              <p class="w-full text-xs text-muted">
                Suit le semestre choisi dans le menu.
              </p>
            </div>
            <ul v-if="concertsAt(selectedLocation).length">
              <li
                v-for="concert in concertsAt(selectedLocation)"
                :key="concert.id"
                class="flex flex-wrap items-center gap-x-4 gap-y-1 border-b border-muted py-3 last:border-b-0"
              >
                <span class="w-32 font-semibold first-letter:uppercase">
                  {{ formatLongDay(concert.date) }}
                </span>
                <span
                  class="rounded-full bg-tertiary-200 px-2.5 py-0.5 text-sm font-medium text-on-tertiary"
                >
                  {{ concert.name || "Concert" }}
                </span>
                <span class="text-toned">
                  {{ place(concert.room_id)?.room.name }} · Début
                  {{
                    concert.starts_at
                      ? formatTime(concert.starts_at)
                      : "à définir"
                  }}
                </span>
              </li>
            </ul>
            <p v-else class="py-3 text-muted">
              Ce lieu n’est utilisé par aucun concert de ce semestre.
            </p>
          </section>

          <UButton
            v-if="selectedLocation"
            label="Supprimer ce lieu"
            icon="i-lucide-trash-2"
            color="error"
            variant="outline"
            size="lg"
            class="self-start"
            @click="deleteLocation(selectedLocation)"
          />
        </template>

        <UEmpty
          v-else
          icon="i-lucide-map-pin"
          title="Aucun lieu sélectionné"
          description="Choisissez un lieu dans la liste pour le consulter ou le modifier."
          class="rounded-lg border border-dashed border-default"
        />
      </div>
    </div>
  </div>
</template>
