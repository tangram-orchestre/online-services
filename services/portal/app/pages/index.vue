<script setup lang="ts">
import type { Semester } from "#hey-api/types.gen";
import type { DropdownMenuItem } from "@nuxt/ui";
import type SemesterDialogComponent from "~/components/SemesterDialog.vue";

const route = useRoute();
const toast = useToast();
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
    `Êtes-vous sûr de vouloir supprimer le semestre « ${semester.long_name} » ?`,
  ).then((confirmed) => {
    if (!confirmed) return;
    deleteSemesterBySemesterId({
      composable: "$fetch",
      path: { semester_id: semester.id },
      onResponseError: onResponseErrorHandler("semestre", false, (errorMsg) => {
        toast.add({ title: errorMsg, color: "error" });
        refresh();
      }),
    }).then(() => {
      if (semesters.value) {
        semesters.value = semesters.value.filter((s) => s.id !== semester.id);
      }
      selectedSemesterId.value = undefined;
      toast.add({
        title: "Semestre supprimé avec succès",
        color: "success",
      });
    });
  });
};

const semesterMenu = computed<DropdownMenuItem[][]>(() => [
  sortedSemesters.value.map((s) => ({
    label: `${s.short_name} — ${s.long_name}`,
    type: "checkbox" as const,
    checked: s.id === selectedSemesterId.value,
    onSelect: () => {
      selectedSemesterId.value = s.id;
    },
  })),
  [
    {
      label: "Nouveau semestre",
      icon: "i-lucide-plus",
      onSelect: () => semesterDialogRef.value?.open(),
    },
  ],
]);

const semesterActions = computed<DropdownMenuItem[][]>(() => {
  const semester = selectedSemester.value;
  if (!semester) return [];
  return [
    [
      {
        type: "label",
        label: `${formatDate(semester.start_date)} → ${formatDate(semester.end_date)}`,
        icon: "i-lucide-calendar-range",
      },
    ],
    [
      {
        label: "Modifier le semestre",
        icon: "i-lucide-pencil",
        onSelect: () => semesterDialogRef.value?.open(semester),
      },
      {
        label: "Supprimer le semestre",
        icon: "i-lucide-trash-2",
        color: "error",
        onSelect: () => deleteSemester(semester),
      },
    ],
  ];
});
</script>

<template>
  <div v-if="selectedSemester" class="flex flex-wrap items-center gap-1">
    <UDropdownMenu :items="semesterMenu">
      <UButton
        :label="`${selectedSemester.long_name} (${selectedSemester.short_name})`"
        trailing-icon="i-lucide-chevron-down"
        color="neutral"
        variant="ghost"
        size="xl"
        class="text-2xl font-semibold"
      />
    </UDropdownMenu>

    <UDropdownMenu :items="semesterActions">
      <UButton
        icon="i-lucide-ellipsis-vertical"
        color="neutral"
        variant="ghost"
        aria-label="Détails du semestre"
      />
    </UDropdownMenu>
  </div>

  <UEmpty
    v-else-if="status !== 'pending'"
    icon="i-lucide-calendar"
    title="Aucun semestre"
    description="Créez un semestre pour y ajouter des morceaux et des concerts."
    :actions="[
      {
        label: 'Nouveau semestre',
        icon: 'i-lucide-plus',
        onClick: () => semesterDialogRef?.open(),
      },
    ]"
  />

  <div v-if="selectedSemester" class="mt-4 flex flex-col gap-8">
    <SemesterConcerts :semester="selectedSemester" />
    <SemesterPieces :semester="selectedSemester" />
  </div>

  <SemesterDialog
    ref="semesterDialog"
    :semesters="semesters ?? []"
    @saved="onSemesterSaved"
  />
</template>
