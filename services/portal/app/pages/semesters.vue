<script setup lang="ts">
import { add } from "date-fns";
import type { GetDummyErrors, BadRequestReason } from "#hey-api/types.gen";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";

import { zNewSemester } from "#hey-api/zod.gen";
import { toast } from "vuetify-sonner";

const formatDate = (value: string) => {
  return new Date(value).toLocaleDateString("fr");
};

const serializeDate = (date: Date) => {
  return date.toISOString().split("T")[0];
};

const {
  data: semesters,
  refresh,
  status,
} = getSemesters({
  composable: "useAsyncData",
  key: "getSemesters",
});

const zSemester = zNewSemester.refine((obj) => obj.start_date < obj.end_date, {
  path: ["end_date"],
  message: "La date de fin doit être postérieure à la date de début",
});

const { handleSubmit, handleReset, defineField, errors, isSubmitting } =
  useForm({
    validationSchema: toTypedSchema(zSemester),
  });

const [name, nameProps] = defineField("name");
const [start_date, start_dateProps] = defineField("start_date");
const [end_date, end_dateProps] = defineField("end_date");

const setDefaultValues = () => {
  const today = new Date();
  const defaultStartDate =
    semesters.value?.reduce((latest, semester) => {
      const semesterEnd = new Date(semester.end_date);
      return semesterEnd > latest ? semesterEnd : latest;
    }, today) || today;
  const defaultEndDate = add(defaultStartDate, { months: 6 });

  name.value = "";
  start_date.value = serializeDate(defaultStartDate);
  end_date.value = serializeDate(defaultEndDate);
};

const semesterDialogShown = ref(false);
const saveSemesterId = ref<number | null>(null);
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
    } else {
      saveSemesterError.value = `Erreur ${e.response.status}`;
    }
    refresh();
  };

  if (!name.value || !start_date.value || !end_date.value) {
    return;
  }

  const newValue = {
    name: name.value,
    start_date: start_date.value,
    end_date: end_date.value,
  };

  let promise;

  if (id !== null) {
    const body = { id, ...newValue };
    promise = putSemester({
      composable: "$fetch",
      body: body,
      onResponseError: onResponseError,
    }).then(() => {
      if (semesters.value) {
        semesters.value = semesters.value.map((s) =>
          s.id === id ? { ...s, ...newValue } : s,
        );
      }
    });
  } else {
    promise = postSemester({
      composable: "$fetch",
      body: newValue,
      onResponseError: onResponseError,
    }).then((created) => {
      if (semesters.value) {
        semesters.value = [...semesters.value, created];
      }
    });
  }

  promise.then(() => {
    semesterDialogShown.value = false;
    toast.success("Semestre enregistré avec succès");
  });
};

const submit = handleSubmit(() => {
  saveSemester(saveSemesterId.value);
});

const deleteSemestersDialog = ref(false);
const deleteSemester = (id: number) => {
  deleteSemesterBySemesterId({
    composable: "$fetch",
    path: { semester_id: id },
  })
    .then(() => {
      if (semesters.value) {
        semesters.value = semesters.value.filter((s) => s.id !== id);
      }
      deleteSemestersDialog.value = false;
      toast.success("Semestre supprimé avec succès");
    })
    .catch(() => {
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
                setDefaultValues();
                saveSemesterId = null;
                semesterDialogShown = true;
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

                saveSemesterId = item.id;
                semesterDialogShown = true;
              }
            "
          />

          <v-icon
            color="medium-emphasis"
            icon="mdi-delete"
            size="small"
            @click="
              () => {
                confirmDialog(
                  'Confirmer la suppression',
                  `Êtes-vous sûr de vouloir supprimer le semestre « ${item.name} » ?`,
                ).then((confirmed) => {
                  if (confirmed) {
                    deleteSemester(item.id);
                  }
                });
              }
            "
          />
        </div>
      </template>
    </v-data-table>
  </v-sheet>

  <v-dialog v-model="semesterDialogShown" max-width="450" :modal="true">
    <v-form @submit.prevent="submit">
      <v-card
        :title="
          saveSemesterId === null ? 'Nouveau semestre' : 'Modifier semestre'
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
          <div v-if="saveSemesterError" class="text-error">
            {{ saveSemesterError }}
          </div>
        </v-card-text>

        <v-card-actions>
          <v-btn @click="() => (semesterDialogShown = false)">Annuler</v-btn>
          <v-btn :loading="isSubmitting" type="submit">Enregistrer</v-btn>
        </v-card-actions>
      </v-card>
    </v-form>
  </v-dialog>
</template>
