<script setup lang="ts">
import { add } from "date-fns";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import { zNewSemester } from "#hey-api/zod.gen";
import type { Semester } from "#hey-api/types.gen";

const props = defineProps<{
  semesters: Semester[];
}>();

const emit = defineEmits<{
  saved: [semester: Semester];
}>();

const toast = useToast();
const formId = useId();

const serializeDate = (date: Date) => date.toISOString().split("T")[0];

const zSemesterForm = zNewSemester.refine(
  (obj) => obj.start_date < obj.end_date,
  {
    path: ["end_date"],
    message: "La date de fin doit être postérieure à la date de début",
  },
);

const { handleSubmit, resetForm, defineField, errors, isSubmitting } = useForm({
  validationSchema: toTypedSchema(zSemesterForm),
});

const [name, nameProps] = defineField("name");
const [start_date] = defineField("start_date");
const [end_date] = defineField("end_date");

const shown = ref(false);
const editedId = ref<number | null>(null);
const saveError = ref<string | null>(null);

/** Open the dialog to edit `semester`, or to create a new one if omitted. */
const open = (semester?: Semester) => {
  saveError.value = null;
  if (semester) {
    editedId.value = semester.id;
    resetForm({
      values: {
        name: semester.name,
        start_date: semester.start_date,
        end_date: semester.end_date,
      },
    });
  } else {
    // Default to a 6 months semester starting after the latest one.
    const today = new Date();
    const defaultStartDate = props.semesters.reduce((latest, semester) => {
      const semesterEnd = new Date(semester.end_date);
      return semesterEnd > latest ? semesterEnd : latest;
    }, today);
    editedId.value = null;
    resetForm({
      values: {
        name: "",
        start_date: serializeDate(defaultStartDate),
        end_date: serializeDate(add(defaultStartDate, { months: 6 })),
      },
    });
  }
  shown.value = true;
};

defineExpose({ open });

const submit = handleSubmit((values) => {
  const onResponseError = onResponseErrorHandler(
    "semestre",
    false,
    (errorMsg) => {
      saveError.value = errorMsg;
    },
  );

  const id = editedId.value;
  const promise: Promise<Semester> =
    id !== null
      ? putSemester({
          composable: "$fetch",
          body: { id, ...values },
          onResponseError,
        }).then(() => ({ id, ...values }))
      : postSemester({
          composable: "$fetch",
          body: values,
          onResponseError,
        });

  return promise.then((semester) => {
    shown.value = false;
    toast.add({ title: "Semestre enregistré avec succès", color: "success" });
    emit("saved", semester);
  });
});
</script>

<template>
  <UModal
    v-model:open="shown"
    :title="editedId === null ? 'Nouveau semestre' : 'Modifier le semestre'"
  >
    <template #body>
      <form :id="formId" class="flex flex-col gap-4" @submit.prevent="submit">
        <UFormField label="Nom" :error="errors.name">
          <UInput v-model="name" v-bind="nameProps" autofocus class="w-full" />
        </UFormField>
        <UFormField label="Début" :error="errors.start_date">
          <DateInput v-model="start_date" />
        </UFormField>
        <UFormField label="Fin" :error="errors.end_date">
          <DateInput v-model="end_date" />
        </UFormField>
        <p v-if="saveError" class="text-sm text-error">{{ saveError }}</p>
      </form>
    </template>

    <template #footer>
      <div class="flex w-full justify-end gap-2">
        <UButton
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
