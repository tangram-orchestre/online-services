<script setup lang="ts" generic="T extends string | number">
defineProps<{
  label: string;
  items: { value: T; label: string }[];
}>();

const model = defineModel<T>({ required: true });
</script>

<template>
  <div role="group" :aria-label="label" class="flex flex-wrap gap-2">
    <UButton
      v-for="item in items"
      :key="item.value"
      :label="item.label"
      :aria-pressed="model === item.value"
      :color="model === item.value ? 'primary' : 'neutral'"
      :variant="model === item.value ? 'solid' : 'outline'"
      class="rounded-full"
      @click="model = item.value"
    />
  </div>
</template>
