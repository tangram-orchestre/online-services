<script setup lang="ts">
defineProps<{
  /** Small uppercase line above the title (« Semestre A26 »). */
  overline?: string;
  title: string;
  description?: string;
}>();
</script>

<template>
  <header class="flex flex-wrap items-end justify-between gap-5">
    <div class="flex flex-col gap-1.5">
      <span
        v-if="overline"
        class="text-sm font-semibold tracking-wider text-muted uppercase"
      >
        {{ overline }}
      </span>
      <h1
        class="text-4xl leading-none font-bold tracking-tight text-highlighted sm:text-5xl"
      >
        {{ title }}
      </h1>
      <p v-if="description" class="text-muted">{{ description }}</p>
    </div>
    <slot name="actions" />
  </header>
</template>
