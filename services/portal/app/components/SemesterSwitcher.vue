<script setup lang="ts">
import type { DropdownMenuItem } from "@nuxt/ui";
import type { Semester } from "#hey-api/types.gen";
import type SemesterDialogComponent from "~/components/SemesterDialog.vue";

const { canEdit } = useRoles();

const toast = useToast();

const {
  semesters,
  sortedSemesters,
  selectedSemesterId,
  selectedSemester,
  refresh,
  status,
} = useSemesters();

const selectedIndex = computed(() =>
  sortedSemesters.value.findIndex((s) => s.id === selectedSemesterId.value),
);

// Semesters are sorted most recent first.
const previous = computed(() => sortedSemesters.value[selectedIndex.value + 1]);
const next = computed(() =>
  selectedIndex.value > 0
    ? sortedSemesters.value[selectedIndex.value - 1]
    : undefined,
);

const dialog =
  useTemplateRef<InstanceType<typeof SemesterDialogComponent>>(
    "semesterDialog",
  );

const onSaved = (semester: Semester) => {
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
      toast.add({ title: "Semestre supprimé avec succès", color: "success" });
    });
  });
};

const items = computed<DropdownMenuItem[][]>(() => {
  const semester = selectedSemester.value;
  return [
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
        disabled: !canEdit.value,
        onSelect: () => dialog.value?.open(),
      },
      ...(semester
        ? [
            {
              label: `Modifier ${semester.short_name}`,
              icon: "i-lucide-pencil",
              disabled: !canEdit.value,
              onSelect: () => dialog.value?.open(semester),
            },
            {
              label: `Supprimer ${semester.short_name}`,
              icon: "i-lucide-trash-2",
              color: "error" as const,
              disabled: !canEdit.value,
              onSelect: () => deleteSemester(semester),
            },
          ]
        : []),
    ],
  ];
});
</script>

<template>
  <div v-if="selectedSemester" class="flex flex-col gap-2">
    <span
      id="semester-switcher-label"
      class="text-xs font-semibold tracking-widest text-muted uppercase"
    >
      Semestre
    </span>
    <div
      role="group"
      aria-labelledby="semester-switcher-label"
      class="flex items-stretch rounded-xl border border-default bg-muted"
    >
      <UButton
        icon="i-lucide-chevron-left"
        color="neutral"
        variant="link"
        aria-label="Semestre précédent"
        :disabled="!previous"
        class="px-3"
        @click="selectedSemesterId = previous?.id"
      />
      <UDropdownMenu :items="items">
        <button
          type="button"
          class="flex flex-1 flex-col items-center rounded-lg py-2 leading-tight hover:bg-elevated focus-visible:outline-2 focus-visible:outline-primary"
          aria-label="Choisir ou gérer les semestres"
        >
          <span class="font-display text-xl font-bold text-highlighted">
            {{ selectedSemester.short_name }}
          </span>
          <span class="text-xs text-muted">
            {{ selectedSemester.long_name }}
          </span>
        </button>
      </UDropdownMenu>
      <UButton
        icon="i-lucide-chevron-right"
        color="neutral"
        variant="link"
        aria-label="Semestre suivant"
        :disabled="!next"
        class="px-3"
        @click="selectedSemesterId = next?.id"
      />
    </div>
  </div>

  <UButton
    v-else-if="status !== 'pending'"
    label="Créer un semestre"
    icon="i-lucide-plus"
    color="neutral"
    variant="outline"
    block
    :disabled="!canEdit"
    @click="dialog?.open()"
  />

  <SemesterDialog
    ref="semesterDialog"
    :semesters="semesters ?? []"
    @saved="onSaved"
  />
</template>
