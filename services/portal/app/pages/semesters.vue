<script setup lang="ts">
// The semester view now lives at the root; keep old links working.
definePageMeta({
  redirect: (to) => ({ path: "/", query: to.query }),
});
</script>

<template>
  <div />
</template>
