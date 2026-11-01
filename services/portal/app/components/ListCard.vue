<script setup lang="ts">
/**
 * A card listing items, one per row (`<li>` in the default slot), with
 * optional column titles.
 */
defineProps<{
  /** Shown instead of the rows when there are none. */
  empty?: string;
  /** Grid of the column titles (`header` slot), matching the rows'. */
  headerClass?: string;
}>();
</script>

<template>
  <div
    class="@container overflow-hidden rounded-lg border border-default bg-default"
  >
    <div
      v-if="$slots.header"
      class="hidden gap-4 border-b border-default px-5 py-3 text-xs font-semibold tracking-wider text-muted uppercase"
      :class="headerClass"
    >
      <slot name="header" />
    </div>
    <ul class="divide-y divide-muted">
      <slot />
      <li v-if="empty" class="px-5 py-8 text-center text-muted">
        {{ empty }}
      </li>
    </ul>
  </div>
</template>
