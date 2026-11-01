<script setup lang="ts">
import type PieceDialogComponent from "~/components/PieceDialog.vue";

definePageMeta({ middleware: "orga" });
useHead({ title: "Morceaux" });

const { canEdit } = useRoles();

const route = useRoute();
const router = useRouter();

const { status: semestersStatus, pageLink } = useSemesters();
const { semester, pieces, concertsOf, loading } = useProgramme();
const { placeName } = useLocations();
const { personById } = usePeople();

// `?filter=unscheduled` comes from the dashboard.
const { filter, filters, filtered } = usePieceFilter(
  route.query.filter === "unscheduled" ? "unscheduled" : "all",
);
const search = ref("");

const rows = computed(() =>
  filtered.value
    .filter((p) =>
      matchesSearch(search.value, [
        p.name,
        p.composer,
        peopleNames(p.arranger_ids, personById.value),
        peopleNames(p.conductor_ids, personById.value),
      ]),
    )
    .map((piece) => ({ piece, concerts: concertsOf(piece.id) })),
);

const { selectedId, shown, detailOpen, rowState, onDeleted } = useListSelection(
  "piece",
  pieces,
  () => rows.value[0]?.piece,
);
const shownConcerts = computed(() =>
  shown.value ? concertsOf(shown.value.id) : [],
);
const shownCredits = computed(() =>
  shown.value
    ? [
        {
          label: "Arrangement",
          people: peopleOf(shown.value.arranger_ids, personById.value),
          empty: "Aucun",
        },
        {
          label: "Direction",
          people: peopleOf(shown.value.conductor_ids, personById.value),
          empty: "À définir",
        },
      ]
    : [],
);

const dialog =
  useTemplateRef<InstanceType<typeof PieceDialogComponent>>("pieceDialog");

// `?edit=new` (from the dashboard) opens the creation dialog. Client side
// only: the dialog state is not transferred from the server.
onMounted(() => {
  if (route.query.edit !== "new") return;
  if (canEdit.value) dialog.value?.open();
  router.replace({ query: { ...route.query, edit: undefined } });
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
          :disabled="!canEdit"
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
      <ListCard
        v-else
        header-class="@xl:grid @xl:grid-cols-[minmax(0,3fr)_minmax(0,2fr)]"
        :empty="
          rows.length
            ? undefined
            : pieces.length
              ? 'Aucun morceau ne correspond'
              : 'Aucun morceau pour ce semestre'
        "
      >
        <template #header>
          <span>Morceau</span>
          <span>Concerts</span>
        </template>
        <ListRow
          v-for="{ piece, concerts: pieceConcerts } in rows"
          :key="piece.id"
          :state="rowState(piece.id)"
          class="grid gap-2 @xl:grid-cols-[minmax(0,3fr)_minmax(0,2fr)] @xl:items-center @xl:gap-4"
          @click="selectedId = piece.id"
        >
          <span class="flex min-w-0 flex-col gap-0.5">
            <span
              class="font-display text-lg leading-snug font-semibold text-highlighted"
            >
              {{ piece.name }}
            </span>
            <span
              v-if="pieceCredits(piece, personById)"
              class="text-sm text-muted"
            >
              {{ pieceCredits(piece, personById) }}
            </span>
          </span>
          <span class="flex flex-wrap gap-1.5">
            <ConcertChip
              v-for="concert in pieceConcerts"
              :key="concert.id"
              :concert="concert"
            />
            <span v-if="!pieceConcerts.length" class="text-sm text-dimmed">
              Hors programme
            </span>
          </span>
        </ListRow>
      </ListCard>

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
          </div>

          <dl class="flex flex-col gap-1.5">
            <div
              v-for="credit in shownCredits"
              :key="credit.label"
              class="flex gap-3"
            >
              <dt class="w-28 shrink-0 font-semibold">{{ credit.label }}</dt>
              <dd v-if="credit.people.length" class="min-w-0">
                <template
                  v-for="(person, index) in credit.people"
                  :key="person.id"
                >
                  <template v-if="index">, </template>
                  <ULink
                    :to="pageLink('/people', { person: person.id })"
                    class="text-highlighted hover:text-secondary"
                  >
                    {{ personName(person) }}</ULink
                  >
                </template>
              </dd>
              <dd v-else class="text-dimmed">{{ credit.empty }}</dd>
            </div>
          </dl>

          <section class="flex flex-col gap-2">
            <h3 class="font-semibold">Au programme de</h3>
            <ul v-if="shownConcerts.length" class="flex flex-col gap-1">
              <li v-for="concert in shownConcerts" :key="concert.id">
                <ULink
                  :to="pageLink('/concerts', { concert: concert.id })"
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
                :to="pageLink('/concerts')"
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
              :disabled="!canEdit"
              @click="dialog?.open(shown)"
            />
            <UButton
              label="Supprimer"
              icon="i-lucide-trash-2"
              color="error"
              variant="ghost"
              :disabled="!canEdit"
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
