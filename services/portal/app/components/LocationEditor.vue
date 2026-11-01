<script setup lang="ts">
import { z } from "zod";
import type { LocationWithDetails } from "#hey-api/types.gen";

const props = defineProps<{
  /** Location to edit, or undefined to create a new one. */
  location?: LocationWithDetails;
  /** Concerts count by room id (all semesters): such rooms can't be removed. */
  roomConcerts: Map<number, number>;
}>();

const emit = defineEmits<{
  saved: [location: LocationWithDetails];
  cancel: [];
}>();

const toast = useToast();
const formId = useId();

/** Value of a number input: a number, or "" when empty or not a number. */
type NumberInput = number | "";

type RoomForm = {
  id?: number;
  name: string;
  /** No audience: audience and total capacities are cleared. */
  rehearsal_only: boolean;
  musician_capacity: NumberInput;
  audience_capacity: NumberInput;
  total_capacity: NumberInput;
};

type ContactForm = {
  name: string;
  role: string;
  email: string;
  phone_number: string;
};

type Form = {
  name: string;
  street: string;
  zipcode: string;
  city: string;
  notes: string;
  rooms: RoomForm[];
  contacts: ContactForm[];
};

const concertsIn = (room: RoomForm) =>
  room.id === undefined ? 0 : (props.roomConcerts.get(room.id) ?? 0);

const capacity = z.preprocess(
  (v) => (v === "" || v === undefined ? null : v),
  z
    .number({ invalid_type_error: "Nombre attendu" })
    .int("Nombre entier attendu")
    .min(1, "Doit être supérieure à 0")
    .nullable(),
);

const zRoomForm = z
  .object({
    id: z.number().optional(),
    name: z.string().trim().min(1, "Nom requis").max(128),
    rehearsal_only: z.boolean(),
    musician_capacity: capacity,
    audience_capacity: capacity,
    total_capacity: capacity,
  })
  .superRefine((room, ctx) => {
    const issue = (field: keyof RoomForm, message: string) =>
      ctx.addIssue({ code: z.ZodIssueCode.custom, path: [field], message });

    if (room.rehearsal_only) {
      if (room.id !== undefined && props.roomConcerts.get(room.id)) {
        issue(
          "rehearsal_only",
          "Cette salle accueille des concerts : déplacez-les dans une autre salle d’abord.",
        );
      }
      return;
    }
    if (room.audience_capacity == null) {
      issue("audience_capacity", "Requise pour accueillir des concerts");
    }
    const total = room.total_capacity;
    if (total != null) {
      if (room.musician_capacity != null && total < room.musician_capacity) {
        issue("total_capacity", "Inférieure à la capacité musiciens");
      } else if (
        room.audience_capacity != null &&
        total < room.audience_capacity
      ) {
        issue("total_capacity", "Inférieure à la capacité public");
      }
    }
  });

const zForm = z.object({
  name: z.string().trim().min(1, "Nom requis").max(128),
  street: z.string().trim().min(1, "Adresse requise").max(256),
  zipcode: z.string().trim().min(1, "Code postal requis").max(16),
  city: z.string().trim().min(1, "Ville requise").max(128),
  notes: z.string().trim(),
  rooms: z
    .array(zRoomForm)
    .min(1, "Ajoutez au moins une salle")
    .superRefine((rooms, ctx) => {
      const seen = new Set<string>();
      rooms.forEach((room, i) => {
        const key = room.name.trim().toLowerCase();
        if (key && seen.has(key)) {
          ctx.addIssue({
            code: z.ZodIssueCode.custom,
            path: [i, "name"],
            message: "Deux salles ont ce nom",
          });
        }
        seen.add(key);
      });
    }),
  contacts: z.array(
    z.object({
      name: z.string().trim().min(1, "Nom requis").max(128),
      role: z.string().trim().max(128),
      email: z
        .string()
        .trim()
        .max(254)
        .refine((v) => !v || z.string().email().safeParse(v).success, {
          message: "Adresse e-mail invalide",
        }),
      phone_number: z.string().trim().max(32),
    }),
  ),
});

const emptyRoom = (name = ""): RoomForm => ({
  name,
  rehearsal_only: false,
  musician_capacity: "",
  audience_capacity: "",
  total_capacity: "",
});

const initialForm = (): Form => {
  const l = props.location;
  return {
    name: l?.name ?? "",
    street: l?.street ?? "",
    zipcode: l?.zipcode ?? "",
    city: l?.city ?? "",
    notes: l?.notes ?? "",
    rooms: l
      ? l.rooms.map((r) => ({
          id: r.id,
          name: r.name,
          rehearsal_only: r.audience_capacity == null,
          musician_capacity: r.musician_capacity ?? "",
          audience_capacity: r.audience_capacity ?? "",
          total_capacity: r.total_capacity ?? "",
        }))
      : [emptyRoom("Salle principale")],
    contacts:
      l?.contacts.map((c) => ({
        name: c.name,
        role: c.role ?? "",
        email: c.email ?? "",
        phone_number: c.phone_number ?? "",
      })) ?? [],
  };
};

const form = ref<Form>(initialForm());
const saved = ref(JSON.stringify(form.value));
const dirty = computed(() => JSON.stringify(form.value) !== saved.value);

/*
 * Validation runs on every change. An error is shown once its field was
 * changed or left (touched), or after a save attempt, so that a new form
 * isn't covered in red.
 */
const errors = ref<Record<string, string>>({});
const touched = ref(new Set<string>());
const submitted = ref(false);
const saveError = ref<string | null>(null);
const isSubmitting = ref(false);

const validate = () => {
  const result = zForm.safeParse(form.value);
  const next: Record<string, string> = {};
  if (!result.success) {
    for (const issue of result.error.issues) {
      next[issue.path.join(".")] ??= issue.message;
    }
  }
  errors.value = next;
  return result;
};

/** Error to display for a field path such as `rooms.0.name`. */
const errorOf = (path: string) =>
  submitted.value || touched.value.has(path) ? errors.value[path] : undefined;

/** Leaf values of the form, by path. */
const flatten = (
  value: unknown,
  prefix = "",
  out = new Map<string, unknown>(),
) => {
  if (value && typeof value === "object") {
    for (const [key, v] of Object.entries(value)) {
      flatten(v, prefix ? `${prefix}.${key}` : key, out);
    }
  } else {
    out.set(prefix, value);
  }
  return out;
};

let previous = flatten(form.value);
watch(
  form,
  (now) => {
    const current = flatten(now);
    // Paths that didn't exist (a row just added) aren't touched yet.
    for (const [path, value] of current) {
      if (previous.has(path) && previous.get(path) !== value) {
        touched.value.add(path);
      }
    }
    previous = current;
    validate();
  },
  { deep: true },
);

// Leaving a field touches it too, so that an empty required field is flagged.
const onFocusOut = (event: FocusEvent) => {
  const path = (event.target as HTMLElement | null)?.closest<HTMLElement>(
    "[data-path]",
  )?.dataset.path;
  if (path) touched.value.add(path);
};

validate();

/** Removing a row shifts the following ones: forget their touched state. */
const untouchRows = (prefix: "rooms" | "contacts") => {
  touched.value = new Set(
    [...touched.value].filter((p) => !p.startsWith(`${prefix}.`)),
  );
  previous = flatten(form.value);
};

const addRoom = () => form.value.rooms.push(emptyRoom());
const removeRoom = (index: number) => {
  form.value.rooms.splice(index, 1);
  untouchRows("rooms");
};
const addContact = () =>
  form.value.contacts.push({
    name: "",
    role: "",
    email: "",
    phone_number: "",
  });
const removeContact = (index: number) => {
  form.value.contacts.splice(index, 1);
  untouchRows("contacts");
};

/**
 * « Répétitions uniquement » resets the audience and total capacities either
 * way: hidden while ticked, empty when unticked.
 */
const setRehearsalOnly = (index: number, value: boolean) => {
  const room = form.value.rooms[index]!;
  room.audience_capacity = "";
  room.total_capacity = "";
  room.rehearsal_only = value;
  // Tell right away what the new state requires: an audience for concerts,
  // or no concerts in a rehearsal-only room.
  touched.value.add(
    `rooms.${index}.${value ? "rehearsal_only" : "audience_capacity"}`,
  );
};

/** A total capacity that musicians and audience can't reach caps nothing. */
const uselessTotal = (room: RoomForm) =>
  typeof room.total_capacity === "number" &&
  typeof room.musician_capacity === "number" &&
  typeof room.audience_capacity === "number" &&
  room.total_capacity >= room.musician_capacity + room.audience_capacity;

const submit = async () => {
  submitted.value = true;
  saveError.value = null;
  const result = validate();
  if (!result.success) return;

  const values = result.data;
  const body = {
    name: values.name,
    street: values.street,
    zipcode: values.zipcode,
    city: values.city,
    notes: values.notes || undefined,
    rooms: values.rooms.map((r) => ({
      id: r.id,
      name: r.name,
      musician_capacity: r.musician_capacity ?? undefined,
      audience_capacity: r.rehearsal_only
        ? undefined
        : (r.audience_capacity ?? undefined),
      total_capacity: r.rehearsal_only
        ? undefined
        : (r.total_capacity ?? undefined),
    })),
    contacts: values.contacts.map((c) => ({
      name: c.name,
      role: c.role || undefined,
      email: c.email || undefined,
      phone_number: c.phone_number || undefined,
    })),
  };
  const onResponseError = onResponseErrorHandler("lieu", false, (errorMsg) => {
    saveError.value = errorMsg;
  });

  isSubmitting.value = true;
  try {
    const location = props.location
      ? await putLocation({
          composable: "$fetch",
          body: { id: props.location.id, ...body },
          onResponseError,
        })
      : await postLocation({ composable: "$fetch", body, onResponseError });
    toast.add({ title: "Lieu enregistré avec succès", color: "success" });
    emit("saved", location);
  } catch {
    // Reported by onResponseError.
  } finally {
    isSubmitting.value = false;
  }
};

const reset = () => {
  form.value = initialForm();
  previous = flatten(form.value);
  touched.value = new Set();
  submitted.value = false;
  saveError.value = null;
  validate();
};

const cancel = () => {
  reset();
  emit("cancel");
};

// Once saved, the parent passes the new location: it becomes the reference.
watch(
  () => props.location,
  () => {
    reset();
    saved.value = JSON.stringify(form.value);
  },
);
</script>

<template>
  <form
    :id="formId"
    class="flex flex-col gap-8 rounded-lg border border-default bg-default p-5 sm:p-6"
    novalidate
    @submit.prevent="submit"
    @focusout="onFocusOut"
  >
    <h2 class="text-3xl leading-tight font-bold text-highlighted">
      {{ location ? form.name || location.name : "Nouveau lieu" }}
    </h2>

    <div class="grid gap-x-6 gap-y-5 sm:grid-cols-2">
      <UFormField
        label="Nom"
        data-path="name"
        :error="errorOf('name')"
        class="sm:col-span-2"
      >
        <UInput
          v-model="form.name"
          size="lg"
          :autofocus="!location"
          class="w-full"
        />
      </UFormField>
      <UFormField
        label="Adresse"
        data-path="street"
        :error="errorOf('street')"
        class="sm:col-span-2"
      >
        <UInput
          v-model="form.street"
          size="lg"
          placeholder="Numéro et rue"
          class="w-full"
        />
      </UFormField>
      <UFormField
        label="Code postal"
        data-path="zipcode"
        :error="errorOf('zipcode')"
      >
        <UInput
          v-model="form.zipcode"
          size="lg"
          inputmode="numeric"
          class="w-full"
        />
      </UFormField>
      <UFormField label="Ville" data-path="city" :error="errorOf('city')">
        <UInput v-model="form.city" size="lg" class="w-full" />
      </UFormField>
    </div>

    <fieldset class="flex flex-col gap-3">
      <legend class="mb-3 font-display text-lg font-semibold">Salles</legend>

      <!-- Container queries: the capacity fields follow the card width,
           not the window's, so numbers never get truncated. -->
      <div
        v-for="(room, i) in form.rooms"
        :key="room.id ?? `new-${i}`"
        class="@container flex flex-col gap-4 rounded-md border border-muted bg-muted p-4"
      >
        <div class="flex items-start gap-2">
          <UFormField
            label="Nom de la salle"
            :data-path="`rooms.${i}.name`"
            :error="errorOf(`rooms.${i}.name`)"
            :help="
              concertsIn(room)
                ? `${concertsIn(room)} ${concertsIn(room) > 1 ? 'concerts' : 'concert'} dans cette salle`
                : undefined
            "
            class="flex-1"
          >
            <UInput v-model="room.name" size="lg" class="w-full" />
          </UFormField>
          <UTooltip
            :text="
              concertsIn(room)
                ? 'Cette salle accueille des concerts'
                : 'Retirer la salle'
            "
          >
            <UButton
              icon="i-lucide-trash-2"
              color="neutral"
              variant="ghost"
              aria-label="Retirer la salle"
              :disabled="concertsIn(room) > 0 || form.rooms.length === 1"
              class="mt-6"
              @click="removeRoom(i)"
            />
          </UTooltip>
        </div>

        <section
          :aria-label="`Capacités de ${room.name || 'la salle'}`"
          class="flex flex-col gap-4 border-t border-default pt-4"
        >
          <div
            class="flex flex-wrap items-center justify-between gap-x-4 gap-y-2"
          >
            <h4 class="font-display font-semibold">Capacités</h4>
            <!-- Disabled controls get no hover events: the wrapper does. -->
            <UTooltip
              text="Cette salle accueille des concerts : elle doit garder un public."
              :disabled="!concertsIn(room) || room.rehearsal_only"
            >
              <span :data-path="`rooms.${i}.rehearsal_only`">
                <UCheckbox
                  :model-value="room.rehearsal_only"
                  label="Répétitions uniquement"
                  :disabled="concertsIn(room) > 0 && !room.rehearsal_only"
                  @update:model-value="setRehearsalOnly(i, $event === true)"
                />
              </span>
            </UTooltip>
          </div>
          <p
            v-if="errorOf(`rooms.${i}.rehearsal_only`)"
            class="-mt-2 text-sm text-error"
          >
            {{ errorOf(`rooms.${i}.rehearsal_only`) }}
          </p>

          <div class="grid gap-4 @lg:grid-cols-3">
            <UFormField
              label="Musiciens"
              :data-path="`rooms.${i}.musician_capacity`"
              :error="errorOf(`rooms.${i}.musician_capacity`)"
              class="min-w-0"
            >
              <UInput
                v-model.number="room.musician_capacity"
                type="number"
                inputmode="numeric"
                min="1"
                step="1"
                size="lg"
                placeholder="Inconnue"
                class="w-full"
              />
            </UFormField>
            <template v-if="!room.rehearsal_only">
              <UFormField
                label="Public"
                :data-path="`rooms.${i}.audience_capacity`"
                :error="errorOf(`rooms.${i}.audience_capacity`)"
                class="min-w-0"
              >
                <UInput
                  v-model.number="room.audience_capacity"
                  type="number"
                  inputmode="numeric"
                  min="1"
                  step="1"
                  size="lg"
                  class="w-full"
                />
              </UFormField>
              <UFormField
                label="Totale"
                hint="Facultatif"
                :data-path="`rooms.${i}.total_capacity`"
                :error="errorOf(`rooms.${i}.total_capacity`)"
                :help="
                  uselessTotal(room)
                    ? 'Ne plafonne rien : musiciens et public tiennent déjà.'
                    : undefined
                "
                class="min-w-0"
              >
                <UInput
                  v-model.number="room.total_capacity"
                  type="number"
                  inputmode="numeric"
                  min="1"
                  step="1"
                  size="lg"
                  placeholder="Non plafonnée"
                  class="w-full"
                />
              </UFormField>
            </template>
            <p v-else class="text-sm text-muted @lg:col-span-2 @lg:self-center">
              Pas de public : cette salle ne peut pas accueillir de concert.
            </p>
          </div>
        </section>
      </div>

      <p v-if="submitted && errors.rooms" class="text-sm text-error">
        {{ errors.rooms }}
      </p>
      <UButton
        label="Ajouter une salle"
        icon="i-lucide-plus"
        color="neutral"
        variant="outline"
        class="self-start"
        @click="addRoom()"
      />
    </fieldset>

    <fieldset class="flex flex-col gap-3">
      <legend class="mb-1 font-display text-lg font-semibold">Contacts</legend>
      <p v-if="!form.contacts.length" class="-mt-2 text-sm text-muted">
        Aucun contact pour ce lieu.
      </p>

      <div
        v-for="(contact, i) in form.contacts"
        :key="i"
        class="flex items-start gap-2 rounded-md border border-muted bg-muted p-4"
      >
        <div class="grid flex-1 gap-x-4 gap-y-4 sm:grid-cols-2">
          <UFormField
            label="Nom"
            :data-path="`contacts.${i}.name`"
            :error="errorOf(`contacts.${i}.name`)"
          >
            <UInput v-model="contact.name" size="lg" class="w-full" />
          </UFormField>
          <UFormField
            label="Rôle"
            hint="Facultatif"
            :data-path="`contacts.${i}.role`"
            :error="errorOf(`contacts.${i}.role`)"
          >
            <UInput
              v-model="contact.role"
              size="lg"
              placeholder="Régisseur, gardien…"
              class="w-full"
            />
          </UFormField>
          <UFormField
            label="E-mail"
            hint="Facultatif"
            :data-path="`contacts.${i}.email`"
            :error="errorOf(`contacts.${i}.email`)"
          >
            <UInput
              v-model="contact.email"
              type="email"
              size="lg"
              class="w-full"
            />
          </UFormField>
          <UFormField
            label="Téléphone"
            hint="Facultatif"
            :data-path="`contacts.${i}.phone_number`"
            :error="errorOf(`contacts.${i}.phone_number`)"
          >
            <UInput
              v-model="contact.phone_number"
              type="tel"
              size="lg"
              class="w-full"
            />
          </UFormField>
        </div>
        <UButton
          icon="i-lucide-trash-2"
          color="neutral"
          variant="ghost"
          aria-label="Retirer le contact"
          class="mt-6"
          @click="removeContact(i)"
        />
      </div>

      <UButton
        label="Ajouter un contact"
        icon="i-lucide-plus"
        color="neutral"
        variant="outline"
        class="self-start"
        @click="addContact()"
      />
    </fieldset>

    <UFormField
      label="Notes pratiques"
      data-path="notes"
      :error="errorOf('notes')"
    >
      <UTextarea
        v-model="form.notes"
        :rows="3"
        autoresize
        size="lg"
        placeholder="Accès, clés, piano, parking…"
        class="w-full"
      />
    </UFormField>

    <p v-if="saveError" class="text-sm text-error">{{ saveError }}</p>

    <div
      v-if="!location || dirty"
      class="sticky bottom-0 -mx-5 -mb-5 flex flex-wrap justify-end gap-2 rounded-b-lg border-t border-default bg-default p-4 sm:-mx-6 sm:-mb-6"
    >
      <UButton
        label="Annuler"
        color="neutral"
        variant="ghost"
        size="lg"
        @click="cancel()"
      />
      <UButton
        label="Enregistrer"
        type="submit"
        size="lg"
        :loading="isSubmitting"
      />
    </div>
  </form>
</template>
