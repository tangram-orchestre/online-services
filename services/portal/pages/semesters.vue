<script setup lang="ts">
import { add } from "date-fns";
import {
  deleteSemesterBySemesterId,
  getSemesters,
  postSemester,
  putSemester,
  type GetDummyErrors,
  type NewSemester,
  type BadRequestReason,
  type Semester,
} from "~/client";

import {
  convert_fields_from_dates,
  convert_fields_to_dates,
  type Converted,
} from "~/utils/date";
import type { ElementOfRefArray } from "~/utils/utils";
import { zNewSemester } from "~/client/zod.gen";
import { z } from "zod";
import { title } from "process";

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

const semestersComputed = computed(() => {
  if (semesters.value) {
    console.log(semesters.value);
    return semesters.value
      .map((s) => convert_fields_to_dates(s, ["start_date", "end_date"]))
      .sort((a, b) => {
        if (a.start_date < b.start_date) {
          return -1;
        }
        if (a.start_date > b.start_date) {
          return 1;
        }
        return 0;
      });
  } else {
    return [];
  }
});

const headers = [
  { title: "Nom", key: "name" },
  { title: "Début", key: "start_date" },
  { title: "Fin", key: "end_date" },
  { title: "Actions", value: "actions" },
];

const defaultSemester = () => {
  return {
    name: "",
    start_date: new Date(),
    end_date: add(new Date(), { months: 5 }),
  };
};

const activator = ref<Element | null>(null);

const registerActivator = (event: MouseEvent) => {
  activator.value = event.currentTarget as Element;
};

const newSemester: Ref<
  Converted<Date, Semester | NewSemester, "start_date" | "end_date">
> = ref(defaultSemester());

const zSemester = zNewSemester
  .merge(
    z.object({
      name: z.string().min(3),
      start_date: z.date(),
      end_date: z.date(),
    }),
  )
  .refine((obj) => obj.start_date < obj.end_date, {
    path: ["end_date"],
    message: "La date de fin doit être postérieure à la date de début",
  });

const semesterDialogState = ref<"hidden" | "post" | "put">("hidden");
const isSavePending = ref(false);
const saveSemesterError = ref<string | null>(null);

const saveSemesterReset = () => {
  semesterDialogState.value = "hidden";
  newSemester.value = defaultSemester();
  saveSemesterError.value = null;
};

const saveSemester = () => {
  isSavePending.value = true;

  const onResponseError = (e: {
    response: { status: number; _data: BadRequestReason };
  }) => {
    console.log("Error while saving semester", e);
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

  const newValue = convert_fields_from_dates(newSemester.value);

  let promise;

  if ("id" in newValue) {
    update_by_id(semesters.value, newValue);
    promise = putSemester({
      composable: "$fetch",
      body: newValue,
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

  promise.then(saveSemesterReset).finally(() => (isSavePending.value = false));
};

const selectedSemesterIds = ref<number[]>([]);
const deleteSemestersDialog = ref(false);
const deleteSemesters = () => {
  Promise.all(
    selectedSemesterIds.value.map((id) => {
      return deleteSemesterBySemesterId({
        composable: "$fetch",
        path: { semester_id: id },
      });
    }),
  )
    .then(() => {
      console.log("success");
      if (semesters.value) {
        semesters.value = semesters.value.filter(
          (s) => !selectedSemesterIds.value.some((id) => s.id == id),
        );
      }
      deleteSemestersDialog.value = false;
      selectedSemesterIds.value = [];
    })
    .catch(() => {
      console.log("error");
      refresh();
    });
};
</script>

<template>
  <h1>Semestres</h1>

  <p>Gestion des semestres.</p>

  <v-btn @click="refresh">Rafraichir</v-btn>
  <v-btn @click="deleteSemesters">Supprimer</v-btn>

  <v-data-table
    v-model="selectedSemesterIds"
    :loading="status == 'pending'"
    :items="semestersComputed"
    :headers
    :show-select="true"
    item-value="id"
  >
    <template #item.start_date="{ item }">
      {{ formatDate(item.start_date) }}
    </template>
    <template #item.end_date="{ item }">
      {{ formatDate(item.end_date) }}
    </template>

    <template #item.actions="{ item }">
      <v-btn
        variant="text"
        icon
        @click="
          () => {
            newSemester = item;
            semesterDialogState = 'put';
          }
        "
        @mouseenter="registerActivator($event)"
      >
        <v-icon>mdi-pencil</v-icon>
      </v-btn>
    </template>
  </v-data-table>

  <v-dialog
    v-if="activator"
    :value="semesterDialogState != 'hidden'"
    :activator="activator"
    max-width="450"
    :modal="true"
  >
    <v-confirm-edit
      ref="confirm"
      v-model="newSemester"
      ok-text="save"
      @cancel="semesterDialogState = 'hidden'"
      @save="saveSemester"
    >
      <template #default="{ model: proxyModel, actions }">
        <v-card
          :title="
            semesterDialogState == 'post'
              ? 'Nouveau semestre'
              : 'Modifier semestre'
          "
        >
          <v-card-text>
            <v-text-field v-model="proxyModel.value.name" label="Nom" />

            <v-date-picker
              v-model="proxyModel.value.start_date"
              label="Start Date"
            />
          </v-card-text>

          <template #actions>
            <component :is="actions" />
          </template>
        </v-card>
      </template>
    </v-confirm-edit>
  </v-dialog>

  <!--
  <Dialog
    :visible="semesterDialogState != 'hidden'"
    :style="{ width: '450px' }"
    :header="
      semesterDialogState == 'post' ? 'Nouveau semestre' : 'Modifier semestre'
    "
    :modal="true"
    @update:visible="semesterDialogState = 'hidden'"
  >
    <Form
      v-slot="$form"
      :initial-values="newSemester"
      class="flex flex-col gap-4 w-full"
      @submit="
        () => {
          saveSemester();
        }
      "
    >
      <div class="flex flex-col gap-4">
        <div class="flex flex-col gap-1">
          <div>Nom</div>
          <InputText v-model="newSemester.name" name="name" />
          <Message
            v-if="$form.name?.invalid"
            severity="error"
            size="small"
            variant="simple"
          >
            {{ $form.name.error?.message }}
          </Message>
        </div>
        <div class="flex flex-col gap-1">
          <div>Date de début</div>
          <DatePicker
            v-model="newSemester.start_date"
            name="start_date"
            show-icon
            :manual-input="false"
          />
          <Message
            v-if="$form.start_date?.invalid"
            severity="error"
            size="small"
            variant="simple"
          >
            {{ $form.start_date.error?.message }}
          </Message>
        </div>
        <div class="flex flex-col gap-1">
          <div>Date de fin</div>
          <DatePicker
            v-model="newSemester.end_date"
            name="end_date"
            show-icon
            :manual-input="false"
          />
          <Message
            v-if="$form.end_date?.invalid"
            severity="error"
            size="small"
            variant="simple"
          >
            {{ $form.end_date.error?.message }}
          </Message>
        </div>
        <Message
          v-if="saveSemesterError"
          severity="error"
          size="small"
          variant="outline"
        >
          {{ saveSemesterError }}
        </Message>
      </div>
      <div class="flex justify-end gap-2">
        <Button
          label="Annuler"
          icon="pi pi-times"
          text
          @click="saveSemesterReset"
        />
        <Button
          label="Valider"
          type="submit"
          icon="pi pi-check"
          :loading="isSavePending"
          :disabled="!$form.valid"
        />
      </div>
    </Form>
  </Dialog>

  <Dialog
    v-model:visible="deleteSemestersDialog"
    :style="{ width: '450px' }"
    header="Confirmation"
    :modal="true"
  >
    <div class="flex items-center gap-4">
      <i class="pi pi-exclamation-triangle !text-3xl" />
      <span v-if="selectedSemesters">
        Êtes-vous sûr de vouloir supprimer
        <b>{{ selectedSemesters.length }}</b>
        semestre{{ selectedSemesters.length > 1 ? "s" : "" }} ?
      </span>
    </div>
    <template #footer>
      <Button
        label="Non"
        icon="pi pi-times"
        text
        @click="deleteSemestersDialog = false"
      />
      <Button label="Oui" icon="pi pi-check" @click="deleteSemesters" />
    </template>
  </Dialog> -->
</template>
