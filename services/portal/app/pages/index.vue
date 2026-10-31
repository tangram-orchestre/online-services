<script setup lang="ts">
import type { Semester } from "#hey-api/types.gen";
import { toast } from "vuetify-sonner";
import type SemesterDialogComponent from "~/components/SemesterDialog.vue";

const route = useRoute();
const router = useRouter();

// Semester management is reserved to the Orga group; others land on their
// profile.
const user = await currentUser();
if (!user.value.groups.includes("Orga")) {
  await navigateTo("/profile", { replace: true });
}

const formatDate = (value: string) => new Date(value).toLocaleDateString("fr");

const {
  data: semesters,
  refresh,
  status,
} = getSemesters({
  composable: "useAsyncData",
  key: "getSemesters",
});

const sortedSemesters = computed(() =>
  [...(semesters.value ?? [])].sort((a, b) =>
    b.start_date.localeCompare(a.start_date),
  ),
);

const semesterItems = computed(() =>
  sortedSemesters.value.map((s) => ({ title: s.name, value: s.id })),
);

/** Semester containing today, or the most recent one. */
const defaultSemester = () => {
  const today = new Date().toISOString().split("T")[0]!;
  return (
    sortedSemesters.value.find(
      (s) => s.start_date <= today && today <= s.end_date,
    ) ?? sortedSemesters.value[0]
  );
};

// Selected semester is kept in the URL so it survives reloads and can be
// shared.
const selectedSemesterId = computed<number | undefined>({
  get: () => {
    const fromQuery = Number(route.query.semester);
    if (sortedSemesters.value.some((s) => s.id === fromQuery)) {
      return fromQuery;
    }
    return defaultSemester()?.id;
  },
  set: (id) => {
    router.replace({ query: { ...route.query, semester: id } });
  },
});

const selectedSemester = computed(() =>
  sortedSemesters.value.find((s) => s.id === selectedSemesterId.value),
);

const semesterDialogRef =
  useTemplateRef<InstanceType<typeof SemesterDialogComponent>>(
    "semesterDialog",
  );

const onSemesterSaved = (semester: Semester) => {
  if (semesters.value) {
    const exists = semesters.value.some((s) => s.id === semester.id);
    semesters.value = exists
      ? semesters.value.map((s) => (s.id === semester.id ? semester : s))
      : [...semesters.value, semester];
  }
  selectedSemesterId.value = semester.id;
};

const deleteSemester = (semester: Semester) => {
  confirmDialog(
    "Confirmer la suppression",
    `Êtes-vous sûr de vouloir supprimer le semestre « ${semester.name} » ?`,
  ).then((confirmed) => {
    if (!confirmed) return;
    deleteSemesterBySemesterId({
      composable: "$fetch",
      path: { semester_id: semester.id },
      onResponseError: onResponseErrorHandler("semestre", false, (errorMsg) => {
        toast.error(errorMsg);
        refresh();
      }),
    }).then(() => {
      if (semesters.value) {
        semesters.value = semesters.value.filter((s) => s.id !== semester.id);
      }
      selectedSemesterId.value = undefined;
      toast.success("Semestre supprimé avec succès");
    });
  });
};
</script>

<template>
  <div v-if="selectedSemester" class="d-flex align-center flex-wrap ga-2">
    <v-menu>
      <template #activator="{ props: menuProps }">
        <v-btn
          v-bind="menuProps"
          variant="text"
          class="text-h5 text-none px-2"
          append-icon="mdi-menu-down"
          :text="`Semestre ${selectedSemester.name}`"
        />
      </template>
      <v-list density="compact">
        <v-list-item
          v-for="item in semesterItems"
          :key="item.value"
          :title="item.title"
          :active="item.value === selectedSemesterId"
          @click="selectedSemesterId = item.value"
        />
        <v-divider class="my-1" />
        <v-list-item
          prepend-icon="mdi-plus"
          title="Nouveau semestre"
          @click="semesterDialogRef?.open()"
        />
      </v-list>
    </v-menu>

    <v-menu>
      <template #activator="{ props: menuProps }">
        <v-btn
          v-bind="menuProps"
          icon="mdi-dots-vertical"
          size="small"
          variant="text"
          color="medium-emphasis"
          title="Détails du semestre"
        />
      </template>
      <v-list density="compact">
        <v-list-item
          prepend-icon="mdi-calendar-range"
          :title="`${formatDate(selectedSemester.start_date)} → ${formatDate(selectedSemester.end_date)}`"
          subtitle="Période du semestre"
          :link="false"
        />
        <v-divider class="my-1" />
        <v-list-item
          prepend-icon="mdi-pencil"
          title="Modifier le semestre"
          @click="semesterDialogRef?.open(selectedSemester)"
        />
        <v-list-item
          prepend-icon="mdi-delete"
          title="Supprimer le semestre"
          base-color="error"
          @click="deleteSemester(selectedSemester)"
        />
      </v-list>
    </v-menu>
  </div>

  <v-empty-state
    v-else-if="status !== 'pending'"
    icon="mdi-calendar-blank"
    title="Aucun semestre"
    text="Créez un semestre pour y ajouter des morceaux et des concerts."
  >
    <template #actions>
      <v-btn
        prepend-icon="mdi-plus"
        text="Nouveau semestre"
        border
        rounded="lg"
        @click="semesterDialogRef?.open()"
      />
    </template>
  </v-empty-state>

  <div v-if="selectedSemester" class="d-flex flex-column ga-8 mt-4">
    <semester-concerts :semester="selectedSemester" />
    <semester-pieces :semester="selectedSemester" />
  </div>

  <semester-dialog
    ref="semesterDialog"
    :semesters="semesters ?? []"
    @saved="onSemesterSaved"
  />
</template>
