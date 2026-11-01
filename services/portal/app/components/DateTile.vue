<script setup lang="ts">
const props = defineProps<{
  /** API date (`YYYY-MM-DD`). */
  date: string;
  /** `tertiary` highlights concerts; `secondary` is the quieter variant. */
  color?: "tertiary" | "secondary";
}>();

const day = computed(() => parseDay(props.date));
const weekday = computed(() =>
  day.value
    .toLocaleDateString("fr", { weekday: "short" })
    .replace(".", "")
    .toUpperCase(),
);
const month = computed(() =>
  day.value.toLocaleDateString("fr", { month: "short" }),
);
</script>

<template>
  <time
    :datetime="date"
    class="flex w-15 shrink-0 flex-col items-center rounded-md py-2 leading-tight"
    :class="
      color === 'secondary'
        ? 'bg-secondary/15 text-highlighted'
        : 'bg-tertiary text-on-tertiary'
    "
  >
    <span class="text-[11px] font-semibold tracking-wider">{{ weekday }}</span>
    <span class="font-display text-3xl leading-none font-bold">
      {{ day.getDate() }}
    </span>
    <span class="text-[11px]">{{ month }}</span>
  </time>
</template>
