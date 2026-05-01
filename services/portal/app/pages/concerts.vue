<script setup lang="ts">
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import { zNewConcert } from "#hey-api/zod.gen";
import { toast } from "vuetify-sonner";

const formatDate = (value: string) => new Date(value).toLocaleDateString("fr");
const formatTime = (value: string) => value.slice(0, 5);
const normalizeTime = (value: string) =>
  value.length === 5 ? `${value}:00` : value;

const {
  data: concerts,
  refresh,
  status,
} = getConcerts({
  composable: "useAsyncData",
  key: "getConcerts",
});

const { data: semesters } = getSemesters({
  composable: "useAsyncData",
  key: "getSemesters",
});

const { data: locations } = getLocations({
  composable: "useAsyncData",
  key: "getLocations",
});

const semesterItems = computed(
  () => semesters.value?.map((s) => ({ title: s.name, value: s.id })) ?? [],
);

const locationItems = computed(
  () =>
    locations.value?.map((l) => ({
      title: `${l.name} — ${l.city}`,
      value: l.id,
    })) ?? [],
);

const semesterName = (id: number) =>
  semesters.value?.find((s) => s.id === id)?.name ?? String(id);

const locationName = (id: number) => {
  const l = locations.value?.find((l) => l.id === id);
  return l ? `${l.name}` : String(id);
};

const zConcert = zNewConcert.refine(
  (obj) => !obj.doors_open_at || obj.doors_open_at < obj.starts_at,
  {
    path: ["doors_open_at"],
    message:
      "L'ouverture des portes doit être avant l'heure de début du concert",
  },
);

const { handleSubmit, handleReset, defineField, errors, isSubmitting } =
  useForm({
    validationSchema: toTypedSchema(zConcert),
  });

const [date, dateProps] = defineField("date");
const [starts_at] = defineField("starts_at");
const [doors_open_at] = defineField("doors_open_at");
const [tickets_url, tickets_urlProps] = defineField("tickets_url");
const [video_url, video_urlProps] = defineField("video_url");
const [isPublic, isPublicProps] = defineField("public");
const [location_id, location_idProps] = defineField("location_id");
const [semester_id, semester_idProps] = defineField("semester_id");

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

  if (
    !date.value ||
    !starts_at.value ||
    !location_id.value ||
    !semester_id.value
  ) {
    return;
  }

  const newValue = {
    date: date.value,
    starts_at: normalizeTime(starts_at.value),
    doors_open_at: doors_open_at.value
      ? normalizeTime(doors_open_at.value)
      : undefined,
    tickets_url: tickets_url.value || undefined,
    video_url: video_url.value || undefined,
    public: isPublic.value ?? false,
    location_id: location_id.value,
    semester_id: semester_id.value,
  };

  let promise;

  if (id !== null) {
    promise = putConcert({
      composable: "$fetch",
      body: { id, ...newValue },
      onResponseError,
    }).then(() => {
      if (concerts.value) {
        concerts.value = concerts.value.map((c) =>
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
      if (concerts.value) {
        concerts.value = [...concerts.value, created];
      }
    });
  }

  promise.then(() => {
    concertDialogShown.value = false;
    toast.success("Concert enregistré avec succès");
  });
};

const submit = handleSubmit(() => {
  saveConcert(saveConcertId.value);
});

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
            toast.error(errorMsg);
            refresh();
          },
        ),
      }).then(() => {
        if (concerts.value) {
          concerts.value = concerts.value.filter((c) => c.id !== id);
        }
        toast.success("Concert supprimé avec succès");
      });
    }
  });
};
</script>

<template>
  <v-sheet border rounded>
    <v-data-table
      v-if="concerts"
      :loading="status == 'pending'"
      :items="concerts"
      :headers="[
        { title: 'Date', key: 'date' },
        { title: 'Semestre', key: 'semester_id' },
        { title: 'Lieu', key: 'location_id' },
        { title: 'Début', key: 'starts_at' },
        { title: 'Public', key: 'public' },
        { title: 'Actions', value: 'actions', align: 'end' },
      ]"
      :hide-default-footer="concerts.length < 11"
      :sort-by="[{ key: 'date', order: 'desc' }]"
      item-value="id"
    >
      <template #top>
        <v-toolbar flat>
          <v-toolbar-title>
            <v-icon
              color="medium-emphasis"
              icon="mdi-music"
              size="x-small"
              start
            />
            Concerts
          </v-toolbar-title>

          <v-btn
            class="me-2"
            prepend-icon="mdi-plus"
            rounded="lg"
            text="Nouveau concert"
            border
            @click="
              () => {
                handleReset();
                saveConcertId = null;
                concertDialogShown = true;
              }
            "
          />
        </v-toolbar>
      </template>

      <template #item.date="{ item }">
        {{ formatDate(item.date) }}
      </template>
      <template #item.semester_id="{ item }">
        {{ semesterName(item.semester_id) }}
      </template>
      <template #item.location_id="{ item }">
        {{ locationName(item.location_id) }}
      </template>
      <template #item.starts_at="{ item }">
        {{ formatTime(item.starts_at) }}
      </template>
      <template #item.public="{ item }">
        <v-icon
          :icon="item.public ? 'mdi-check-circle' : 'mdi-close-circle'"
          :color="item.public ? 'success' : 'medium-emphasis'"
          size="small"
        />
      </template>

      <template #item.actions="{ item }">
        <div class="d-flex ga-2 justify-end">
          <v-icon
            color="medium-emphasis"
            icon="mdi-pencil"
            size="small"
            @click="
              () => {
                date = item.date;
                starts_at = item.starts_at;
                doors_open_at = item.doors_open_at ?? undefined;
                tickets_url = item.tickets_url ?? undefined;
                video_url = item.video_url ?? undefined;
                isPublic = item.public;
                location_id = item.location_id;
                semester_id = item.semester_id;
                saveConcertId = item.id;
                concertDialogShown = true;
              }
            "
          />
          <v-icon
            color="medium-emphasis"
            icon="mdi-delete"
            size="small"
            @click="deleteConcert(item.id, item.date)"
          />
        </div>
      </template>
    </v-data-table>
  </v-sheet>

  <v-dialog v-model="concertDialogShown" max-width="500" :modal="true">
    <v-form @submit.prevent="submit">
      <v-card
        :title="saveConcertId === null ? 'Nouveau concert' : 'Modifier concert'"
      >
        <v-card-text>
          <v-date-input
            v-model="date"
            v-bind="dateProps"
            label="Date"
            :error-messages="errors.date"
            prepend-icon=""
          />
          <v-select
            v-model="semester_id"
            v-bind="semester_idProps"
            label="Semestre"
            :items="semesterItems"
            :error-messages="errors.semester_id"
          />
          <v-select
            v-model="location_id"
            v-bind="location_idProps"
            label="Lieu"
            :items="locationItems"
            :error-messages="errors.location_id"
          />

          <v-text-field
            :model-value="doors_open_at ? formatTime(doors_open_at) : undefined"
            label="Ouverture des portes"
            :error-messages="errors.doors_open_at"
            :append-inner-icon="doors_open_at ? 'mdi-close-circle' : undefined"
            readonly
            @click:append-inner.stop="doors_open_at = undefined"
          >
            <v-menu
              :close-on-content-click="false"
              activator="parent"
              min-width="0"
            >
              <v-time-picker v-model="doors_open_at" format="24hr" />
            </v-menu>
          </v-text-field>
          <v-text-field
            :model-value="starts_at ? formatTime(starts_at) : undefined"
            label="Début"
            :error-messages="errors.starts_at"
            readonly
          >
            <v-menu
              :close-on-content-click="false"
              activator="parent"
              min-width="0"
            >
              <v-time-picker v-model="starts_at" format="24hr" />
            </v-menu>
          </v-text-field>
          <v-text-field
            v-model="tickets_url"
            v-bind="tickets_urlProps"
            label="Lien billetterie"
            :error-messages="errors.tickets_url"
            clearable
          />
          <v-text-field
            v-model="video_url"
            v-bind="video_urlProps"
            label="Lien vidéo"
            :error-messages="errors.video_url"
            clearable
          />
          <v-switch
            v-model="isPublic"
            v-bind="isPublicProps"
            label="Public"
            color="primary"
            :error-messages="errors.public"
          />
          <div v-if="saveConcertError" class="text-error">
            {{ saveConcertError }}
          </div>
        </v-card-text>

        <v-card-actions>
          <v-btn @click="() => (concertDialogShown = false)">Annuler</v-btn>
          <v-btn :loading="isSubmitting" type="submit">Enregistrer</v-btn>
        </v-card-actions>
      </v-card>
    </v-form>
  </v-dialog>
</template>
