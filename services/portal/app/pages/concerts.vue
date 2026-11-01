<script setup lang="ts">
import type { ConcertWithPieces, Piece } from "#hey-api/types.gen";
import type ConcertDialogComponent from "~/components/ConcertDialog.vue";
import type ConcertProgrammeDialogComponent from "~/components/ConcertProgrammeDialog.vue";

// Reserved to the Orga group; others land on their profile.
const user = await currentUser();
if (!user.value.groups.includes("Orga")) {
  await navigateTo("/profile", { replace: true });
}

const route = useRoute();
const router = useRouter();

const { status: semestersStatus } = useSemesters();
const { semester, concerts, pieces, pieceById, loading } = useProgramme();
const { place } = useLocations();

type Filter = "all" | "upcoming" | "past";
const filter = ref<Filter>("all");
const search = ref("");

const isUpcoming = (concert: ConcertWithPieces) => daysUntil(concert.date) >= 0;
const upcoming = computed(() => concerts.value.filter(isUpcoming));

const filters = computed(() => [
  { value: "all" as const, label: `Tous (${concerts.value.length})` },
  { value: "upcoming" as const, label: `À venir (${upcoming.value.length})` },
  {
    value: "past" as const,
    label: `Passés (${concerts.value.length - upcoming.value.length})`,
  },
]);

/** Lowercase, without accents. */
const normalize = (value: string) =>
  value
    .normalize("NFD")
    .replace(/\p{Diacritic}/gu, "")
    .toLowerCase();

const rows = computed(() => {
  const query = normalize(search.value.trim());
  return concerts.value
    .filter((c) =>
      filter.value === "all"
        ? true
        : (filter.value === "upcoming") === isUpcoming(c),
    )
    .filter((c) => {
      if (!query) return true;
      const p = place(c.room_id);
      return [c.name, p?.location.name, p?.room.name, p?.location.city].some(
        (field) => field && normalize(field).includes(query),
      );
    });
});

const selectedId = useSelectedId("concert");
const selected = computed(() =>
  concerts.value.find((c) => c.id === selectedId.value),
);
/**
 * On large screens, the next concert (or the last one, once they are all
 * past) is shown until another is selected.
 */
const shown = computed(
  () => selected.value ?? rows.value.find(isUpcoming) ?? rows.value.at(-1),
);
/** The default item is only highlighted next to the large screen panel. */
const rowClass = (id: number) =>
  selected.value?.id === id
    ? "border-secondary bg-secondary/10 hover:bg-secondary/15"
    : shown.value?.id === id
      ? "border-transparent lg:border-secondary lg:bg-secondary/10 lg:hover:bg-secondary/15"
      : "border-transparent";
const detailOpen = computed({
  get: () => !!selected.value,
  set: (open) => {
    if (!open) selectedId.value = undefined;
  },
});

const programme = (concert: ConcertWithPieces) =>
  concert.piece_ids
    .map((id) => pieceById.value.get(id))
    .filter((p): p is Piece => !!p);

/** « Dimanche 13 décembre 2026 » */
const fullDate = (value: string) =>
  parseDay(value).toLocaleDateString("fr", {
    weekday: "long",
    day: "numeric",
    month: "long",
    year: "numeric",
  });

const schedule = (concert: ConcertWithPieces) => [
  {
    label: "Rendez-vous des musiciens",
    icon: "i-lucide-users",
    time: concert.meeting_at,
  },
  {
    label: "Ouverture des portes",
    icon: "i-lucide-door-open",
    time: concert.doors_open_at,
  },
  { label: "Début", icon: "i-lucide-music", time: concert.starts_at },
];

const concertDialog =
  useTemplateRef<InstanceType<typeof ConcertDialogComponent>>("concertDialog");
const programmeDialog =
  useTemplateRef<InstanceType<typeof ConcertProgrammeDialogComponent>>(
    "programmeDialog",
  );

const onDeleted = (id: number) => {
  if (selectedId.value === id) selectedId.value = undefined;
};

// Links from the dashboard open a dialog: `?edit=new` to create a concert,
// `?edit=concert` / `?edit=programme` for the selected concert. Client side
// only: the dialog state is not transferred from the server.
onMounted(() =>
  watch(
    [() => route.query.edit, concerts],
    ([edit]) => {
      if (!edit) return;
      if (edit === "new") concertDialog.value?.open();
      else if (!selected.value) return;
      else if (edit === "concert") concertDialog.value?.open(selected.value);
      else if (edit === "programme")
        programmeDialog.value?.open(selected.value);
      router.replace({ query: { ...route.query, edit: undefined } });
    },
    { immediate: true },
  ),
);

const pieceLink = (pieceId: number) => ({
  path: "/pieces",
  query: { semester: semester.value?.id, piece: pieceId },
});
</script>

<template>
  <div v-if="semester" class="flex flex-col gap-6">
    <PageHeader
      :overline="`Semestre ${semester.short_name}`"
      title="Concerts"
      description="Les dates du semestre, leur lieu et leur programme."
    >
      <template #actions>
        <UButton
          label="Ajouter un concert"
          icon="i-lucide-plus"
          size="lg"
          @click="concertDialog?.open()"
        />
      </template>
    </PageHeader>

    <div class="flex flex-wrap items-center gap-3">
      <UInput
        v-model="search"
        icon="i-lucide-search"
        placeholder="Rechercher un concert ou un lieu"
        size="lg"
        class="min-w-64 flex-1"
        :ui="{ base: 'bg-default' }"
      />
      <FilterChips
        v-if="concerts.length"
        v-model="filter"
        :items="filters"
        label="Filtrer les concerts"
      />
    </div>

    <div
      class="grid items-start gap-6 lg:grid-cols-[minmax(0,1fr)_minmax(0,22rem)] xl:grid-cols-[minmax(0,1fr)_minmax(0,26rem)]"
    >
      <USkeleton v-if="loading" class="h-80 rounded-lg" />
      <div
        v-else
        class="@container overflow-hidden rounded-lg border border-default bg-default"
      >
        <div
          class="hidden gap-4 border-b border-default py-3 ps-[calc(1.25rem+3px+4.625rem)] pe-5 text-xs font-semibold tracking-wider text-muted uppercase @2xl:grid @2xl:grid-cols-[minmax(0,5fr)_minmax(0,4fr)_minmax(0,2fr)]"
        >
          <span>Concert</span>
          <span>Lieu</span>
          <span>Programme</span>
        </div>
        <ul>
          <li
            v-for="concert in rows"
            :key="concert.id"
            class="border-b border-muted last:border-b-0"
          >
            <button
              type="button"
              class="flex w-full items-center gap-3.5 border-s-3 px-5 py-3.5 text-start transition-colors hover:bg-muted focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-primary"
              :class="rowClass(concert.id)"
              :aria-current="shown?.id === concert.id ? 'true' : undefined"
              @click="selectedId = concert.id"
            >
              <DateTile
                :date="concert.date"
                :color="isUpcoming(concert) ? 'tertiary' : 'secondary'"
              />
              <!-- Stacked in a narrow list, in columns in a wide one. -->
              <span
                class="grid min-w-0 flex-1 gap-1 @2xl:grid-cols-[minmax(0,5fr)_minmax(0,4fr)_minmax(0,2fr)] @2xl:items-center @2xl:gap-4"
              >
                <span class="flex min-w-0 flex-col">
                  <span
                    class="font-display text-lg leading-snug font-semibold"
                    :class="
                      isUpcoming(concert) ? 'text-highlighted' : 'text-toned'
                    "
                  >
                    {{ concertTitle(concert) }}
                  </span>
                  <span class="text-sm text-muted">
                    {{
                      concert.starts_at
                        ? formatTime(concert.starts_at)
                        : "Horaire à définir"
                    }}
                    · {{ countdown(concert.date) ?? "Passé" }}
                  </span>
                </span>
                <span class="flex min-w-0 flex-col text-sm">
                  <span v-if="place(concert.room_id)">
                    <span class="font-medium text-highlighted">
                      {{ place(concert.room_id)?.location.name }}
                    </span>
                    <span class="text-muted">
                      · {{ place(concert.room_id)?.room.name }}
                    </span>
                  </span>
                  <span v-else class="text-dimmed">Salle à définir</span>
                  <span
                    v-if="place(concert.room_id) && !concert.room_booked"
                    class="flex items-center gap-1 font-medium text-warning"
                  >
                    <UIcon name="i-lucide-clock" class="shrink-0" />
                    Réservation à confirmer
                  </span>
                </span>
                <span
                  class="text-sm"
                  :class="
                    concert.piece_ids.length ? 'text-toned' : 'text-dimmed'
                  "
                >
                  {{
                    concert.piece_ids.length
                      ? piecesCount(concert.piece_ids.length)
                      : "Programme vide"
                  }}
                </span>
              </span>
            </button>
          </li>
          <li v-if="!rows.length" class="px-5 py-8 text-center text-muted">
            {{
              concerts.length
                ? "Aucun concert ne correspond"
                : "Aucun concert pour ce semestre"
            }}
          </li>
        </ul>
      </div>

      <DetailPanel
        v-if="shown"
        v-model:open="detailOpen"
        title="Détails du concert"
      >
        <div class="flex flex-col gap-6">
          <div class="flex flex-col gap-1">
            <span
              class="text-sm font-semibold tracking-wider text-muted uppercase"
            >
              {{ fullDate(shown.date) }}
            </span>
            <h2
              class="font-display text-2xl leading-tight font-bold text-highlighted xl:text-3xl"
            >
              {{ concertTitle(shown) }}
            </h2>
            <span class="text-muted">
              {{ countdown(shown.date) ?? "Passé" }}
            </span>
          </div>

          <section class="flex flex-col gap-2">
            <h3 class="font-semibold">Lieu</h3>
            <div v-if="place(shown.room_id)" class="flex flex-col">
              <span class="text-highlighted">
                {{ place(shown.room_id)?.location.name }} ·
                {{ place(shown.room_id)?.room.name }}
              </span>
              <span class="text-sm text-muted">
                {{ place(shown.room_id)?.location.street }},
                {{ place(shown.room_id)?.location.zipcode }}
                {{ place(shown.room_id)?.location.city }}
              </span>
            </div>
            <p v-else class="text-muted">Salle à définir</p>
            <UBadge
              v-if="place(shown.room_id) && !shown.room_booked"
              label="Réservation à confirmer"
              icon="i-lucide-clock"
              color="warning"
              variant="subtle"
              class="self-start rounded-full"
            />
          </section>

          <section class="flex flex-col gap-2">
            <h3 class="font-semibold">Horaires</h3>
            <dl class="flex flex-col gap-1">
              <div
                v-for="item in schedule(shown)"
                :key="item.label"
                class="flex items-center gap-2"
              >
                <UIcon :name="item.icon" class="shrink-0 text-muted" />
                <dt class="text-toned">{{ item.label }}</dt>
                <dd
                  class="ms-auto"
                  :class="item.time ? 'font-medium' : 'text-dimmed'"
                >
                  {{ item.time ? formatTime(item.time) : "À définir" }}
                </dd>
              </div>
            </dl>
          </section>

          <section class="flex flex-col gap-2">
            <div class="flex items-center justify-between gap-2">
              <h3 class="font-semibold">Programme</h3>
              <UButton
                label="Modifier"
                icon="i-lucide-list-music"
                color="neutral"
                variant="ghost"
                size="sm"
                @click="programmeDialog?.open(shown)"
              />
            </div>
            <ol v-if="programme(shown).length" class="flex flex-col gap-0.5">
              <li v-for="(piece, index) in programme(shown)" :key="piece.id">
                <ULink
                  :to="pieceLink(piece.id)"
                  class="-mx-2 flex items-baseline gap-2.5 rounded-md px-2 py-1 hover:bg-muted"
                >
                  <span class="w-4 shrink-0 text-end text-sm text-muted">
                    {{ index + 1 }}
                  </span>
                  <span class="min-w-0">
                    <span class="font-medium text-highlighted">
                      {{ piece.name }}
                    </span>
                    <span v-if="piece.composer" class="text-sm text-muted">
                      · {{ piece.composer }}
                    </span>
                  </span>
                </ULink>
              </li>
            </ol>
            <p v-else class="text-muted">
              Programme vide.
              <template v-if="!pieces.length">
                Ajoutez d’abord les morceaux du semestre depuis la page
                <ULink
                  :to="{ path: '/pieces', query: { semester: semester.id } }"
                  class="font-medium text-secondary hover:text-highlighted"
                >
                  Morceaux</ULink
                >.
              </template>
            </p>
          </section>

          <section class="flex flex-col gap-2">
            <h3 class="font-semibold">Diffusion</h3>
            <div class="flex flex-wrap items-center gap-2">
              <UBadge
                :label="shown.public ? 'Public' : 'Privé'"
                :icon="shown.public ? 'i-lucide-globe' : 'i-lucide-lock'"
                :color="shown.public ? 'success' : 'neutral'"
                variant="subtle"
                class="rounded-full"
              />
              <UButton
                v-if="shown.tickets_url"
                :to="shown.tickets_url"
                target="_blank"
                label="Billetterie"
                icon="i-lucide-ticket"
                color="neutral"
                variant="link"
                size="sm"
              />
              <UButton
                v-if="shown.video_url"
                :to="shown.video_url"
                target="_blank"
                label="Vidéo"
                icon="i-lucide-circle-play"
                color="neutral"
                variant="link"
                size="sm"
              />
            </div>
          </section>

          <div class="flex flex-wrap gap-2 border-t border-muted pt-5">
            <UButton
              label="Modifier"
              icon="i-lucide-pencil"
              color="primary"
              variant="outline"
              @click="concertDialog?.open(shown)"
            />
            <UButton
              label="Supprimer"
              icon="i-lucide-trash-2"
              color="error"
              variant="ghost"
              @click="concertDialog?.remove(shown)"
            />
          </div>
        </div>
      </DetailPanel>
    </div>

    <ConcertDialog
      ref="concertDialog"
      :semester="semester"
      @saved="(id) => (selectedId = id)"
      @deleted="onDeleted"
    />
    <ConcertProgrammeDialog ref="programmeDialog" :pieces="pieces" />
  </div>

  <UEmpty
    v-else-if="semestersStatus !== 'pending'"
    icon="i-lucide-calendar"
    title="Aucun semestre"
    description="Créez un semestre depuis le menu Semestre pour y ajouter des concerts."
  />
</template>
