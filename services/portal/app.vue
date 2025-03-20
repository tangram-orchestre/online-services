<script lang="ts" setup>
const toggleDarkMode = () => {
  const html = document.querySelector("html")!;
  html.classList.toggle("tangram-dark");
};
</script>

<template>
  <div>
    <Button
      label="Toggle Dark Theme"
      icon="pi pi-moon"
      @click="toggleDarkMode()"
    />
  </div>
</template>

<style lang="scss">
@use "~/assets/scss/main.scss";
</style>
