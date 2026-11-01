<script setup lang="ts">
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import { zNewPiece } from "#hey-api/zod.gen";
import type { Piece, Semester } from "#hey-api/types.gen";
import type { TableColumn } from "@nuxt/ui";

const props = defineProps<{
  semester: Semester;
}>();

const toast = useToast();
const formId = useId();

const {
  data: allPieces,
  refresh,
  status,
} = getPieces({
  composable: "useAsyncData",
  key: "getPieces",
});

// TODO: filter server side once the API supports it.
const pieces = computed(
  () =>
    allPieces.value?.filter((p) => p.semester_id === props.semester.id) ?? [],
);

const { data: allConcerts } = getConcerts({
  composable: "useAsyncData",
  key: "getConcerts",
});

const formatShortDate = (value: string) =>
  new Date(value).toLocaleDateString("fr", { day: "numeric", month: "short" });

/** Concerts whose programme contains the piece, by date. */
const concertsOf = (pieceId: number) =>
  (allConcerts.value ?? [])
    .filter((c) => c.piece_ids.includes(pieceId))
    .sort((a, b) => a.date.localeCompare(b.date));

const { handleSubmit, handleReset, defineField, errors, isSubmitting } =
  useForm({
    validationSchema: toTypedSchema(zNewPiece.omit({ semester_id: true })),
  });

const [name, nameProps] = defineField("name");
const [source, sourceProps] = defineField("source");
const [composer, composerProps] = defineField("composer");
const [arranger, arrangerProps] = defineField("arranger");

const pieceDialogShown = ref(false);
const savePieceId = ref<number | null>(null);
const savePieceError = ref<string | null>(null);

watch(pieceDialogShown, (shown) => {
  if (shown) {
    savePieceError.value = null;
  }
});

const savePiece = (id: number | null) => {
  const onResponseError = onResponseErrorHandler(
    "morceau",
    false,
    (errorMsg) => {
      savePieceError.value = errorMsg;
    },
  );

  if (!name.value) return;

  const newValue = {
    name: name.value,
    source: source.value || undefined,
    composer: composer.value || undefined,
    arranger: arranger.value || undefined,
    semester_id: props.semester.id,
  };

  let promise;

  if (id !== null) {
    promise = putPiece({
      composable: "$fetch",
      body: { id, ...newValue },
      onResponseError,
    }).then(() => {
      if (allPieces.value) {
        allPieces.value = allPieces.value.map((p) =>
          p.id === id ? { ...p, ...newValue } : p,
        );
      }
    });
  } else {
    promise = postPiece({
      composable: "$fetch",
      body: newValue,
      onResponseError,
    }).then((created) => {
      if (allPieces.value) {
        allPieces.value = [...allPieces.value, created];
      }
    });
  }

  promise.then(() => {
    pieceDialogShown.value = false;
    toast.add({ title: "Morceau enregistré avec succès", color: "success" });
  });
};

const submit = handleSubmit(() => {
  savePiece(savePieceId.value);
});

const editPiece = (piece: Piece) => {
  name.value = piece.name;
  source.value = piece.source ?? undefined;
  composer.value = piece.composer ?? undefined;
  arranger.value = piece.arranger ?? undefined;
  savePieceId.value = piece.id;
  pieceDialogShown.value = true;
};

// Small screens get one card per piece instead of the (too wide) table.
const piecesByName = computed(() =>
  [...pieces.value].sort((a, b) => a.name.localeCompare(b.name)),
);

const newPiece = () => {
  handleReset();
  savePieceId.value = null;
  pieceDialogShown.value = true;
};

const deleteEditedPiece = () => {
  const piece = allPieces.value?.find((p) => p.id === savePieceId.value);
  if (piece) {
    deletePiece(piece.id, piece.name);
  }
};

const deletePiece = (id: number, pieceName: string) => {
  confirmDialog(
    "Confirmer la suppression",
    `Êtes-vous sûr de vouloir supprimer le morceau « ${pieceName} » ?`,
  ).then((confirmed) => {
    if (confirmed) {
      deletePieceByPieceId({
        composable: "$fetch",
        path: { piece_id: id },
        onResponseError: onResponseErrorHandler(
          "morceau",
          false,
          (errorMsg) => {
            toast.add({ title: errorMsg, color: "error" });
            refresh();
          },
        ),
      }).then(() => {
        if (allPieces.value) {
          allPieces.value = allPieces.value.filter((p) => p.id !== id);
        }
        pieceDialogShown.value = false;
        toast.add({ title: "Morceau supprimé avec succès", color: "success" });
      });
    }
  });
};

const columns: TableColumn<Piece>[] = [
  { accessorKey: "name", header: sortableHeader("Nom") },
  { accessorKey: "source", header: sortableHeader("Source") },
  { accessorKey: "composer", header: sortableHeader("Compositeur") },
  { accessorKey: "arranger", header: sortableHeader("Arrangeur") },
  { id: "concerts", header: "Concerts" },
  { id: "actions", meta: { class: { td: "text-right" } } },
];
const sorting = ref([{ id: "name", desc: false }]);
</script>

<template>
  <section>
    <div class="mb-3 flex items-center gap-1">
      <h2 class="flex items-center gap-2 text-lg font-semibold">
        <UIcon name="i-lucide-music-2" class="text-muted" />
        Morceaux
      </h2>
      <UButton
        icon="i-lucide-plus"
        color="neutral"
        variant="ghost"
        aria-label="Nouveau morceau"
        @click="newPiece()"
      />
    </div>

    <!-- Small screens: one card per piece -->
    <div class="md:hidden">
      <UProgress v-if="status == 'pending' && pieces.length === 0" />
      <p
        v-else-if="pieces.length === 0"
        class="py-4 text-center text-sm text-muted"
      >
        Aucun morceau pour ce semestre
      </p>
      <div v-else class="flex flex-col gap-2">
        <UCard v-for="piece in piecesByName" :key="piece.id" class="relative">
          <UButton
            class="absolute top-2 right-2"
            icon="i-lucide-pencil"
            color="neutral"
            variant="ghost"
            size="sm"
            aria-label="Modifier le morceau"
            @click="editPiece(piece)"
          />
          <div class="pe-8">
            <p class="font-medium">{{ piece.name }}</p>
            <p
              v-if="piece.composer || piece.arranger"
              class="text-sm text-muted"
            >
              {{ piece.composer }}
              <template v-if="piece.arranger">
                <template v-if="piece.composer"> · </template>
                arr. {{ piece.arranger }}
              </template>
            </p>
          </div>
          <p v-if="piece.source" class="mt-2 text-sm text-muted">
            {{ piece.source }}
          </p>
          <div
            v-if="concertsOf(piece.id).length"
            class="mt-2 flex flex-wrap gap-1"
          >
            <UBadge
              v-for="concert in concertsOf(piece.id)"
              :key="concert.id"
              :label="formatShortDate(concert.date)"
              icon="i-lucide-music"
              color="neutral"
              variant="subtle"
              size="sm"
            />
          </div>
        </UCard>
      </div>
    </div>

    <!-- Larger screens: table -->
    <UCard class="hidden md:block" :ui="{ body: 'p-0 sm:p-0' }">
      <UTable
        v-model:sorting="sorting"
        :data="pieces"
        :columns="columns"
        :loading="status == 'pending'"
        empty="Aucun morceau pour ce semestre"
      >
        <template #concerts-cell="{ row }">
          <div class="flex flex-wrap gap-1">
            <UBadge
              v-for="concert in concertsOf(row.original.id)"
              :key="concert.id"
              :label="formatShortDate(concert.date)"
              color="neutral"
              variant="subtle"
              size="sm"
            />
            <span
              v-if="concertsOf(row.original.id).length === 0"
              class="text-muted"
              >—</span
            >
          </div>
        </template>

        <template #actions-cell="{ row }">
          <UButton
            icon="i-lucide-pencil"
            color="neutral"
            variant="ghost"
            size="sm"
            aria-label="Modifier le morceau"
            @click="editPiece(row.original)"
          />
        </template>
      </UTable>
    </UCard>
  </section>

  <UModal
    v-model:open="pieceDialogShown"
    :title="savePieceId === null ? 'Nouveau morceau' : 'Modifier le morceau'"
    :description="semester.long_name"
  >
    <template #body>
      <form :id="formId" class="flex flex-col gap-4" @submit.prevent="submit">
        <UFormField label="Nom" :error="errors.name">
          <UInput v-model="name" v-bind="nameProps" autofocus class="w-full" />
        </UFormField>
        <UFormField label="Source" :error="errors.source">
          <UInput v-model="source" v-bind="sourceProps" class="w-full" />
        </UFormField>
        <UFormField label="Compositeur" :error="errors.composer">
          <UInput v-model="composer" v-bind="composerProps" class="w-full" />
        </UFormField>
        <UFormField label="Arrangeur" :error="errors.arranger">
          <UInput v-model="arranger" v-bind="arrangerProps" class="w-full" />
        </UFormField>
        <p v-if="savePieceError" class="text-sm text-error">
          {{ savePieceError }}
        </p>
      </form>
    </template>

    <template #footer>
      <div class="flex w-full items-center gap-2">
        <UButton
          v-if="savePieceId !== null"
          label="Supprimer"
          icon="i-lucide-trash-2"
          color="error"
          variant="ghost"
          @click="deleteEditedPiece()"
        />
        <UButton
          class="ms-auto"
          label="Annuler"
          color="neutral"
          variant="ghost"
          @click="pieceDialogShown = false"
        />
        <UButton
          label="Enregistrer"
          type="submit"
          :form="formId"
          :loading="isSubmitting"
        />
      </div>
    </template>
  </UModal>
</template>
