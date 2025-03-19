<script setup lang="ts">
const target = ref<null | HTMLElement>(null);

useNavBarSection(target);
</script>

<template>
  <div ref="target" class="w-full">
    <slot />
  </div>
</template>
