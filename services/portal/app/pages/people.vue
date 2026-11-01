<script setup lang="ts">
import type { Person, PieceWithPeople } from "#hey-api/types.gen";
import type PersonDialogComponent from "~/components/PersonDialog.vue";

definePageMeta({ middleware: "orga" });

const { canEdit } = useRoles();

const route = useRoute();
const router = useRouter();

const { pageLink } = useSemesters();
const { people, status } = usePeople();
const { semester, pieces } = useProgramme();

const search = ref("");

const rows = computed(() =>
  people.value.filter((p) =>
    matchesSearch(search.value, [p.first_name, p.last_name, p.display_name]),
  ),
);

/** Pieces of the selected semester the person arranges and conducts. */
const creditsOf = (person: Person) => {
  const arranged: PieceWithPeople[] = [];
  const conducted: PieceWithPeople[] = [];
  for (const piece of pieces.value) {
    if (piece.arranger_ids.includes(person.id)) arranged.push(piece);
    if (piece.conductor_ids.includes(person.id)) conducted.push(piece);
  }
  return { arranged, conducted };
};

/** « Arrangement · Direction » */
const rolesSummary = (person: Person) => {
  const { arranged, conducted } = creditsOf(person);
  return [arranged.length && "Arrangement", conducted.length && "Direction"]
    .filter(Boolean)
    .join(" · ");
};

const { selectedId, shown, detailOpen, rowState, onDeleted } = useListSelection(
  "person",
  people,
  () => rows.value[0],
);

const shownCredits = computed(() => {
  if (!shown.value) return [];
  const { arranged, conducted } = creditsOf(shown.value);
  return [
    { label: "Arrangement", pieces: arranged },
    { label: "Direction", pieces: conducted },
  ].filter((credit) => credit.pieces.length);
});

const dialog =
  useTemplateRef<InstanceType<typeof PersonDialogComponent>>("personDialog");

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
      title="Personnes"
      description="Les membres de l’orchestre crédités sur les morceaux, avec ou sans compte."
    >
      <template #actions>
        <UButton
          label="Ajouter une personne"
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
      placeholder="Rechercher une personne"
      size="lg"
      class="w-full"
      :ui="{ base: 'bg-default' }"
    />

    <div
      class="grid items-start gap-6 lg:grid-cols-[minmax(0,1fr)_minmax(0,22rem)] xl:grid-cols-[minmax(0,1fr)_minmax(0,26rem)]"
    >
      <USkeleton v-if="status === 'pending'" class="h-80 rounded-lg" />
      <ListCard
        v-else
        header-class="@xl:grid @xl:grid-cols-[minmax(0,3fr)_minmax(0,2fr)]"
        :empty="
          rows.length
            ? undefined
            : people.length
              ? 'Aucune personne ne correspond'
              : 'Aucune personne pour l’instant'
        "
      >
        <template #header>
          <span>Personne</span>
          <span v-if="semester">Semestre {{ semester.short_name }}</span>
        </template>
        <ListRow
          v-for="person in rows"
          :key="person.id"
          :state="rowState(person.id)"
          class="grid gap-1 @xl:grid-cols-[minmax(0,3fr)_minmax(0,2fr)] @xl:items-center @xl:gap-4"
          @click="selectedId = person.id"
        >
          <span class="flex min-w-0 flex-col gap-0.5">
            <span
              class="font-display text-lg leading-snug font-semibold text-highlighted"
            >
              {{ personName(person) }}
            </span>
            <span class="text-sm text-muted">
              <template v-if="publicName(person)">
                Nom public : {{ publicName(person) }}
              </template>
              <template v-else>Anonyme</template>
            </span>
          </span>
          <span
            v-if="semester"
            class="text-sm"
            :class="rolesSummary(person) ? 'text-toned' : 'text-dimmed'"
          >
            {{ rolesSummary(person) || "Aucun morceau" }}
          </span>
        </ListRow>
      </ListCard>

      <DetailPanel
        v-if="shown"
        v-model:open="detailOpen"
        title="Détails de la personne"
      >
        <div class="flex flex-col gap-6">
          <div class="flex flex-col gap-1">
            <h2
              class="font-display text-2xl leading-tight font-bold text-highlighted xl:text-3xl"
            >
              {{ personName(shown) }}
            </h2>
            <UBadge
              v-if="shown.anonymous"
              label="Anonyme"
              icon="i-lucide-eye-off"
              color="neutral"
              variant="subtle"
              class="self-start rounded-full"
            />
          </div>

          <section class="flex flex-col gap-1">
            <h3 class="font-semibold">Nom public</h3>
            <p v-if="publicName(shown)" class="text-toned">
              {{ publicName(shown) }}
              <span v-if="shown.display_name" class="text-muted">(alias)</span>
            </p>
            <p v-else class="text-muted">
              Anonyme : son nom n’apparaît sur aucun support public de Tangram.
            </p>
          </section>

          <section v-if="semester" class="flex flex-col gap-3">
            <h3 class="font-semibold">Semestre {{ semester.short_name }}</h3>
            <div
              v-for="credit in shownCredits"
              :key="credit.label"
              class="flex flex-col gap-1"
            >
              <span
                class="text-xs font-semibold tracking-wider text-muted uppercase"
              >
                {{ credit.label }}
              </span>
              <ul class="flex flex-col gap-0.5">
                <li v-for="piece in credit.pieces" :key="piece.id">
                  <ULink
                    :to="pageLink('/pieces', { piece: piece.id })"
                    class="-mx-2 flex items-center gap-2 rounded-md px-2 py-1 hover:bg-muted"
                  >
                    <span class="min-w-0">
                      <span class="font-medium text-highlighted">
                        {{ piece.name }}
                      </span>
                      <span v-if="piece.composer" class="text-sm text-muted">
                        · {{ piece.composer }}
                      </span>
                    </span>
                    <UIcon
                      name="i-lucide-chevron-right"
                      class="ms-auto shrink-0 text-muted"
                    />
                  </ULink>
                </li>
              </ul>
            </div>
            <p v-if="!shownCredits.length" class="text-muted">
              Créditée sur aucun morceau ce semestre.
            </p>
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
              :disabled="!canEdit || shownCredits.length > 0"
              @click="dialog?.remove(shown)"
            />
          </div>
          <p
            v-if="canEdit && shownCredits.length"
            class="-mt-4 text-sm text-muted"
          >
            Retirez-la des crédits de ses morceaux pour pouvoir la supprimer.
          </p>
        </div>
      </DetailPanel>
    </div>

    <PersonDialog
      ref="personDialog"
      @saved="(person) => (selectedId = person.id)"
      @deleted="onDeleted"
    />
  </div>
</template>
