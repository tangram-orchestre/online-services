<script setup lang="ts">
import { parseDate } from "@internationalized/date";
import type { CalendarDate } from "@internationalized/date";

// Date field with a calendar popover, bound to an ISO date string
// ("2025-01-31") like the API uses.

const model = defineModel<string | undefined>();

const props = defineProps<{
  min?: string;
  max?: string;
}>();

const toDate = (value?: string) => (value ? parseDate(value) : undefined);

const date = computed<CalendarDate | undefined>({
  get: () => toDate(model.value),
  set: (value) => {
    model.value = value?.toString();
  },
});

const inputDate = useTemplateRef("inputDate");
</script>

<template>
  <UInputDate
    ref="inputDate"
    v-model="date"
    :min-value="toDate(props.min)"
    :max-value="toDate(props.max)"
    class="w-full"
  >
    <template #trailing>
      <UPopover :reference="inputDate?.inputsRef[3]?.$el">
        <UButton
          color="neutral"
          variant="link"
          size="sm"
          icon="i-lucide-calendar"
          aria-label="Choisir une date"
          class="px-0"
        />
        <template #content>
          <UCalendar
            v-model="date"
            :min-value="toDate(props.min)"
            :max-value="toDate(props.max)"
            class="p-2"
          />
        </template>
      </UPopover>
    </template>
  </UInputDate>
</template>
