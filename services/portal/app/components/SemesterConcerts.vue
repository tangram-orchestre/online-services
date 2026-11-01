<script setup lang="ts">
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import { zNewConcert } from "#hey-api/zod.gen";
import type { Concert, ConcertWithPieces, Semester } from "#hey-api/types.gen";
import type ConcertProgrammeDialogComponent from "~/components/ConcertProgrammeDialog.vue";

const props = defineProps<{
  semester: Semester;
}>();

const toast = useToast();
const formId = useId();

const formatDate = (value: string) => new Date(value).toLocaleDateString("fr");
const formatTime = (value: string) => value.slice(0, 5);
const formatLongDate = (value: string) =>
  new Date(value).toLocaleDateString("fr", {
    weekday: "short",
    day: "numeric",
    month: "long",
  });
const normalizeTime = (value: string) =>
  value.length === 5 ? `${value}:00` : value;

const {
  data: allConcerts,
  refresh,
  status,
} = getConcerts({
  composable: "useAsyncData",
  key: "getConcerts",
});

// TODO: filter server side once the API supports it.
const concerts = computed(
  () =>
    allConcerts.value
      ?.filter((c) => c.semester_id === props.semester.id)
      .sort((a, b) => a.date.localeCompare(b.date)) ?? [],
);

const { data: locations } = getLocations({
  composable: "useAsyncData",
  key: "getLocations",
});

const locationItems = computed(
  () =>
    locations.value?.map((l) => ({
      label: `${l.name} — ${l.city}`,
      value: l.id,
    })) ?? [],
);

const location = (id: number) => locations.value?.find((l) => l.id === id);

const { data: allPieces } = getPieces({
  composable: "useAsyncData",
  key: "getPieces",
});

const semesterPieces = computed(
  () =>
    allPieces.value?.filter((p) => p.semester_id === props.semester.id) ?? [],
);

const pieceName = (id: number) =>
  allPieces.value?.find((p) => p.id === id)?.name ?? "Morceau inconnu";

const programmeDialog =
  useTemplateRef<InstanceType<typeof ConcertProgrammeDialogComponent>>(
    "programmeDialog",
  );

const onProgrammeSaved = (concertId: number, pieceIds: number[]) => {
  if (allConcerts.value) {
    allConcerts.value = allConcerts.value.map((c) =>
      c.id === concertId ? { ...c, piece_ids: pieceIds } : c,
    );
  }
};

const zConcertForm = zNewConcert
  .omit({ semester_id: true })
  .refine((obj) => !obj.doors_open_at || obj.doors_open_at < obj.starts_at, {
    path: ["doors_open_at"],
    message:
      "L'ouverture des portes doit être avant l'heure de début du concert",
  })
  .refine((obj) => !obj.meeting_at || obj.meeting_at < obj.starts_at, {
    path: ["meeting_at"],
    message:
      "Le rendez-vous des musiciens doit être avant l'heure de début du concert",
  });

const { handleSubmit, handleReset, defineField, errors, isSubmitting } =
  useForm({
    validationSchema: toTypedSchema(zConcertForm),
  });

const [date] = defineField("date");
const [starts_at] = defineField("starts_at");
const [doors_open_at] = defineField("doors_open_at");
const [meeting_at] = defineField("meeting_at");
const [tickets_url, tickets_urlProps] = defineField("tickets_url");
const [video_url, video_urlProps] = defineField("video_url");
const [isPublic, isPublicProps] = defineField("public");
const [location_id] = defineField("location_id");

const concertDialogShown = ref(false);
const saveConcertId = ref<number | null>(null);
const saveConcertError = ref<string | null>(null);

watch(concertDialogShown, (shown) => {
  if (shown) {
    saveConcertError.value = null;
  }
});

const saveConcert = (id: number | null) => {
  const onResponseError = onResponseErrorHandler(
    "concert",
    false,
    (errorMsg) => {
      saveConcertError.value = errorMsg;
    },
  );

  if (!date.value || !starts_at.value || !location_id.value) {
    return;
  }

  const newValue = {
    date: date.value,
    starts_at: normalizeTime(starts_at.value),
    doors_open_at: doors_open_at.value
      ? normalizeTime(doors_open_at.value)
      : undefined,
    meeting_at: meeting_at.value ? normalizeTime(meeting_at.value) : undefined,
    tickets_url: tickets_url.value || undefined,
    video_url: video_url.value || undefined,
    public: isPublic.value ?? false,
    location_id: location_id.value,
    semester_id: props.semester.id,
  };

  let promise;

  if (id !== null) {
    promise = putConcert({
      composable: "$fetch",
      body: { id, ...newValue },
      onResponseError,
    }).then(() => {
      if (allConcerts.value) {
        allConcerts.value = allConcerts.value.map((c) =>
          c.id === id ? { ...c, ...newValue } : c,
        );
      }
    });
  } else {
    promise = postConcert({
      composable: "$fetch",
      body: newValue,
      onResponseError,
    }).then((created) => {
      if (allConcerts.value) {
        allConcerts.value = [...allConcerts.value, created];
      }
    });
  }

  promise.then(() => {
    concertDialogShown.value = false;
    toast.add({ title: "Concert enregistré avec succès", color: "success" });
  });
};

const submit = handleSubmit(() => {
  saveConcert(saveConcertId.value);
});

const newConcert = () => {
  handleReset();
  saveConcertId.value = null;
  concertDialogShown.value = true;
};

const editConcert = (concert: Concert | ConcertWithPieces) => {
  date.value = concert.date;
  starts_at.value = concert.starts_at;
  doors_open_at.value = concert.doors_open_at ?? undefined;
  meeting_at.value = concert.meeting_at ?? undefined;
  tickets_url.value = concert.tickets_url ?? undefined;
  video_url.value = concert.video_url ?? undefined;
  isPublic.value = concert.public;
  location_id.value = concert.location_id;
  saveConcertId.value = concert.id;
  concertDialogShown.value = true;
};

const deleteEditedConcert = () => {
  const concert = allConcerts.value?.find((c) => c.id === saveConcertId.value);
  if (concert) {
    deleteConcert(concert.id, concert.date);
  }
};

const deleteConcert = (id: number, concertDate: string) => {
  confirmDialog(
    "Confirmer la suppression",
    `Êtes-vous sûr de vouloir supprimer le concert du ${formatDate(concertDate)} ?`,
  ).then((confirmed) => {
    if (confirmed) {
      deleteConcertByConcertId({
        composable: "$fetch",
        path: { concert_id: id },
        onResponseError: onResponseErrorHandler(
          "concert",
          false,
          (errorMsg) => {
            toast.add({ title: errorMsg, color: "error" });
            refresh();
          },
        ),
      }).then(() => {
        if (allConcerts.value) {
          allConcerts.value = allConcerts.value.filter((c) => c.id !== id);
        }
        concertDialogShown.value = false;
        toast.add({ title: "Concert supprimé avec succès", color: "success" });
      });
    }
  });
};
</script>

<template>
  <section>
    <div class="mb-3 flex items-center gap-1">
      <h2 class="flex items-center gap-2 text-lg font-semibold">
        <UIcon name="i-lucide-music" class="text-muted" />
        Concerts
      </h2>
      <UButton
        icon="i-lucide-plus"
        color="neutral"
        variant="ghost"
        aria-label="Nouveau concert"
        @click="newConcert()"
      />
    </div>

    <UProgress v-if="status == 'pending' && concerts.length === 0" />
    <p
      v-else-if="concerts.length === 0"
      class="py-4 text-center text-sm text-muted"
    >
      Aucun concert pour ce semestre
    </p>
    <div v-else class="grid gap-3 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
      <UCard
        v-for="concert in concerts"
        :key="concert.id"
        class="relative flex flex-col"
        :ui="{ body: 'flex-1', footer: 'flex items-center gap-1' }"
      >
        <template #header>
          <UButton
            class="absolute top-2 right-2"
            icon="i-lucide-pencil"
            color="neutral"
            variant="ghost"
            size="sm"
            aria-label="Modifier le concert"
            @click="editConcert(concert)"
          />
          <div class="pe-8">
            <p class="font-semibold capitalize">
              {{ formatLongDate(concert.date) }}
            </p>
            <p class="flex items-center gap-1 text-sm text-muted">
              <UIcon name="i-lucide-map-pin" class="shrink-0" />
              {{ location(concert.location_id)?.name ?? "Lieu inconnu" }}
              <template v-if="location(concert.location_id)">
                · {{ location(concert.location_id)?.city }}
              </template>
            </p>
          </div>
        </template>

        <div class="flex flex-col gap-1 text-sm">
          <div v-if="concert.meeting_at" class="flex items-center gap-2">
            <UIcon name="i-lucide-users" class="text-muted" />
            Rendez-vous
            <span class="ms-auto font-medium">
              {{ formatTime(concert.meeting_at) }}
            </span>
          </div>
          <div v-if="concert.doors_open_at" class="flex items-center gap-2">
            <UIcon name="i-lucide-door-open" class="text-muted" />
            Ouverture des portes
            <span class="ms-auto font-medium">
              {{ formatTime(concert.doors_open_at) }}
            </span>
          </div>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-music" class="text-muted" />
            Début
            <span class="ms-auto font-medium">
              {{ formatTime(concert.starts_at) }}
            </span>
          </div>
        </div>

        <USeparator class="my-3" />
        <p class="mb-1 text-xs font-medium tracking-wide text-muted uppercase">
          Programme
        </p>
        <ol v-if="concert.piece_ids.length" class="list-decimal ps-5 text-sm">
          <li v-for="pieceId in concert.piece_ids" :key="pieceId">
            {{ pieceName(pieceId) }}
          </li>
        </ol>
        <p v-else class="text-sm text-muted">Programme vide</p>

        <template #footer>
          <UBadge
            :label="concert.public ? 'Public' : 'Privé'"
            :icon="concert.public ? 'i-lucide-globe' : 'i-lucide-lock'"
            :color="concert.public ? 'success' : 'neutral'"
            variant="subtle"
          />
          <UButton
            v-if="concert.tickets_url"
            :to="concert.tickets_url"
            target="_blank"
            icon="i-lucide-ticket"
            color="neutral"
            variant="ghost"
            size="sm"
            aria-label="Billetterie"
          />
          <UButton
            v-if="concert.video_url"
            :to="concert.video_url"
            target="_blank"
            icon="i-lucide-circle-play"
            color="neutral"
            variant="ghost"
            size="sm"
            aria-label="Vidéo"
          />
          <UButton
            class="ms-auto"
            icon="i-lucide-list-music"
            color="neutral"
            variant="ghost"
            size="sm"
            aria-label="Modifier le programme"
            @click="programmeDialog?.open(concert)"
          />
        </template>
      </UCard>
    </div>
  </section>

  <ConcertProgrammeDialog
    ref="programmeDialog"
    :pieces="semesterPieces"
    @saved="onProgrammeSaved"
  />

  <UModal
    v-model:open="concertDialogShown"
    :title="saveConcertId === null ? 'Nouveau concert' : 'Modifier le concert'"
    :description="semester.long_name"
  >
    <template #body>
      <form :id="formId" class="flex flex-col gap-4" @submit.prevent="submit">
        <UFormField label="Date" :error="errors.date">
          <DateInput
            v-model="date"
            :min="semester.start_date"
            :max="semester.end_date"
          />
        </UFormField>
        <UFormField label="Lieu" :error="errors.location_id">
          <USelectMenu
            v-model="location_id"
            :items="locationItems"
            value-key="value"
            :search-input="{ placeholder: 'Rechercher…' }"
            class="w-full"
          >
            <template #empty>
              Aucun lieu.
              <ULink to="/locations" class="text-primary">
                Gérer les lieux
              </ULink>
            </template>
          </USelectMenu>
        </UFormField>

        <UFormField
          label="Rendez-vous des musiciens"
          :error="errors.meeting_at"
        >
          <TimeInput v-model="meeting_at" clearable />
        </UFormField>
        <UFormField label="Ouverture des portes" :error="errors.doors_open_at">
          <TimeInput v-model="doors_open_at" clearable />
        </UFormField>
        <UFormField label="Début" :error="errors.starts_at">
          <TimeInput v-model="starts_at" />
        </UFormField>
        <UFormField label="Lien billetterie" :error="errors.tickets_url">
          <UInput
            v-model="tickets_url"
            v-bind="tickets_urlProps"
            type="url"
            class="w-full"
          />
        </UFormField>
        <UFormField label="Lien vidéo" :error="errors.video_url">
          <UInput
            v-model="video_url"
            v-bind="video_urlProps"
            type="url"
            class="w-full"
          />
        </UFormField>
        <UFormField :error="errors.public">
          <USwitch v-model="isPublic" v-bind="isPublicProps" label="Public" />
        </UFormField>
        <p v-if="saveConcertError" class="text-sm text-error">
          {{ saveConcertError }}
        </p>
      </form>
    </template>

    <template #footer>
      <div class="flex w-full items-center gap-2">
        <UButton
          v-if="saveConcertId !== null"
          label="Supprimer"
          icon="i-lucide-trash-2"
          color="error"
          variant="ghost"
          @click="deleteEditedConcert()"
        />
        <UButton
          class="ms-auto"
          label="Annuler"
          color="neutral"
          variant="ghost"
          @click="concertDialogShown = false"
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
