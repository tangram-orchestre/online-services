<script setup lang="ts">
import { add } from "date-fns";
import {
  deleteSemesterBySemesterId,
  getSemesters,
  postSemester,
  putSemester,
  type GetDummyErrors,
  type NewSemester,
  type Reason,
  type Semester,
} from "~/client";

import {
  convert_fields_from_dates,
  convert_fields_to_dates,
  type Converted,
} from "~/utils/date";
import type { ElementOfRefArray } from "~/utils/utils";
import { zodResolver } from "@primevue/forms/resolvers/zod";
import { zNewSemester } from "~/client/zod.gen";
import { z } from "zod";

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
    return null;
  }
});

const defaultSemester = () => {
  return {
    name: "",
    start_date: new Date(),
    end_date: add(new Date(), { months: 5 }),
  };
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

const semesterResolver = ref(zodResolver(zSemester));

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
    response: { status: number; _data: Reason };
  }) => {
    if (e.response._data && e.response.status === 400) {
      const d = e.response._data as GetDummyErrors[400];
      if (d === "UniqueViolation") {
        saveSemesterError.value = "Un semestre avec ce nom existe déjà";
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

type SemesterComputed = ElementOfRefArray<typeof semestersComputed>;

const selectedSemesters = ref<SemesterComputed[]>([]);
const deleteSemestersDialog = ref(false);
const deleteSemesters = () => {
  Promise.all(
    selectedSemesters.value.map((s) => {
      return deleteSemesterBySemesterId({
        composable: "$fetch",
        path: { semester_id: s.id },
      });
    }),
  )
    .then(() => {
      console.log("success");
      if (semesters.value) {
        semesters.value = semesters.value.filter(
          (s) => !selectedSemesters.value.some((o) => s.id == o.id),
        );
      }
      deleteSemestersDialog.value = false;
      selectedSemesters.value = [];
    })
    .catch(() => {
      console.log("error");
      refresh();
    });
};
</script>

<template>
  <DataTable
    v-if="semestersComputed"
    v-model:selection="selectedSemesters"
    :value="semestersComputed"
    edit-mode="row"
    data-key="id"
  >
    <template #header>
      <div class="flex flex-wrap items-center justify-between gap-2">
        <span class="text-xl font-bold">Semestres</span>
        <Button
          icon="pi pi-refresh"
          rounded
          raised
          :loading="status === 'pending'"
          @click="() => refresh()"
        />
      </div>
      <div>
        <Button
          label="Créer"
          icon="pi pi-plus"
          class="mr-2"
          @click="semesterDialogState = 'post'"
        />
        <Button
          label="Supprimer"
          icon="pi pi-trash"
          severity="danger"
          outlined
          :disabled="selectedSemesters.length == 0"
          @click="deleteSemestersDialog = true"
        />
      </div>
    </template>

    <Column selection-mode="multiple" class="w-10" :exportable="false" />

    <Column class="w-12" field="name" header="Nom">
      <template #body="{ data }">
        {{ data.name }}
      </template>
    </Column>

    <Column class="w-24" field="start_date" data-type="date" header="Début">
      <template #body="{ data }">
        {{ formatDate(data.start_date) }}
      </template>
    </Column>

    <Column class="w-24" field="end_date" data-type="date" header="Fin">
      <template #body="{ data }">
        {{ formatDate(data.end_date) }}
      </template>
    </Column>

    <Column class="!text-end">
      <template #body="{ data }">
        <Button
          icon="pi pi-pencil"
          severity="secondary"
          rounded
          @click="
            () => {
              newSemester = data;
              semesterDialogState = 'put';
            }
          "
        />
      </template>
    </Column>
  </DataTable>

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
      :resolver="semesterResolver"
      class="flex flex-col gap-4 w-full"
      @submit="
        ({ valid }) => {
          if (valid) {
            saveSemester();
          }
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
  </Dialog>
</template>
