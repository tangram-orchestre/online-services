<script setup lang="ts">
/** A selectable row of a `ListCard`, its details shown in a `DetailPanel`. */
// Classes and listeners go to the button, not to the `<li>`.
defineOptions({ inheritAttrs: false });

defineProps<{
  /**
   * `selected` when explicitly selected; `shown` when shown by default in the
   * large screen panel, so only highlighted on large screens.
   */
  state?: "selected" | "shown";
}>();
</script>

<template>
  <li>
    <button
      v-bind="$attrs"
      type="button"
      class="w-full border-s-3 px-5 py-3.5 text-start transition-colors hover:bg-muted focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-primary"
      :class="
        state === 'selected'
          ? 'border-secondary bg-secondary/10 hover:bg-secondary/15'
          : state === 'shown'
            ? 'border-transparent lg:border-secondary lg:bg-secondary/10 lg:hover:bg-secondary/15'
            : 'border-transparent'
      "
      :aria-current="state ? 'true' : undefined"
    >
      <slot />
    </button>
  </li>
</template>
