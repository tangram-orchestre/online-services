<script setup lang="ts">
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import { zNewPiece } from "#hey-api/zod.gen";
import type { Piece, Semester } from "#hey-api/types.gen";
import { toast } from "vuetify-sonner";

const props = defineProps<{
  semester: Semester;
}>();

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
  const onResponseError = onResponseErrorHandler("pièce", true, (errorMsg) => {
    savePieceError.value = errorMsg;
  });

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
    toast.success("Pièce enregistrée avec succès");
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
const { smAndDown } = useDisplay();
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
    `Êtes-vous sûr de vouloir supprimer la pièce « ${pieceName} » ?`,
  ).then((confirmed) => {
    if (confirmed) {
      deletePieceByPieceId({
        composable: "$fetch",
        path: { piece_id: id },
        onResponseError: onResponseErrorHandler("pièce", true, (errorMsg) => {
          toast.error(errorMsg);
          refresh();
        }),
      }).then(() => {
        if (allPieces.value) {
          allPieces.value = allPieces.value.filter((p) => p.id !== id);
        }
        pieceDialogShown.value = false;
        toast.success("Pièce supprimée avec succès");
      });
    }
  });
};
</script>

<template>
  <section>
    <div class="d-flex align-center mb-3">
      <h2 class="text-h6 d-flex align-center">
        <v-icon
          color="medium-emphasis"
          icon="mdi-music-note"
          size="small"
          start
        />
        Pièces
      </h2>
      <v-btn
        class="ms-1"
        icon="mdi-plus"
        size="small"
        variant="text"
        color="medium-emphasis"
        title="Nouvelle pièce"
        @click="newPiece()"
      />
    </div>

    <template v-if="smAndDown">
      <v-progress-linear
        v-if="status == 'pending' && pieces.length === 0"
        indeterminate
      />
      <div
        v-else-if="pieces.length === 0"
        class="text-body-2 text-medium-emphasis py-4 text-center"
      >
        Aucune pièce pour ce semestre
      </div>
      <div v-else class="d-flex flex-column ga-2">
        <v-card v-for="piece in piecesByName" :key="piece.id" border flat>
          <v-btn
            class="edit-button"
            icon="mdi-pencil"
            size="small"
            variant="text"
            color="medium-emphasis"
            title="Modifier la pièce"
            @click="editPiece(piece)"
          />
          <v-card-item class="pe-12">
            <v-card-title>{{ piece.name }}</v-card-title>
            <v-card-subtitle v-if="piece.composer || piece.arranger">
              {{ piece.composer }}
              <template v-if="piece.arranger">
                <template v-if="piece.composer"> · </template>
                arr. {{ piece.arranger }}
              </template>
            </v-card-subtitle>
          </v-card-item>
          <v-card-text v-if="piece.source" class="text-medium-emphasis">
            {{ piece.source }}
          </v-card-text>
        </v-card>
      </div>
    </template>

    <v-sheet v-else border rounded>
      <v-data-table
        :loading="status == 'pending'"
        :items="pieces"
        :headers="[
          { title: 'Nom', key: 'name' },
          { title: 'Source', key: 'source' },
          { title: 'Compositeur', key: 'composer' },
          { title: 'Arrangeur', key: 'arranger' },
          { title: 'Actions', value: 'actions', align: 'end', sortable: false },
        ]"
        :hide-default-footer="pieces.length < 11"
        :sort-by="[{ key: 'name', order: 'asc' }]"
        no-data-text="Aucune pièce pour ce semestre"
        item-value="id"
      >
        <template #item.actions="{ item }">
          <div class="d-flex ga-2 justify-end">
            <v-icon
              color="medium-emphasis"
              icon="mdi-pencil"
              size="small"
              @click="editPiece(item)"
            />
          </div>
        </template>
      </v-data-table>
    </v-sheet>
  </section>

  <v-dialog v-model="pieceDialogShown" max-width="450" :modal="true">
    <v-form @submit.prevent="submit">
      <v-card
        :title="savePieceId === null ? 'Nouvelle pièce' : 'Modifier pièce'"
        :subtitle="`Semestre ${semester.name}`"
      >
        <v-card-text>
          <v-text-field
            v-model="name"
            v-bind="nameProps"
            label="Nom"
            :error-messages="errors.name"
            autofocus
          />
          <v-text-field
            v-model="source"
            v-bind="sourceProps"
            label="Source"
            :error-messages="errors.source"
          />
          <v-text-field
            v-model="composer"
            v-bind="composerProps"
            label="Compositeur"
            :error-messages="errors.composer"
          />
          <v-text-field
            v-model="arranger"
            v-bind="arrangerProps"
            label="Arrangeur"
            :error-messages="errors.arranger"
          />
          <div v-if="savePieceError" class="text-error">
            {{ savePieceError }}
          </div>
        </v-card-text>

        <v-card-actions>
          <v-btn
            v-if="savePieceId !== null"
            color="error"
            prepend-icon="mdi-delete"
            text="Supprimer"
            @click="deleteEditedPiece()"
          />
          <v-spacer />
          <v-btn @click="() => (pieceDialogShown = false)">Annuler</v-btn>
          <v-btn :loading="isSubmitting" type="submit">Enregistrer</v-btn>
        </v-card-actions>
      </v-card>
    </v-form>
  </v-dialog>
</template>

<style scoped>
.edit-button {
  position: absolute;
  top: 4px;
  right: 4px;
}
</style>
