<script setup lang="ts">
const slots = defineSlots();
</script>

<template>
  <NavBarSection v-for="(child, index) in slots.default()" :key="index">
    <component :is="child" />
  </NavBarSection>
</template>
