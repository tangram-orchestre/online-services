<script setup lang="ts">
import { parseTime } from "@internationalized/date";
import type { Time } from "@internationalized/date";

// 24h time field bound to an ISO time string ("20:30:00") like the API uses.

const model = defineModel<string | undefined | null>();

defineProps<{
  clearable?: boolean;
}>();

const time = computed<Time | undefined>({
  get: () => (model.value ? parseTime(model.value) : undefined),
  set: (value) => {
    model.value = value?.toString();
  },
});
</script>

<template>
  <UInputTime v-model="time" :hour-cycle="24" class="w-full">
    <template v-if="clearable && model" #trailing>
      <UButton
        color="neutral"
        variant="link"
        size="sm"
        icon="i-lucide-circle-x"
        aria-label="Effacer"
        class="px-0"
        @click="model = undefined"
      />
    </template>
  </UInputTime>
</template>
