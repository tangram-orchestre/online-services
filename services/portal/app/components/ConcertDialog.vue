<script setup lang="ts">
import { useForm } from "vee-validate";
import { z } from "zod";
import { toTypedSchema } from "@vee-validate/zod";
import { zNewConcert } from "#hey-api/zod.gen";
import type { Concert, ConcertWithPieces, Semester } from "#hey-api/types.gen";

const props = defineProps<{
  semester: Semester;
}>();

const emit = defineEmits<{
  /** After a creation or an update. */
  saved: [concertId: number];
  deleted: [concertId: number];
}>();

const toast = useToast();
const formId = useId();

const normalizeTime = (value: string) =>
  value.length === 5 ? `${value}:00` : value;

const { allConcerts, refreshConcerts } = useProgramme();
const { locations } = useLocations();

/** Every room; rehearsal-only rooms are listed but cannot host concerts. */
const roomItems = computed(() =>
  [...(locations.value ?? [])]
    .sort((a, b) => a.name.localeCompare(b.name, "fr"))
    .flatMap((l) =>
      l.rooms.map((room) => ({
        label: `${l.name} · ${room.name}`,
        description: isRehearsalOnly(room)
          ? "Répétitions uniquement"
          : `${l.city} · ${room.audience_capacity} places`,
        value: room.id,
        disabled: isRehearsalOnly(room),
      })),
    ),
);

/** An optional link: empty, or an http(s) URL. */
const zLink = z
  .string()
  .trim()
  .refine((v) => !v || /^https?:\/\/\S+\.\S+$/.test(v), {
    message: "Lien invalide : il doit commencer par https://",
  })
  .optional();

const zConcertForm = zNewConcert
  .omit({ semester_id: true })
  .extend({
    name: z.string().trim().max(128, "128 caractères maximum").optional(),
    tickets_url: zLink,
    video_url: zLink,
  })
  .refine(
    (obj) =>
      !obj.doors_open_at || !obj.starts_at || obj.doors_open_at < obj.starts_at,
    {
      path: ["doors_open_at"],
      message:
        "L'ouverture des portes doit être avant l'heure de début du concert",
    },
  )
  .refine(
    (obj) =>
      !obj.meeting_at || !obj.starts_at || obj.meeting_at < obj.starts_at,
    {
      path: ["meeting_at"],
      message:
        "Le rendez-vous des musiciens doit être avant l'heure de début du concert",
    },
  );

// A new concert is private, and its room (if any) not booked yet.
const initialValues = { public: false, room_booked: false };

const { handleSubmit, handleReset, defineField, errors, isSubmitting } =
  useForm({
    validationSchema: toTypedSchema(zConcertForm),
    initialValues,
  });

const [name, nameProps] = defineField("name");
const [date] = defineField("date");
const [starts_at] = defineField("starts_at");
const [doors_open_at] = defineField("doors_open_at");
const [meeting_at] = defineField("meeting_at");
const [tickets_url, tickets_urlProps] = defineField("tickets_url");
const [video_url, video_urlProps] = defineField("video_url");
const [isPublic, isPublicProps] = defineField("public");
const [room_id] = defineField("room_id");
const [room_booked] = defineField("room_booked");

// Without a room, there is no booking to confirm.
watch(room_id, (id) => {
  if (id == null) room_booked.value = false;
});

const shown = ref(false);
const editedId = ref<number | null>(null);
const saveError = ref<string | null>(null);

/** Open the dialog to edit `concert`, or to create a new one if omitted. */
const open = (concert?: Concert | ConcertWithPieces) => {
  saveError.value = null;
  if (concert) {
    name.value = concert.name ?? undefined;
    date.value = concert.date;
    starts_at.value = concert.starts_at ?? undefined;
    doors_open_at.value = concert.doors_open_at ?? undefined;
    meeting_at.value = concert.meeting_at ?? undefined;
    tickets_url.value = concert.tickets_url ?? undefined;
    video_url.value = concert.video_url ?? undefined;
    isPublic.value = concert.public;
    room_id.value = concert.room_id ?? undefined;
    room_booked.value = concert.room_booked;
    editedId.value = concert.id;
  } else {
    handleReset();
    // Reset to the initial values, not to those of the last edited concert.
    isPublic.value = initialValues.public;
    room_booked.value = initialValues.room_booked;
    editedId.value = null;
  }
  shown.value = true;
};

const save = (id: number | null) => {
  const onResponseError = onResponseErrorHandler(
    "concert",
    false,
    (errorMsg) => {
      saveError.value = errorMsg;
    },
  );

  if (!date.value) return;

  const newValue = {
    name: name.value?.trim() || undefined,
    date: date.value,
    starts_at: starts_at.value ? normalizeTime(starts_at.value) : undefined,
    doors_open_at: doors_open_at.value
      ? normalizeTime(doors_open_at.value)
      : undefined,
    meeting_at: meeting_at.value ? normalizeTime(meeting_at.value) : undefined,
    tickets_url: tickets_url.value || undefined,
    video_url: video_url.value || undefined,
    public: isPublic.value ?? false,
    room_id: room_id.value ?? undefined,
    room_booked: room_id.value != null && (room_booked.value ?? false),
    semester_id: props.semester.id,
  };

  const promise: Promise<number> =
    id !== null
      ? putConcert({
          composable: "$fetch",
          body: { id, ...newValue },
          onResponseError,
        }).then(() => {
          if (allConcerts.value) {
            allConcerts.value = allConcerts.value.map((c) =>
              c.id === id ? { ...c, ...newValue } : c,
            );
          }
          return id;
        })
      : postConcert({
          composable: "$fetch",
          body: newValue,
          onResponseError,
        }).then((created) => {
          if (allConcerts.value) {
            allConcerts.value = [...allConcerts.value, created];
          }
          return created.id;
        });

  return promise.then((savedId) => {
    shown.value = false;
    toast.add({ title: "Concert enregistré avec succès", color: "success" });
    emit("saved", savedId);
  });
};

const submit = handleSubmit(() => save(editedId.value));

/** Ask for confirmation, then delete `concert`. */
const remove = (concert: Concert | ConcertWithPieces) => {
  confirmDialog(
    "Confirmer la suppression",
    `Êtes-vous sûr de vouloir supprimer « ${concertTitle(concert)} » ?`,
  ).then((confirmed) => {
    if (!confirmed) return;
    deleteConcertByConcertId({
      composable: "$fetch",
      path: { concert_id: concert.id },
      onResponseError: onResponseErrorHandler("concert", false, (errorMsg) => {
        toast.add({ title: errorMsg, color: "error" });
        refreshConcerts();
      }),
    }).then(() => {
      if (allConcerts.value) {
        allConcerts.value = allConcerts.value.filter(
          (c) => c.id !== concert.id,
        );
      }
      shown.value = false;
      toast.add({ title: "Concert supprimé avec succès", color: "success" });
      emit("deleted", concert.id);
    });
  });
};

defineExpose({ open, remove });

const removeEdited = () => {
  const concert = allConcerts.value?.find((c) => c.id === editedId.value);
  if (concert) remove(concert);
};
</script>

<template>
  <UModal
    v-model:open="shown"
    :title="editedId === null ? 'Nouveau concert' : 'Modifier le concert'"
    :description="semester.long_name"
  >
    <template #body>
      <form :id="formId" class="flex flex-col gap-4" @submit.prevent="submit">
        <UFormField label="Nom" hint="Facultatif" :error="errors.name">
          <UInput
            v-model="name"
            v-bind="nameProps"
            placeholder="Concert de Noël"
            class="w-full"
          />
        </UFormField>
        <UFormField label="Date" :error="errors.date">
          <DateInput
            v-model="date"
            :min="semester.start_date"
            :max="semester.end_date"
          />
        </UFormField>
        <UFormField label="Salle" :error="errors.room_id">
          <USelectMenu
            v-model="room_id"
            :items="roomItems"
            value-key="value"
            :search-input="{ placeholder: 'Rechercher…' }"
            placeholder="À définir"
            clear
            class="w-full"
          >
            <template #empty>
              Aucune salle.
              <ULink to="/locations" class="text-primary">
                Gérer les lieux
              </ULink>
            </template>
          </USelectMenu>
        </UFormField>
        <UFormField
          :error="errors.room_booked"
          :help="
            room_id == null
              ? undefined
              : 'À cocher une fois la réservation validée par le lieu'
          "
        >
          <USwitch
            v-model="room_booked"
            :disabled="room_id == null"
            label="Réservation confirmée"
          />
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
          <TimeInput v-model="starts_at" clearable />
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
