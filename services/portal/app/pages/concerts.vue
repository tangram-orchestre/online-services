<script setup lang="ts">
import type { ConcertWithPieces, PieceWithPeople } from "#hey-api/types.gen";
import type ConcertDialogComponent from "~/components/ConcertDialog.vue";
import type ConcertProgrammeDialogComponent from "~/components/ConcertProgrammeDialog.vue";
import type DefaultImageDialogComponent from "~/components/DefaultImageDialog.vue";

definePageMeta({ middleware: "orga" });

const { canEdit } = useRoles();

const route = useRoute();
const router = useRouter();

const { status: semestersStatus, pageLink } = useSemesters();
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

const rows = computed(() =>
  concerts.value
    .filter((c) =>
      filter.value === "all"
        ? true
        : (filter.value === "upcoming") === isUpcoming(c),
    )
    .filter((c) => {
      const p = place(c.room_id);
      return matchesSearch(search.value, [
        c.name,
        p?.location.name,
        p?.room.name,
        p?.location.city,
      ]);
    }),
);

// On large screens, the next concert (or the last one, once they are all
// past) is shown until another is selected.
const { selectedId, selected, shown, detailOpen, rowState, onDeleted } =
  useListSelection(
    "concert",
    concerts,
    () => rows.value.find(isUpcoming) ?? rows.value.at(-1),
  );

const programme = (concert: ConcertWithPieces) =>
  concert.piece_ids
    .map((id) => pieceById.value.get(id))
    .filter((p): p is PieceWithPeople => !!p);

/** « Dimanche 13 décembre 2026 » */
const fullDate = (value: string) =>
  parseDay(value).toLocaleDateString("fr", {
    weekday: "long",
    day: "numeric",
    month: "long",
    year: "numeric",
  });

const { settings } = useSettings();

/**
 * Image of the concert on the showcase website, and where it comes from: its
 * own, its room's, its venue's, else the default one.
 */
const concertImage = (concert: ConcertWithPieces) => {
  const p = place(concert.room_id);
  if (concert.image_id)
    return { id: concert.image_id, source: "Image du concert" };
  if (p?.room.image_id)
    return {
      id: p.room.image_id,
      source: `Photo de la salle « ${p.room.name} »`,
    };
  if (p?.location.image_id)
    return {
      id: p.location.image_id,
      source: `Photo du lieu « ${p.location.name} »`,
    };
  const id = settings.value?.default_concert_image_id;
  return id ? { id, source: "Image par défaut", isDefault: true } : undefined;
};

const shownImage = computed(() => shown.value && concertImage(shown.value));

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
const defaultImageDialog =
  useTemplateRef<InstanceType<typeof DefaultImageDialogComponent>>(
    "defaultImageDialog",
  );
const programmeDialog =
  useTemplateRef<InstanceType<typeof ConcertProgrammeDialogComponent>>(
    "programmeDialog",
  );

// Links from the dashboard open a dialog: `?edit=new` to create a concert,
// `?edit=concert` / `?edit=programme` for the selected concert. Client side
// only: the dialog state is not transferred from the server.
onMounted(() =>
  watch(
    [() => route.query.edit, concerts],
    ([edit]) => {
      if (!edit) return;
      // Read-only users only get the concert selected.
      if (canEdit.value) {
        if (edit === "new") concertDialog.value?.open();
        else if (!selected.value) return;
        else if (edit === "concert") concertDialog.value?.open(selected.value);
        else if (edit === "programme")
          programmeDialog.value?.open(selected.value);
      }
      router.replace({ query: { ...route.query, edit: undefined } });
    },
    { immediate: true },
  ),
);
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
          :disabled="!canEdit"
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
      <ListCard
        v-else
        header-class="ps-[calc(1.25rem+3px+4.625rem)]! @2xl:grid @2xl:grid-cols-[minmax(0,5fr)_minmax(0,4fr)_minmax(0,2fr)]"
        :empty="
          rows.length
            ? undefined
            : concerts.length
              ? 'Aucun concert ne correspond'
              : 'Aucun concert pour ce semestre'
        "
      >
        <template #header>
          <span>Concert</span>
          <span>Lieu</span>
          <span>Programme</span>
        </template>
        <ListRow
          v-for="concert in rows"
          :key="concert.id"
          :state="rowState(concert.id)"
          class="flex items-center gap-3.5"
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
                :class="isUpcoming(concert) ? 'text-highlighted' : 'text-toned'"
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
              :class="concert.piece_ids.length ? 'text-toned' : 'text-dimmed'"
            >
              {{
                concert.piece_ids.length
                  ? piecesCount(concert.piece_ids.length)
                  : "Programme vide"
              }}
            </span>
          </span>
        </ListRow>
      </ListCard>

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
          <!-- As on the showcase website. -->
          <section v-if="shownImage" class="flex flex-col gap-2">
            <h3 class="font-semibold">Image sur le site</h3>
            <div class="flex items-center gap-4">
              <a
                :href="imageUrl(shownImage.id)"
                target="_blank"
                class="shrink-0"
              >
                <img
                  :src="imageUrl(shownImage.id)"
                  :alt="shownImage.source"
                  class="aspect-video w-28 rounded-md object-cover"
                />
              </a>
              <div class="flex min-w-0 flex-col gap-0.5">
                <span
                  class="text-sm"
                  :class="
                    shownImage.isDefault
                      ? 'font-medium text-warning'
                      : 'text-toned'
                  "
                >
                  {{ shownImage.source }}
                </span>
                <span v-if="shownImage.isDefault" class="text-xs text-muted">
                  Ajoutez une image au concert, à sa salle ou à son lieu.
                </span>
                <span class="flex flex-wrap gap-x-3">
                  <UButton
                    label="Changer"
                    color="neutral"
                    variant="link"
                    size="xs"
                    class="px-0"
                    :disabled="!canEdit"
                    @click="concertDialog?.open(shown)"
                  />
                  <UButton
                    v-if="shownImage.isDefault"
                    label="Image par défaut…"
                    color="neutral"
                    variant="link"
                    size="xs"
                    class="px-0"
                    :disabled="!canEdit"
                    @click="defaultImageDialog?.open()"
                  />
                </span>
              </div>
            </div>
          </section>

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
                :disabled="!canEdit"
                @click="programmeDialog?.open(shown)"
              />
            </div>
            <ol v-if="programme(shown).length" class="flex flex-col gap-0.5">
              <li v-for="(piece, index) in programme(shown)" :key="piece.id">
                <ULink
                  :to="pageLink('/pieces', { piece: piece.id })"
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
                  :to="pageLink('/pieces')"
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
              <ConcertPublicity :concert="shown" />
            </div>
          </section>

          <div class="flex flex-wrap gap-2 border-t border-muted pt-5">
            <UButton
              label="Modifier"
              icon="i-lucide-pencil"
              color="primary"
              variant="outline"
              :disabled="!canEdit"
              @click="concertDialog?.open(shown)"
            />
            <UButton
              label="Supprimer"
              icon="i-lucide-trash-2"
              color="error"
              variant="ghost"
              :disabled="!canEdit"
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
    <DefaultImageDialog ref="defaultImageDialog" />
  </div>

  <UEmpty
    v-else-if="semestersStatus !== 'pending'"
    icon="i-lucide-calendar"
    title="Aucun semestre"
    description="Créez un semestre depuis le menu Semestre pour y ajouter des concerts."
  />
</template>
