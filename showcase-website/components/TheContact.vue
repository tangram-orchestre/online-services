<script setup lang="ts">
import { getApiHello } from "~/client";

const name = ref("Luc");

const nameDebounced = useDebounce(name, 250);

const { data, status } = getApiHello({
  composable: "useAsyncData",
  key: "hello",
  query: { name: nameDebounced },
  asyncDataOptions: {
    watch: [nameDebounced],
  },
});
</script>

<!-- eslint-disable tailwindcss/no-custom-classname -->
<template>
  <div class="bg-slate-900 px-4 pb-10 pt-20 text-white md:pb-16">
    <h1>CONTACT</h1>
    <p>{{ status }} {{ data }}</p>
    <input v-model="name" class="text-black" />
  </div>
</template>

<style scoped>
.image-shadow {
  box-shadow: 0.7em 0.7em 2em #000000a6;
}
</style>
