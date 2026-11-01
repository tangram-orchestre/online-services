<script setup lang="ts">
import type {
  ConcertWithPieces,
  LocationWithDetails as Location,
  Room,
} from "#hey-api/types.gen";
import type LocationDialogComponent from "~/components/LocationDialog.vue";

definePageMeta({ middleware: "orga" });
useHead({ title: "Lieux" });

const { canEdit } = useRoles();

const route = useRoute();
const router = useRouter();

const { pageLink } = useSemesters();
const { locations, status, place } = useLocations();
const { allConcerts, concerts, semester } = useProgramme();

const search = ref("");

const sorted = computed(() =>
  [...(locations.value ?? [])].sort((a, b) =>
    a.name.localeCompare(b.name, "fr"),
  ),
);

const rows = computed(() =>
  sorted.value.filter((l) =>
    matchesSearch(search.value, [
      l.name,
      l.city,
      l.street,
      l.zipcode,
      ...l.rooms.map((r) => r.name),
    ]),
  ),
);

/** Concerts of the selected semester, by location. */
const semesterConcerts = computed(() => {
  const byLocation = new Map<number, ConcertWithPieces[]>();
  for (const concert of concerts.value) {
    const locationId = place(concert.room_id)?.location.id;
    if (locationId === undefined) continue;
    byLocation.set(locationId, [
      ...(byLocation.get(locationId) ?? []),
      concert,
    ]);
  }
  return byLocation;
});
const concertsAt = (location: Location) =>
  semesterConcerts.value.get(location.id) ?? [];

/** Concerts count by room, all semesters: such locations can't be deleted. */
const roomConcerts = computed(() => roomConcertCounts(allConcerts.value ?? []));
const hostsConcerts = (location: Location) =>
  location.rooms.some((r) => roomConcerts.value.has(r.id));

const concertCount = (count: number) =>
  count === 0 ? "Aucun concert" : `${count} concert${count > 1 ? "s" : ""}`;

/** « 40 musiciens · 300 places » */
const capacities = (room: Room) =>
  [
    room.musician_capacity && `${room.musician_capacity} musiciens`,
    room.audience_capacity && `${room.audience_capacity} places`,
    room.total_capacity && `${room.total_capacity} au total`,
  ]
    .filter(Boolean)
    .join(" · ");

const { selectedId, shown, detailOpen, rowState, onDeleted } = useListSelection(
  "location",
  sorted,
  () => rows.value[0],
);

const dialog =
  useTemplateRef<InstanceType<typeof LocationDialogComponent>>(
    "locationDialog",
  );

// `?edit=new` opens the creation dialog. Client side only: the dialog state
// is not transferred from the server.
onMounted(() => {
  if (route.query.edit !== "new") return;
  if (canEdit.value) dialog.value?.open();
  router.replace({ query: { ...route.query, edit: undefined } });
});
</script>

<template>
  <div class="flex flex-col gap-6">
    <PageHeader
      overline="Tous semestres"
      title="Lieux"
      description="Les lieux de répétition et de concert de Tangram, partagés par tous les semestres."
    >
      <template #actions>
        <UButton
          label="Ajouter un lieu"
          icon="i-lucide-plus"
          size="lg"
          :disabled="!canEdit"
          @click="dialog?.open()"
        />
      </template>
    </PageHeader>

    <UInput
      v-model="search"
      icon="i-lucide-search"
      placeholder="Rechercher un lieu ou une salle"
      size="lg"
      class="w-full"
      :ui="{ base: 'bg-default' }"
    />

    <div
      class="grid items-start gap-6 lg:grid-cols-[minmax(0,1fr)_minmax(0,22rem)] xl:grid-cols-[minmax(0,1fr)_minmax(0,26rem)]"
    >
      <USkeleton
        v-if="status === 'pending' && !locations"
        class="h-80 rounded-lg"
      />
      <ListCard
        v-else
        header-class="@2xl:grid @2xl:grid-cols-[minmax(0,4fr)_minmax(0,4fr)_minmax(0,2fr)]"
        :empty="
          rows.length
            ? undefined
            : sorted.length
              ? 'Aucun lieu ne correspond'
              : 'Aucun lieu pour l’instant'
        "
      >
        <template #header>
          <span>Lieu</span>
          <span>Salles</span>
          <span v-if="semester">Semestre {{ semester.short_name }}</span>
        </template>
        <ListRow
          v-for="location in rows"
          :key="location.id"
          :state="rowState(location.id)"
          class="grid gap-1.5 @2xl:grid-cols-[minmax(0,4fr)_minmax(0,4fr)_minmax(0,2fr)] @2xl:items-center @2xl:gap-4"
          @click="selectedId = location.id"
        >
          <span class="flex min-w-0 items-center gap-3">
            <img
              v-if="location.image_id"
              :src="imageUrl(location.image_id)"
              alt=""
              class="aspect-video w-16 shrink-0 rounded object-cover"
            />
            <span
              v-else
              class="flex aspect-video w-16 shrink-0 items-center justify-center rounded bg-muted text-dimmed"
            >
              <UIcon name="i-lucide-map-pin" />
            </span>
            <span class="flex min-w-0 flex-col">
              <span
                class="font-display text-lg leading-snug font-semibold text-highlighted"
              >
                {{ location.name }}
              </span>
              <span class="text-sm text-muted">
                {{ location.zipcode }} {{ location.city }}
              </span>
            </span>
          </span>
          <span class="flex flex-wrap gap-1.5">
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
          <span
            v-if="semester"
            class="text-sm"
            :class="concertsAt(location).length ? 'text-toned' : 'text-dimmed'"
          >
            {{ concertCount(concertsAt(location).length) }}
          </span>
        </ListRow>
      </ListCard>

      <DetailPanel
        v-if="shown"
        v-model:open="detailOpen"
        title="Détails du lieu"
      >
        <div class="flex flex-col gap-6">
          <img
            v-if="shown.image_id"
            :src="imageUrl(shown.image_id)"
            :alt="`Photo de ${shown.name}`"
            class="aspect-video w-full rounded-md object-cover"
          />
          <div class="flex flex-col gap-1">
            <h2
              class="font-display text-2xl leading-tight font-bold text-highlighted xl:text-3xl"
            >
              {{ shown.name }}
            </h2>
            <span class="text-muted">
              {{ shown.street }}, {{ shown.zipcode }} {{ shown.city }}
            </span>
          </div>

          <section class="flex flex-col gap-2">
            <h3 class="font-semibold">Salles</h3>
            <ul class="flex flex-col gap-2">
              <li
                v-for="room in shown.rooms"
                :key="room.id"
                class="flex items-center gap-3"
              >
                <img
                  v-if="room.image_id"
                  :src="imageUrl(room.image_id)"
                  :alt="`Photo de ${room.name}`"
                  class="aspect-video w-14 shrink-0 rounded object-cover"
                />
                <span
                  v-else
                  class="flex aspect-video w-14 shrink-0 items-center justify-center rounded bg-muted text-dimmed"
                >
                  <UIcon name="i-lucide-door-open" />
                </span>
                <span class="flex min-w-0 flex-col">
                  <span class="flex flex-wrap items-center gap-x-2">
                    <span class="font-medium text-highlighted">
                      {{ room.name }}
                    </span>
                    <UBadge
                      v-if="isRehearsalOnly(room)"
                      label="Répétitions uniquement"
                      color="secondary"
                      variant="subtle"
                      size="sm"
                      class="rounded-full"
                    />
                  </span>
                  <span class="text-sm text-muted">
                    {{ capacities(room) || "Capacités inconnues" }}
                  </span>
                </span>
              </li>
            </ul>
          </section>

          <section class="flex flex-col gap-2">
            <h3 class="font-semibold">Contacts</h3>
            <ul v-if="shown.contacts.length" class="flex flex-col gap-2">
              <li
                v-for="(contact, i) in shown.contacts"
                :key="i"
                class="flex flex-col"
              >
                <span>
                  <span class="font-medium text-highlighted">
                    {{ contact.name }}
                  </span>
                  <span v-if="contact.role" class="text-muted">
                    · {{ contact.role }}
                  </span>
                </span>
                <span
                  v-if="contact.email || contact.phone_number"
                  class="flex flex-wrap gap-x-3 text-sm"
                >
                  <ULink
                    v-if="contact.email"
                    :to="`mailto:${contact.email}`"
                    class="text-secondary hover:text-highlighted"
                  >
                    {{ contact.email }}
                  </ULink>
                  <ULink
                    v-if="contact.phone_number"
                    :to="`tel:${contact.phone_number}`"
                    class="text-secondary hover:text-highlighted"
                  >
                    {{ contact.phone_number }}
                  </ULink>
                </span>
              </li>
            </ul>
            <p v-else class="text-muted">Aucun contact pour ce lieu.</p>
          </section>

          <section v-if="shown.notes" class="flex flex-col gap-2">
            <h3 class="font-semibold">Notes pratiques</h3>
            <p class="whitespace-pre-line text-toned">{{ shown.notes }}</p>
          </section>

          <section v-if="semester" class="flex flex-col gap-2">
            <h3 class="font-semibold">Concerts · {{ semester.short_name }}</h3>
            <ul v-if="concertsAt(shown).length" class="flex flex-col gap-0.5">
              <li v-for="concert in concertsAt(shown)" :key="concert.id">
                <ULink
                  :to="pageLink('/concerts', { concert: concert.id })"
                  class="-mx-2 flex items-center gap-2 rounded-md px-2 py-1 hover:bg-muted"
                >
                  <span class="min-w-0">
                    <span class="font-medium text-highlighted">
                      {{ concertTitle(concert) }}
                    </span>
                    <span
                      class="block text-sm text-muted first-letter:uppercase"
                    >
                      {{ formatLongDay(concert.date) }} ·
                      {{ place(concert.room_id)?.room.name }}
                    </span>
                  </span>
                  <UIcon
                    name="i-lucide-chevron-right"
                    class="ms-auto shrink-0 text-muted"
                  />
                </ULink>
              </li>
            </ul>
            <p v-else class="text-muted">Aucun concert ici ce semestre.</p>
          </section>

          <div class="flex flex-wrap gap-2 border-t border-muted pt-5">
            <UButton
              label="Modifier"
              icon="i-lucide-pencil"
              color="primary"
              variant="outline"
              :disabled="!canEdit"
              @click="dialog?.open(shown)"
            />
            <UButton
              label="Supprimer"
              icon="i-lucide-trash-2"
              color="error"
              variant="ghost"
              :disabled="!canEdit || hostsConcerts(shown)"
              @click="dialog?.remove(shown)"
            />
          </div>
          <p
            v-if="canEdit && hostsConcerts(shown)"
            class="-mt-4 text-sm text-muted"
          >
            Ce lieu accueille des concerts : déplacez-les pour pouvoir le
            supprimer.
          </p>
        </div>
      </DetailPanel>
    </div>

    <LocationDialog
      ref="locationDialog"
      @saved="(location) => (selectedId = location.id)"
      @deleted="onDeleted"
    />
  </div>
</template>
