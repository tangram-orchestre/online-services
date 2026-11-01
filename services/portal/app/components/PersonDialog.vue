<script setup lang="ts">
import { useForm } from "vee-validate";
import { z } from "zod";
import { toTypedSchema } from "@vee-validate/zod";
import type { Person } from "#hey-api/types.gen";

const emit = defineEmits<{
  /** After a creation or an update. */
  saved: [person: Person];
  deleted: [personId: number];
}>();

const toast = useToast();
const formId = useId();

const { allPeople, refresh } = usePeople();

const zName = (max: number) =>
  z
    .string({ required_error: "Champ requis" })
    .trim()
    .min(1, "Champ requis")
    .max(max, `${max} caractères maximum`);

const zPersonForm = z.object({
  first_name: zName(64),
  last_name: zName(64),
  display_name: z.string().trim().max(128, "128 caractères maximum").optional(),
  anonymous: z.boolean(),
});

const { handleSubmit, resetForm, defineField, errors, isSubmitting } = useForm({
  validationSchema: toTypedSchema(zPersonForm),
});

// A required field must not show its error when the dialog is closed (the
// blur would move the buttons under the cursor).
const noBlurValidation = { validateOnBlur: false };
const [firstName, firstNameProps] = defineField("first_name", noBlurValidation);
const [lastName, lastNameProps] = defineField("last_name", noBlurValidation);
const [displayName, displayNameProps] = defineField("display_name");
const [anonymous] = defineField("anonymous");

const shown = ref(false);
const editedId = ref<number | null>(null);
const saveError = ref<string | null>(null);

/**
 * Open the dialog to edit `person`, or to create a new one if omitted,
 * with the names of `prefill`.
 */
const open = (
  person?: Person,
  prefill?: { first_name: string; last_name: string },
) => {
  saveError.value = null;
  editedId.value = person?.id ?? null;
  resetForm({
    values: {
      first_name: person?.first_name ?? prefill?.first_name ?? "",
      last_name: person?.last_name ?? prefill?.last_name ?? "",
      display_name: person?.display_name ?? undefined,
      anonymous: person?.anonymous ?? false,
    },
  });
  shown.value = true;
};

/** « Jeanne Marie Moreau » → Jeanne / Marie Moreau. */
const splitName = (text: string) => {
  const [first = "", ...rest] = text.trim().split(/\s+/);
  return { first_name: first, last_name: rest.join(" ") };
};

/** Open the dialog to create a person named `text`. */
const create = (text: string) => open(undefined, splitName(text));

const submit = handleSubmit((values) => {
  const onResponseError = onResponseErrorHandler(
    "personne",
    true,
    (errorMsg) => {
      saveError.value = errorMsg;
    },
  );

  const newValue = {
    first_name: values.first_name,
    last_name: values.last_name,
    // Someone anonymous has no alias: the alias typed before is dropped.
    display_name: (!values.anonymous && values.display_name) || undefined,
    anonymous: values.anonymous,
  };

  const id = editedId.value;
  const promise: Promise<Person> =
    id !== null
      ? putPerson({
          composable: "$fetch",
          body: { id, ...newValue },
          onResponseError,
        }).then(() => ({ id, ...newValue }))
      : postPerson({
          composable: "$fetch",
          body: newValue,
          onResponseError,
        });

  return promise.then((saved) => {
    if (allPeople.value) {
      allPeople.value =
        id !== null
          ? allPeople.value.map((p) => (p.id === id ? saved : p))
          : [...allPeople.value, saved];
    }
    shown.value = false;
    toast.add({ title: "Personne enregistrée avec succès", color: "success" });
    emit("saved", saved);
  });
});

/** Ask for confirmation, then delete `person`. */
const remove = (person: Person) => {
  confirmDialog(
    "Confirmer la suppression",
    `Êtes-vous sûr de vouloir supprimer « ${personName(person)} » ?`,
  ).then((confirmed) => {
    if (!confirmed) return;
    deletePersonByPersonId({
      composable: "$fetch",
      path: { person_id: person.id },
      onResponseError: onResponseErrorHandler(
        "personne",
        true,
        (errorMsg) => {
          toast.add({ title: errorMsg, color: "error" });
          refresh();
        },
        "Cette personne est créditée sur des morceaux : retirez-la d’abord de leurs crédits.",
      ),
    }).then(() => {
      if (allPeople.value) {
        allPeople.value = allPeople.value.filter((p) => p.id !== person.id);
      }
      shown.value = false;
      toast.add({ title: "Personne supprimée avec succès", color: "success" });
      emit("deleted", person.id);
    });
  });
};

defineExpose({ open, create, remove });

const removeEdited = () => {
  const person = allPeople.value?.find((p) => p.id === editedId.value);
  if (person) remove(person);
};
</script>

<template>
  <UModal
    v-model:open="shown"
    :title="editedId === null ? 'Nouvelle personne' : 'Modifier la personne'"
    description="Membre de l’orchestre crédité sur des morceaux, avec ou sans compte."
  >
    <template #body>
      <form :id="formId" class="flex flex-col gap-4" @submit.prevent="submit">
        <div class="grid gap-4 sm:grid-cols-2">
          <UFormField label="Prénom" :error="errors.first_name">
            <UInput
              v-model="firstName"
              v-bind="firstNameProps"
              autofocus
              class="w-full"
            />
          </UFormField>
          <UFormField label="Nom" :error="errors.last_name">
            <UInput v-model="lastName" v-bind="lastNameProps" class="w-full" />
          </UFormField>
        </div>
        <UFormField
          :error="errors.anonymous"
          help="Son nom n’apparaît sur aucun support public de Tangram."
        >
          <USwitch v-model="anonymous" label="Anonyme" />
        </UFormField>
        <!-- Someone anonymous is not named at all, not even by an alias. -->
        <UFormField
          v-if="!anonymous"
          label="Alias"
          hint="Facultatif"
          help="Utilisé à la place du prénom et du nom sur les supports publics de Tangram."
          :error="errors.display_name"
        >
          <UInput
            v-model="displayName"
            v-bind="displayNameProps"
            class="w-full"
          />
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
