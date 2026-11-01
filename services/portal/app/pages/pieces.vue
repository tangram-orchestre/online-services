<script setup lang="ts">
import type { Piece } from "#hey-api/types.gen";
import type PieceDialogComponent from "~/components/PieceDialog.vue";

// Reserved to the Orga group; others land on their profile.
const user = await currentUser();
if (!user.value.groups.includes("Orga")) {
  await navigateTo("/profile", { replace: true });
}

const route = useRoute();
const router = useRouter();

const { status: semestersStatus } = useSemesters();
const { semester, concerts, pieces, concertsOf, loading } = useProgramme();
const { placeName } = useLocations();

/** A concert id, or the special filters. */
type Filter = number | "all" | "unscheduled";
// `?filter=unscheduled` comes from the dashboard.
const filter = ref<Filter>(
  route.query.filter === "unscheduled" ? "unscheduled" : "all",
);
const search = ref("");

const unscheduled = computed(() =>
  pieces.value.filter((p) => !concertsOf(p.id).length),
);

const filters = computed(() => [
  { value: "all" as Filter, label: `Tous (${pieces.value.length})` },
  ...concerts.value.map((c) => ({
    value: c.id as Filter,
    label: concertShortName(c),
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

// Back to all pieces when the filtered concert disappears (e.g. semester
// change), once loaded.
watch(filters, (items) => {
  if (!loading.value && !items.some((f) => f.value === filter.value)) {
    filter.value = "all";
  }
});

/** Lowercase, without accents: « Dvořák » matches « dvorak ». */
const normalize = (value: string) =>
  value
    .normalize("NFD")
    .replace(/\p{Diacritic}/gu, "")
    .toLowerCase();

const rows = computed(() => {
  const f = filter.value;
  const query = normalize(search.value.trim());
  return pieces.value
    .filter((p) =>
      f === "all"
        ? true
        : f === "unscheduled"
          ? !concertsOf(p.id).length
          : concertsOf(p.id).some((c) => c.id === f),
    )
    .filter(
      (p) =>
        !query ||
        [p.name, p.composer, p.arranger].some(
          (field) => field && normalize(field).includes(query),
        ),
    )
    .map((piece) => ({ piece, concerts: concertsOf(piece.id) }));
});

const selectedId = useSelectedId("piece");
const selected = computed(() =>
  pieces.value.find((p) => p.id === selectedId.value),
);
/** On large screens, the first piece is shown until another is selected. */
const shown = computed(() => selected.value ?? rows.value[0]?.piece);
const shownConcerts = computed(() =>
  shown.value ? concertsOf(shown.value.id) : [],
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

/** « Beethoven · arr. Jean Dupont » */
const credits = (piece: Piece) =>
  [piece.composer, piece.arranger && `arr. ${piece.arranger}`]
    .filter(Boolean)
    .join(" · ");

const dialog =
  useTemplateRef<InstanceType<typeof PieceDialogComponent>>("pieceDialog");

const onDeleted = (id: number) => {
  if (selectedId.value === id) selectedId.value = undefined;
};

// `?edit=new` (from the dashboard) opens the creation dialog. Client side
// only: the dialog state is not transferred from the server.
onMounted(() => {
  if (route.query.edit !== "new") return;
  dialog.value?.open();
  router.replace({ query: { ...route.query, edit: undefined } });
});

const concertLink = (concertId: number) => ({
  path: "/concerts",
  query: { semester: semester.value?.id, concert: concertId },
});
</script>

<template>
  <div v-if="semester" class="flex flex-col gap-6">
    <PageHeader
      :overline="`Semestre ${semester.short_name}`"
      title="Morceaux"
      description="Les morceaux travaillés ce semestre et les concerts où ils sont joués."
    >
      <template #actions>
        <UButton
          label="Ajouter un morceau"
          icon="i-lucide-plus"
          size="lg"
          @click="dialog?.open()"
        />
      </template>
    </PageHeader>

    <div class="flex flex-wrap items-center gap-3">
      <UInput
        v-model="search"
        icon="i-lucide-search"
        placeholder="Rechercher un morceau ou un compositeur"
        size="lg"
        class="min-w-64 flex-1"
        :ui="{ base: 'bg-default' }"
      />
      <FilterChips
        v-if="pieces.length"
        v-model="filter"
        :items="filters"
        label="Filtrer par concert"
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
          class="hidden gap-4 border-b border-default px-5 py-3 text-xs font-semibold tracking-wider text-muted uppercase @xl:grid @xl:grid-cols-[minmax(0,3fr)_minmax(0,2fr)]"
        >
          <span>Morceau</span>
          <span>Concerts</span>
        </div>
        <ul>
          <li
            v-for="{ piece, concerts: pieceConcerts } in rows"
            :key="piece.id"
            class="border-b border-muted last:border-b-0"
          >
            <button
              type="button"
              class="grid w-full gap-2 border-s-3 px-5 py-3.5 text-start transition-colors hover:bg-muted focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-primary @xl:grid-cols-[minmax(0,3fr)_minmax(0,2fr)] @xl:items-center @xl:gap-4"
              :class="rowClass(piece.id)"
              :aria-current="shown?.id === piece.id ? 'true' : undefined"
              @click="selectedId = piece.id"
            >
              <span class="flex min-w-0 flex-col gap-0.5">
                <span
                  class="font-display text-lg leading-snug font-semibold text-highlighted"
                >
                  {{ piece.name }}
                </span>
                <span v-if="credits(piece)" class="text-sm text-muted">
                  {{ credits(piece) }}
                </span>
              </span>
              <span class="flex flex-wrap gap-1.5">
                <span
                  v-for="concert in pieceConcerts"
                  :key="concert.id"
                  class="rounded-full bg-tertiary-200 px-2.5 py-0.5 text-sm font-medium text-on-tertiary"
                >
                  {{ concertShortName(concert) }}
                </span>
                <span v-if="!pieceConcerts.length" class="text-sm text-dimmed">
                  Hors programme
                </span>
              </span>
            </button>
          </li>
          <li v-if="!rows.length" class="px-5 py-8 text-center text-muted">
            {{
              pieces.length
                ? "Aucun morceau ne correspond"
                : "Aucun morceau pour ce semestre"
            }}
          </li>
        </ul>
      </div>

      <DetailPanel
        v-if="shown"
        v-model:open="detailOpen"
        title="Détails du morceau"
      >
        <div class="flex flex-col gap-6">
          <div class="flex flex-col gap-1">
            <span
              v-if="shown.composer"
              class="text-sm font-semibold tracking-wider text-muted uppercase"
            >
              {{ shown.composer }}
            </span>
            <h2
              class="font-display text-2xl leading-tight font-bold text-highlighted xl:text-3xl"
            >
              {{ shown.name }}
            </h2>
            <span v-if="shown.arranger" class="text-muted">
              Arrangement : {{ shown.arranger }}
            </span>
          </div>

          <section class="flex flex-col gap-2">
            <h3 class="font-semibold">Au programme de</h3>
            <ul v-if="shownConcerts.length" class="flex flex-col gap-1">
              <li v-for="concert in shownConcerts" :key="concert.id">
                <ULink
                  :to="concertLink(concert.id)"
                  class="-mx-2 flex items-center gap-3 rounded-md px-2 py-1.5 hover:bg-muted"
                >
                  <span class="flex min-w-0 flex-col">
                    <span class="font-medium text-highlighted">
                      {{ concertTitle(concert) }}
                    </span>
                    <span class="text-sm text-muted first-letter:uppercase">
                      <template v-if="concert.name">
                        {{ formatLongDay(concert.date) }} ·
                      </template>
                      {{ placeName(concert.room_id) ?? "Salle à définir" }}
                    </span>
                  </span>
                  <UIcon
                    name="i-lucide-chevron-right"
                    class="ms-auto shrink-0 text-muted"
                  />
                </ULink>
              </li>
            </ul>
            <p v-else class="text-muted">
              Au programme d’aucun concert pour l’instant. Les programmes se
              construisent depuis la page
              <ULink
                :to="{ path: '/concerts', query: { semester: semester.id } }"
                class="font-medium text-secondary hover:text-highlighted"
              >
                Concerts</ULink
              >.
            </p>
          </section>

          <section v-if="shown.source" class="flex flex-col gap-1">
            <h3 class="font-semibold">Source</h3>
            <p class="text-muted">{{ shown.source }}</p>
          </section>

          <div class="flex flex-wrap gap-2 border-t border-muted pt-5">
            <UButton
              label="Modifier"
              icon="i-lucide-pencil"
              color="primary"
              variant="outline"
              @click="dialog?.open(shown)"
            />
            <UButton
              label="Supprimer"
              icon="i-lucide-trash-2"
              color="error"
              variant="ghost"
              @click="dialog?.remove(shown)"
            />
          </div>
        </div>
      </DetailPanel>
    </div>

    <PieceDialog
      ref="pieceDialog"
      :semester="semester"
      @saved="(id) => (selectedId = id)"
      @deleted="onDeleted"
    />
  </div>

  <UEmpty
    v-else-if="semestersStatus !== 'pending'"
    icon="i-lucide-calendar"
    title="Aucun semestre"
    description="Créez un semestre depuis le menu Semestre pour y ajouter des morceaux."
  />
</template>
