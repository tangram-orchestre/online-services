<script setup lang="ts">
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import { zNewConcert } from "#hey-api/zod.gen";
import type { Concert, Semester } from "#hey-api/types.gen";
import { toast } from "vuetify-sonner";

const props = defineProps<{
  semester: Semester;
}>();

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
      title: `${l.name} — ${l.city}`,
      value: l.id,
    })) ?? [],
);

const location = (id: number) => locations.value?.find((l) => l.id === id);

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

const [date, dateProps] = defineField("date");
const [starts_at] = defineField("starts_at");
const [doors_open_at] = defineField("doors_open_at");
const [meeting_at] = defineField("meeting_at");
const [tickets_url, tickets_urlProps] = defineField("tickets_url");
const [video_url, video_urlProps] = defineField("video_url");
const [isPublic, isPublicProps] = defineField("public");
const [location_id, location_idProps] = defineField("location_id");

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
    toast.success("Concert enregistré avec succès");
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

const editConcert = (concert: Concert) => {
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
            toast.error(errorMsg);
            refresh();
          },
        ),
      }).then(() => {
        if (allConcerts.value) {
          allConcerts.value = allConcerts.value.filter((c) => c.id !== id);
        }
        concertDialogShown.value = false;
        toast.success("Concert supprimé avec succès");
      });
    }
  });
};
</script>

<template>
  <section>
    <div class="d-flex align-center mb-3">
      <h2 class="text-h6 d-flex align-center">
        <v-icon color="medium-emphasis" icon="mdi-music" size="small" start />
        Concerts
      </h2>
      <v-btn
        class="ms-1"
        icon="mdi-plus"
        size="small"
        variant="text"
        color="medium-emphasis"
        title="Nouveau concert"
        @click="newConcert()"
      />
    </div>

    <v-progress-linear
      v-if="status == 'pending' && concerts.length === 0"
      indeterminate
    />
    <div
      v-else-if="concerts.length === 0"
      class="text-body-2 text-medium-emphasis py-4 text-center"
    >
      Aucun concert pour ce semestre
    </div>
    <v-row v-else dense>
      <v-col
        v-for="concert in concerts"
        :key="concert.id"
        cols="12"
        sm="6"
        md="4"
        lg="3"
      >
        <v-card border flat class="h-100 d-flex flex-column">
          <v-btn
            class="edit-button"
            icon="mdi-pencil"
            size="small"
            variant="text"
            color="medium-emphasis"
            title="Modifier le concert"
            @click="editConcert(concert)"
          />
          <v-card-item class="pe-12">
            <v-card-title class="text-capitalize">
              {{ formatLongDate(concert.date) }}
            </v-card-title>
            <v-card-subtitle>
              <v-icon icon="mdi-map-marker" size="x-small" start />
              {{ location(concert.location_id)?.name ?? "Lieu inconnu" }}
              <template v-if="location(concert.location_id)">
                · {{ location(concert.location_id)?.city }}
              </template>
            </v-card-subtitle>
          </v-card-item>

          <v-card-text class="flex-grow-1">
            <div class="d-flex flex-column ga-1">
              <div v-if="concert.meeting_at" class="d-flex align-center">
                <v-icon
                  icon="mdi-account-group"
                  size="small"
                  class="me-2"
                  color="medium-emphasis"
                />
                Rendez-vous
                <span class="ms-auto font-weight-medium">
                  {{ formatTime(concert.meeting_at) }}
                </span>
              </div>
              <div v-if="concert.doors_open_at" class="d-flex align-center">
                <v-icon
                  icon="mdi-door-open"
                  size="small"
                  class="me-2"
                  color="medium-emphasis"
                />
                Ouverture des portes
                <span class="ms-auto font-weight-medium">
                  {{ formatTime(concert.doors_open_at) }}
                </span>
              </div>
              <div class="d-flex align-center">
                <v-icon
                  icon="mdi-music"
                  size="small"
                  class="me-2"
                  color="medium-emphasis"
                />
                Début
                <span class="ms-auto font-weight-medium">
                  {{ formatTime(concert.starts_at) }}
                </span>
              </div>
            </div>
          </v-card-text>

          <v-card-actions>
            <v-chip
              :color="concert.public ? 'success' : undefined"
              :prepend-icon="concert.public ? 'mdi-earth' : 'mdi-lock'"
              size="small"
              variant="flat"
              :text="concert.public ? 'Public' : 'Privé'"
            />
            <v-btn
              v-if="concert.tickets_url"
              :href="concert.tickets_url"
              target="_blank"
              icon="mdi-ticket"
              size="small"
              title="Billetterie"
            />
            <v-btn
              v-if="concert.video_url"
              :href="concert.video_url"
              target="_blank"
              icon="mdi-play-circle"
              size="small"
              title="Vidéo"
            />
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>
  </section>

  <v-dialog
    v-model="concertDialogShown"
    max-width="500"
    :modal="true"
    scrollable
  >
    <!-- The card is the form so that `scrollable` keeps the actions visible -->
    <v-card
      tag="form"
      :title="saveConcertId === null ? 'Nouveau concert' : 'Modifier concert'"
      :subtitle="`Semestre ${semester.name}`"
      @submit.prevent="submit"
    >
      <v-card-text>
        <v-date-input
          v-model="date"
          v-bind="dateProps"
          label="Date"
          :min="semester.start_date"
          :max="semester.end_date"
          :error-messages="errors.date"
          prepend-icon=""
        />
        <v-select
          v-model="location_id"
          v-bind="location_idProps"
          label="Lieu"
          :items="locationItems"
          :error-messages="errors.location_id"
        >
          <template #no-data>
            <v-list-item>
              Aucun lieu.
              <nuxt-link to="/locations">Gérer les lieux</nuxt-link>
            </v-list-item>
          </template>
        </v-select>

        <v-text-field
          :model-value="meeting_at ? formatTime(meeting_at) : undefined"
          label="Rendez-vous des musiciens"
          :error-messages="errors.meeting_at"
          :append-inner-icon="meeting_at ? 'mdi-close-circle' : undefined"
          readonly
          @click:append-inner.stop="meeting_at = undefined"
        >
          <v-menu
            :close-on-content-click="false"
            activator="parent"
            min-width="0"
          >
            <v-time-picker v-model="meeting_at" format="24hr" />
          </v-menu>
        </v-text-field>
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
        <v-btn
          v-if="saveConcertId !== null"
          color="error"
          prepend-icon="mdi-delete"
          text="Supprimer"
          @click="deleteEditedConcert()"
        />
        <v-spacer />
        <v-btn @click="() => (concertDialogShown = false)">Annuler</v-btn>
        <v-btn :loading="isSubmitting" type="submit">Enregistrer</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<style scoped>
.edit-button {
  position: absolute;
  top: 4px;
  right: 4px;
}
</style>
