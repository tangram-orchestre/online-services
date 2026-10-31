<script setup lang="ts">
import { VueDraggable } from "vue-draggable-plus";
import type { ConcertWithPieces, Piece } from "#hey-api/types.gen";
import { toast } from "vuetify-sonner";

const props = defineProps<{
  /** Pieces that can be added to the programme (the concert's semester). */
  pieces: Piece[];
}>();

const emit = defineEmits<{
  saved: [concertId: number, pieceIds: number[]];
}>();

const formatDate = (value: string) => new Date(value).toLocaleDateString("fr");

const shown = ref(false);
const concert = ref<ConcertWithPieces | null>(null);
const programme = ref<Piece[]>([]);
const saveError = ref<string | null>(null);
const saving = ref(false);

/** Open the dialog to edit the programme of `c`. */
const open = (c: ConcertWithPieces) => {
  concert.value = c;
  programme.value = c.piece_ids
    .map((id) => props.pieces.find((p) => p.id === id))
    .filter((p): p is Piece => p !== undefined);
  saveError.value = null;
  shown.value = true;
};

defineExpose({ open });

const availablePieces = computed(() =>
  props.pieces
    .filter((p) => !programme.value.some((selected) => selected.id === p.id))
    .sort((a, b) => a.name.localeCompare(b.name)),
);

// Bound to the "add a piece" autocomplete, reset right after each pick.
const pieceToAdd = ref<number | null>(null);
watch(pieceToAdd, (id) => {
  if (id === null) return;
  const piece = props.pieces.find((p) => p.id === id);
  if (piece) {
    programme.value = [...programme.value, piece];
  }
  nextTick(() => {
    pieceToAdd.value = null;
  });
});

const remove = (id: number) => {
  programme.value = programme.value.filter((p) => p.id !== id);
};

const save = () => {
  if (!concert.value) return;
  const concertId = concert.value.id;
  const pieceIds = programme.value.map((p) => p.id);

  saving.value = true;
  putConcertByConcertIdPieces({
    composable: "$fetch",
    path: { concert_id: concertId },
    body: pieceIds,
    onResponseError: onResponseErrorHandler("programme", false, (errorMsg) => {
      saveError.value = errorMsg;
    }),
  })
    .then(() => {
      shown.value = false;
      toast.success("Programme enregistré avec succès");
      emit("saved", concertId, pieceIds);
    })
    .finally(() => {
      saving.value = false;
    });
};
</script>

<template>
  <v-dialog v-model="shown" max-width="500" :modal="true" scrollable>
    <v-card
      title="Programme"
      :subtitle="concert ? `Concert du ${formatDate(concert.date)}` : undefined"
    >
      <v-card-text>
        <div
          v-if="programme.length === 0"
          class="text-body-2 text-medium-emphasis text-center py-4"
        >
          Aucun morceau au programme
        </div>
        <v-list v-else density="compact" class="py-0">
          <VueDraggable
            v-model="programme"
            :animation="150"
            handle=".drag-handle"
            ghost-class="programme-ghost"
          >
            <v-list-item
              v-for="(piece, index) in programme"
              :key="piece.id"
              :title="`${index + 1}. ${piece.name}`"
              :subtitle="piece.composer ?? undefined"
              rounded
            >
              <template #prepend>
                <v-icon
                  class="drag-handle me-2"
                  icon="mdi-drag"
                  color="medium-emphasis"
                />
              </template>
              <template #append>
                <v-btn
                  icon="mdi-close"
                  size="small"
                  variant="text"
                  title="Retirer du programme"
                  @click="remove(piece.id)"
                />
              </template>
            </v-list-item>
          </VueDraggable>
        </v-list>

        <v-autocomplete
          v-model="pieceToAdd"
          class="mt-4"
          :items="availablePieces"
          item-title="name"
          item-value="id"
          label="Ajouter un morceau"
          prepend-inner-icon="mdi-plus"
          no-data-text="Tous les morceaux du semestre sont au programme"
          hide-details
        >
          <template #item="{ props: itemProps, item }">
            <v-list-item
              v-bind="itemProps"
              :subtitle="item.raw.composer ?? undefined"
            />
          </template>
        </v-autocomplete>

        <div v-if="saveError" class="text-error mt-2">
          {{ saveError }}
        </div>
      </v-card-text>

      <v-card-actions>
        <v-btn @click="() => (shown = false)">Annuler</v-btn>
        <v-btn :loading="saving" @click="save">Enregistrer</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<style scoped>
.drag-handle {
  cursor: grab;
}

:deep(.programme-ghost) {
  opacity: 0.4;
}
</style>
