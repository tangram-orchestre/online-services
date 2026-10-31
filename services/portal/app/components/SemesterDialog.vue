<script setup lang="ts">
import { add } from "date-fns";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import { zNewSemester } from "#hey-api/zod.gen";
import type { Semester } from "#hey-api/types.gen";
import { toast } from "vuetify-sonner";

const props = defineProps<{
  semesters: Semester[];
}>();

const emit = defineEmits<{
  saved: [semester: Semester];
}>();

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
const [start_date, start_dateProps] = defineField("start_date");
const [end_date, end_dateProps] = defineField("end_date");

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
    toast.success("Semestre enregistré avec succès");
    emit("saved", semester);
  });
});
</script>

<template>
  <v-dialog v-model="shown" max-width="450" :modal="true">
    <v-form @submit.prevent="submit">
      <v-card
        :title="editedId === null ? 'Nouveau semestre' : 'Modifier semestre'"
      >
        <v-card-text>
          <v-text-field
            v-model="name"
            v-bind="nameProps"
            label="Nom"
            :error-messages="errors.name"
            autofocus
          />
          <v-date-input
            v-model="start_date"
            v-bind="start_dateProps"
            label="Début"
            :error-messages="errors.start_date"
            prepend-icon=""
          />
          <v-date-input
            v-model="end_date"
            v-bind="end_dateProps"
            label="Fin"
            :error-messages="errors.end_date"
            prepend-icon=""
          />
          <div v-if="saveError" class="text-error">
            {{ saveError }}
          </div>
        </v-card-text>

        <v-card-actions>
          <v-btn @click="() => (shown = false)">Annuler</v-btn>
          <v-btn :loading="isSubmitting" type="submit">Enregistrer</v-btn>
        </v-card-actions>
      </v-card>
    </v-form>
  </v-dialog>
</template>
