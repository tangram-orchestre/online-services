<script setup lang="ts">
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import { zNewPiece } from "#hey-api/zod.gen";
import type { Piece, Semester } from "#hey-api/types.gen";

const props = defineProps<{
  semester: Semester;
}>();

const emit = defineEmits<{
  /** After a creation or an update. */
  saved: [pieceId: number];
  deleted: [pieceId: number];
}>();

const toast = useToast();
const formId = useId();

const { allPieces, allConcerts, refreshPieces } = useProgramme();

const { handleSubmit, resetForm, defineField, errors, isSubmitting } = useForm({
  validationSchema: toTypedSchema(zNewPiece.omit({ semester_id: true })),
});

const [name, nameProps] = defineField("name");
const [source, sourceProps] = defineField("source");
const [composer, composerProps] = defineField("composer");
const [arranger, arrangerProps] = defineField("arranger");

const shown = ref(false);
const editedId = ref<number | null>(null);
const saveError = ref<string | null>(null);

/** Open the dialog to edit `piece`, or to create a new one if omitted. */
const open = (piece?: Piece) => {
  saveError.value = null;
  editedId.value = piece?.id ?? null;
  resetForm({
    values: {
      name: piece?.name ?? "",
      source: piece?.source ?? undefined,
      composer: piece?.composer ?? undefined,
      arranger: piece?.arranger ?? undefined,
    },
  });
  shown.value = true;
};

const submit = handleSubmit((values) => {
  const onResponseError = onResponseErrorHandler(
    "morceau",
    false,
    (errorMsg) => {
      saveError.value = errorMsg;
    },
  );

  const newValue = {
    name: values.name,
    source: values.source || undefined,
    composer: values.composer || undefined,
    arranger: values.arranger || undefined,
    semester_id: props.semester.id,
  };

  const id = editedId.value;
  const promise: Promise<number> =
    id !== null
      ? putPiece({
          composable: "$fetch",
          body: { id, ...newValue },
          onResponseError,
        }).then(() => {
          if (allPieces.value) {
            allPieces.value = allPieces.value.map((p) =>
              p.id === id ? { ...p, ...newValue } : p,
            );
          }
          return id;
        })
      : postPiece({
          composable: "$fetch",
          body: newValue,
          onResponseError,
        }).then((created) => {
          if (allPieces.value) {
            allPieces.value = [...allPieces.value, created];
          }
          return created.id;
        });

  return promise.then((savedId) => {
    shown.value = false;
    toast.add({ title: "Morceau enregistré avec succès", color: "success" });
    emit("saved", savedId);
  });
});

/** Ask for confirmation, then delete `piece`. */
const remove = (piece: Piece) => {
  confirmDialog(
    "Confirmer la suppression",
    `Êtes-vous sûr de vouloir supprimer le morceau « ${piece.name} » ?`,
  ).then((confirmed) => {
    if (!confirmed) return;
    deletePieceByPieceId({
      composable: "$fetch",
      path: { piece_id: piece.id },
      onResponseError: onResponseErrorHandler("morceau", false, (errorMsg) => {
        toast.add({ title: errorMsg, color: "error" });
        refreshPieces();
      }),
    }).then(() => {
      if (allPieces.value) {
        allPieces.value = allPieces.value.filter((p) => p.id !== piece.id);
      }
      // The piece also leaves the programmes it was on.
      if (allConcerts.value) {
        allConcerts.value = allConcerts.value.map((c) => ({
          ...c,
          piece_ids: c.piece_ids.filter((id) => id !== piece.id),
        }));
      }
      shown.value = false;
      toast.add({ title: "Morceau supprimé avec succès", color: "success" });
      emit("deleted", piece.id);
    });
  });
};

defineExpose({ open, remove });

const removeEdited = () => {
  const piece = allPieces.value?.find((p) => p.id === editedId.value);
  if (piece) remove(piece);
};
</script>

<template>
  <UModal
    v-model:open="shown"
    :title="editedId === null ? 'Nouveau morceau' : 'Modifier le morceau'"
    :description="semester.long_name"
  >
    <template #body>
      <form :id="formId" class="flex flex-col gap-4" @submit.prevent="submit">
        <UFormField label="Nom" :error="errors.name">
          <UInput v-model="name" v-bind="nameProps" autofocus class="w-full" />
        </UFormField>
        <UFormField label="Compositeur" :error="errors.composer">
          <UInput v-model="composer" v-bind="composerProps" class="w-full" />
        </UFormField>
        <UFormField label="Arrangeur" :error="errors.arranger">
          <UInput v-model="arranger" v-bind="arrangerProps" class="w-full" />
        </UFormField>
        <UFormField label="Source" :error="errors.source">
          <UInput v-model="source" v-bind="sourceProps" class="w-full" />
        </UFormField>
        <p v-if="saveError" class="text-sm text-error">{{ saveError }}</p>
      </form>
    </template>

    <template #footer>
      <div class="flex w-full items-center gap-2">
        <UButton
          v-if="editedId !== null"
          label="Supprimer"
          icon="i-lucide-trash-2"
          color="error"
          variant="ghost"
          @click="removeEdited()"
        />
        <UButton
          class="ms-auto"
          label="Annuler"
          color="neutral"
          variant="ghost"
          @click="shown = false"
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
