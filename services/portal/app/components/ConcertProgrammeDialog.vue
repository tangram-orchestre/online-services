<script setup lang="ts">
import { VueDraggable } from "vue-draggable-plus";
import type { ConcertWithPieces, Piece } from "#hey-api/types.gen";

const props = defineProps<{
  /** Pieces that can be added to the programme (the concert's semester). */
  pieces: Piece[];
}>();

const emit = defineEmits<{
  saved: [concertId: number, pieceIds: number[]];
}>();

const toast = useToast();

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
    .sort((a, b) => a.name.localeCompare(b.name))
    .map((p) => ({
      label: p.name,
      description: p.composer ?? undefined,
      value: p.id,
    })),
);

// Bound to the "add a piece" autocomplete, reset right after each pick.
const pieceToAdd = ref<number | undefined>(undefined);
watch(pieceToAdd, (id) => {
  if (id === undefined) return;
  const piece = props.pieces.find((p) => p.id === id);
  if (piece) {
    programme.value = [...programme.value, piece];
  }
  nextTick(() => {
    pieceToAdd.value = undefined;
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
      toast.add({
        title: "Programme enregistré avec succès",
        color: "success",
      });
      emit("saved", concertId, pieceIds);
    })
    .finally(() => {
      saving.value = false;
    });
};
</script>

<template>
  <UModal
    v-model:open="shown"
    title="Programme"
    :description="
      concert ? `Concert du ${formatDate(concert.date)}` : undefined
    "
  >
    <template #body>
      <p
        v-if="programme.length === 0"
        class="py-4 text-center text-sm text-muted"
      >
        Aucun morceau au programme
      </p>
      <VueDraggable
        v-else
        v-model="programme"
        tag="ul"
        :animation="150"
        handle=".drag-handle"
        ghost-class="opacity-40"
        class="divide-y divide-default"
      >
        <li
          v-for="(piece, index) in programme"
          :key="piece.id"
          class="flex items-center gap-3 py-2"
        >
          <UIcon
            name="i-lucide-grip-vertical"
            class="drag-handle shrink-0 cursor-grab text-muted"
          />
          <div class="min-w-0 flex-1">
            <p class="truncate">{{ index + 1 }}. {{ piece.name }}</p>
            <p v-if="piece.composer" class="truncate text-sm text-muted">
              {{ piece.composer }}
            </p>
          </div>
          <UButton
            icon="i-lucide-x"
            color="neutral"
            variant="ghost"
            size="sm"
            aria-label="Retirer du programme"
            @click="remove(piece.id)"
          />
        </li>
      </VueDraggable>

      <USelectMenu
        v-model="pieceToAdd"
        :items="availablePieces"
        value-key="value"
        icon="i-lucide-plus"
        placeholder="Ajouter un morceau"
        :search-input="{ placeholder: 'Rechercher…' }"
        class="mt-4 w-full"
      >
        <template #empty>
          Tous les morceaux du semestre sont au programme
        </template>
      </USelectMenu>

      <p v-if="saveError" class="mt-2 text-sm text-error">{{ saveError }}</p>
    </template>

    <template #footer>
      <div class="flex w-full justify-end gap-2">
        <UButton
          label="Annuler"
          color="neutral"
          variant="ghost"
          @click="shown = false"
        />
        <UButton label="Enregistrer" :loading="saving" @click="save" />
      </div>
    </template>
  </UModal>
</template>
