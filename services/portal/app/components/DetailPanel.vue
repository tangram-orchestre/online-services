<script setup lang="ts">
/**
 * Details of the item selected in a list: a column next to the list on large
 * screens, a slide-over on small ones.
 */
defineProps<{
  /** Slide-over title, for small screens. */
  title: string;
}>();

/** Whether an item was explicitly selected: opens the slide-over. */
const open = defineModel<boolean>("open", { required: true });

// The column is shown with CSS (no layout shift on hydration); the
// slide-over must not also open on large screens.
const large = useMediaQuery("(min-width: 64rem)");
const slideoverOpen = computed({
  get: () => open.value && !large.value,
  set: (value) => {
    open.value = value;
  },
});
</script>

<template>
  <aside
    class="sticky top-6 hidden self-start rounded-lg border border-default bg-default p-6 lg:block"
  >
    <slot />
  </aside>
  <USlideover v-model:open="slideoverOpen" :title="title">
    <template #body>
      <slot />
    </template>
  </USlideover>
</template>
