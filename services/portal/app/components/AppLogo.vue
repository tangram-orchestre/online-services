<script setup lang="ts">
defineProps<{
  /** Line shown under the wordmark. */
  subtitle?: string;
}>();
</script>

<template>
  <span class="flex items-center gap-3">
    <!-- The tangram mark keeps its brand colours whatever the theme roles. -->
    <svg viewBox="0 0 36 30" class="h-8 w-auto shrink-0" aria-hidden="true">
      <polygon points="0.5,0.2 20.2,0.2 10.3,10" class="fill-sun-200" />
      <polygon points="5.5,15.1 10.3,10.1 15.3,15.1" class="fill-sun-400" />
      <rect x="16" y="8.6" width="7.1" height="7" class="fill-teal-300" />
      <polygon points="9,15.6 23.1,15.6 23.1,29.7" class="fill-teal-600" />
      <polygon
        points="23.7,9.5 28.7,13 28.7,23 23.7,18"
        class="fill-rose-300"
      />
      <polygon points="28.7,13 33.7,8.5 33.7,18" class="fill-rose-300" />
      <polygon points="21.7,6 35.7,6 28.7,13" class="fill-rose-500" />
    </svg>
    <span class="flex flex-col leading-tight">
      <span
        class="font-display text-xl font-light tracking-[0.14em] text-highlighted"
      >
        TANGRAM
      </span>
      <span v-if="subtitle" class="text-sm text-muted">{{ subtitle }}</span>
    </span>
  </span>
</template>
