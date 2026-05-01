<script setup lang="ts">
import type { GetDummyErrors, BadRequestReason } from "#hey-api/types.gen";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import { zNewPiece } from "#hey-api/zod.gen";
import { toast } from "vuetify-sonner";

const {
  data: pieces,
  refresh,
  status,
} = getPieces({
  composable: "useAsyncData",
  key: "getPieces",
});

const { data: semesters } = getSemesters({
  composable: "useAsyncData",
  key: "getSemesters",
});

const semesterItems = computed(
  () => semesters.value?.map((s) => ({ title: s.name, value: s.id })) ?? [],
);

const semesterName = (id: number) =>
  semesters.value?.find((s) => s.id === id)?.name ?? String(id);

const { handleSubmit, handleReset, defineField, errors, isSubmitting } =
  useForm({
    validationSchema: toTypedSchema(zNewPiece),
  });

const [name, nameProps] = defineField("name");
const [source, sourceProps] = defineField("source");
const [composer, composerProps] = defineField("composer");
const [arranger, arrangerProps] = defineField("arranger");
const [semester_id, semester_idProps] = defineField("semester_id");

const pieceDialogShown = ref(false);
const savePieceId = ref<number | null>(null);
const savePieceError = ref<string | null>(null);

watch(pieceDialogShown, (shown) => {
  if (shown) {
    savePieceError.value = null;
  }
});

const savePiece = (id: number | null) => {
  const onResponseError = (e: {
    response: { status: number; _data: BadRequestReason };
  }) => {
    if (e.response._data && e.response.status === 400) {
      const d = e.response._data as GetDummyErrors[400];
      if (d.type === "UniqueViolation") {
        savePieceError.value = "Une pièce avec ces informations existe déjà";
      } else {
        savePieceError.value = "Erreur inconnue";
      }
    } else {
      savePieceError.value = `Erreur ${e.response.status}`;
    }
    refresh();
  };

  if (!name.value || !semester_id.value) return;

  const newValue = {
    name: name.value,
    source: source.value || undefined,
    composer: composer.value || undefined,
    arranger: arranger.value || undefined,
    semester_id: semester_id.value,
  };

  let promise;

  if (id !== null) {
    promise = putPiece({
      composable: "$fetch",
      body: { id, ...newValue },
      onResponseError,
    }).then(() => {
      if (pieces.value) {
        pieces.value = pieces.value.map((p) =>
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
      if (pieces.value) {
        pieces.value = [...pieces.value, created];
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

const deletePiece = (id: number, pieceName: string) => {
  confirmDialog(
    "Confirmer la suppression",
    `Êtes-vous sûr de vouloir supprimer la pièce « ${pieceName} » ?`,
  ).then((confirmed) => {
    if (confirmed) {
      deletePieceByPieceId({
        composable: "$fetch",
        path: { piece_id: id },
      })
        .then(() => {
          if (pieces.value) {
            pieces.value = pieces.value.filter((p) => p.id !== id);
          }
          toast.success("Pièce supprimée avec succès");
        })
        .catch(() => {
          refresh();
        });
    }
  });
};
</script>

<template>
  <v-sheet border rounded>
    <v-data-table
      v-if="pieces"
      :loading="status == 'pending'"
      :items="pieces"
      :headers="[
        { title: 'Nom', key: 'name' },
        { title: 'Semestre', key: 'semester_id' },
        { title: 'Source', key: 'source' },
        { title: 'Compositeur', key: 'composer' },
        { title: 'Arrangeur', key: 'arranger' },
        { title: 'Actions', value: 'actions', align: 'end' },
      ]"
      :hide-default-footer="pieces.length < 11"
      :sort-by="[{ key: 'name', order: 'asc' }]"
      item-value="id"
    >
      <template #top>
        <v-toolbar flat>
          <v-toolbar-title>
            <v-icon
              color="medium-emphasis"
              icon="mdi-music-note"
              size="x-small"
              start
            />
            Pièces
          </v-toolbar-title>

          <v-btn
            class="me-2"
            prepend-icon="mdi-plus"
            rounded="lg"
            text="Nouvelle pièce"
            border
            @click="
              () => {
                handleReset();
                savePieceId = null;
                pieceDialogShown = true;
              }
            "
          />
        </v-toolbar>
      </template>

      <template #item.semester_id="{ item }">
        {{ semesterName(item.semester_id) }}
      </template>

      <template #item.actions="{ item }">
        <div class="d-flex ga-2 justify-end">
          <v-icon
            color="medium-emphasis"
            icon="mdi-pencil"
            size="small"
            @click="
              () => {
                name = item.name;
                source = item.source ?? undefined;
                composer = item.composer ?? undefined;
                arranger = item.arranger ?? undefined;
                semester_id = item.semester_id;
                savePieceId = item.id;
                pieceDialogShown = true;
              }
            "
          />
          <v-icon
            color="medium-emphasis"
            icon="mdi-delete"
            size="small"
            @click="deletePiece(item.id, item.name)"
          />
        </div>
      </template>
    </v-data-table>
  </v-sheet>

  <v-dialog v-model="pieceDialogShown" max-width="450" :modal="true">
    <v-form @submit.prevent="submit">
      <v-card
        :title="savePieceId === null ? 'Nouvelle pièce' : 'Modifier pièce'"
      >
        <v-card-text>
          <v-text-field
            v-model="name"
            v-bind="nameProps"
            label="Nom"
            :error-messages="errors.name"
            autofocus
          />
          <v-select
            v-model="semester_id"
            v-bind="semester_idProps"
            label="Semestre"
            :items="semesterItems"
            :error-messages="errors.semester_id"
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
          <v-btn @click="() => (pieceDialogShown = false)">Annuler</v-btn>
          <v-btn :loading="isSubmitting" type="submit">Enregistrer</v-btn>
        </v-card-actions>
      </v-card>
    </v-form>
  </v-dialog>
</template>
