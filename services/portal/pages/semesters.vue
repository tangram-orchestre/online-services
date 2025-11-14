<script setup lang="ts">
import { add } from "date-fns";
import type { GetDummyErrors, BadRequestReason } from "#hey-api/types.gen";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";

import { zNewSemester } from "#hey-api/zod.gen";

const formatDate = (value: Date) => {
  return value.toLocaleDateString("fr");
};

const {
  data: semesters,
  refresh,
  status,
} = getSemesters({
  composable: "useAsyncData",
  key: "getSemesters",
});

// const semestersComputed = computed(() => {
//   if (semesters.value) {
//     return semesters.value
//       .sort((a, b) => {
//         if (a.start_date < b.start_date) {
//           return -1;
//         }
//         if (a.start_date > b.start_date) {
//           return 1;
//         }
//         return 0;
//       });
//   } else {
//     return [];
//   }
// });

const defaultSemester = () => {
  return {};
};

const zSemester = zNewSemester.refine((obj) => obj.start_date < obj.end_date, {
  path: ["end_date"],
  message: "La date de fin doit être postérieure à la date de début",
});

const { handleSubmit, handleReset, defineField, errors, isSubmitting } =
  useForm({
    validationSchema: toTypedSchema(zSemester),
    initialValues: {
      name: "",
      start_date: new Date().toISOString(),
      end_date: add(new Date(), { months: 5 }).toISOString(),
    },
  });

const [name, nameProps] = defineField("name");
const [start_date, start_dateProps] = defineField("start_date");
const [end_date, end_dateProps] = defineField("end_date");

const semesterDialogState = ref<"hidden" | "post" | "put">("hidden");
const semesterDialogShown = computed({
  get: () => semesterDialogState.value != "hidden",
  set: (value: boolean) => {
    if (!value) {
      semesterDialogState.value = "hidden";
    }
  },
});
const saveSemesterError = ref<string | null>(null);

const saveSemester = (id: number | null) => {
  const onResponseError = (e: {
    response: { status: number; _data: BadRequestReason };
  }) => {
    if (e.response._data && e.response.status === 400) {
      const d = e.response._data as GetDummyErrors[400];
      if (d.type === "UniqueViolation") {
        saveSemesterError.value = "Un semestre avec ce nom existe déjà";
      } else if (d.type === "CheckViolation") {
        saveSemesterError.value = d.message;
      } else {
        saveSemesterError.value = "Erreur inconnue";
      }
    }

    refresh();
  };

  if (!name.value || !start_date.value || !end_date.value) {
    console.error("Invalid form");
    return;
  }

  const newValue = {
    name: name.value,
    start_date: new Date(start_date.value),
    end_date: new Date(end_date.value),
  };

  let promise;

  if (id !== null) {
    const body = { id, ...newValue };
    update_by_id(semesters.value, body);
    promise = putSemester({
      composable: "$fetch",
      body: body,
      onResponseError: onResponseError,
    });
  } else {
    promise = postSemester({
      composable: "$fetch",
      body: newValue,
      onResponseError: onResponseError,
    }).then((created) => {
      semesters.value?.push(created);
    });
  }

  promise.then(() => {
    semesterDialogState.value = "hidden";
  });
};

const submit = handleSubmit((values) => {
  console.log("Submitting", values);
  alert(JSON.stringify(values, null, 2));
});

const deleteSemestersDialog = ref(false);
const deleteSemester = (id: number) => {
  deleteSemesterBySemesterId({
    composable: "$fetch",
    path: { semester_id: id },
  })
    .then(() => {
      if (semesters.value) {
        semesters.value = semesters.value.filter((s) => s.id == id);
      }
      deleteSemestersDialog.value = false;
    })
    .catch(() => {
      console.log("error");
      refresh();
    });
};
</script>

<template>
  <v-sheet border rounded>
    <v-data-table
      v-if="semesters"
      :loading="status == 'pending'"
      :items="semesters"
      :headers="[
        { title: 'Nom', key: 'name' },
        { title: 'Début', key: 'start_date' },
        { title: 'Fin', key: 'end_date' },
        { title: 'Actions', value: 'actions', align: 'end' },
      ]"
      :hide-default-footer="semesters.length < 11"
      item-value="id"
    >
      <template #top>
        <v-toolbar flat>
          <v-toolbar-title>
            <v-icon
              color="medium-emphasis"
              icon="mdi-calendar-multiple"
              size="x-small"
              start
            />

            Semestres
          </v-toolbar-title>

          <v-btn
            class="me-2"
            prepend-icon="mdi-plus"
            rounded="lg"
            text="Nouveau semestre"
            border
            @click="
              () => {
                handleReset();
                semesterDialogState = 'post';
              }
            "
          />
        </v-toolbar>
      </template>
      <template #item.start_date="{ item }">
        {{ formatDate(item.start_date) }}
      </template>
      <template #item.end_date="{ item }">
        {{ formatDate(item.end_date) }}
      </template>

      <template #item.actions="{ item }">
        <div class="d-flex ga-2 justify-end">
          <v-icon
            color="medium-emphasis"
            icon="mdi-pencil"
            size="small"
            @click="
              () => {
                name = item.name;
                start_date = item.start_date;
                end_date = item.end_date;
                semesterDialogState = 'put';
              }
            "
          />

          <v-icon
            color="medium-emphasis"
            icon="mdi-delete"
            size="small"
            @click="deleteSemester(item.id)"
          />
        </div>
      </template>
    </v-data-table>
  </v-sheet>

  <v-dialog v-model="semesterDialogShown" max-width="450" :modal="true">
    <v-form @submit.prevent="submit">
      <v-card
        :title="
          semesterDialogState == 'post'
            ? 'Nouveau semestre'
            : 'Modifier semestre'
        "
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
          <div
            v-if="saveSemesterError"
            severity="error"
            size="small"
            variant="outline"
          >
            {{ saveSemesterError }}
          </div>
        </v-card-text>

        <v-card-actions>
          <v-btn @click="() => (semesterDialogState = 'hidden')">Annuler</v-btn>
          <v-btn :loading="isSubmitting" type="submit">Enregistrer</v-btn>
        </v-card-actions>
      </v-card>
    </v-form>
  </v-dialog>
</template>
