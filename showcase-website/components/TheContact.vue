<script setup lang="ts">
import { postPublicSendContactForm } from "~/client";

const email = ref("");
const first_name = ref("");
const last_name = ref("");
const message = ref("");

const postForm = async () => {
  await postPublicSendContactForm({
    composable: "$fetch",
    body: {
      email: email.value,
      first_name: first_name.value,
      last_name: last_name.value,
      message: message.value,
    },
  });
};
</script>

<template>
  <div
    class="flex flex-col gap-2 bg-slate-900 px-4 pb-10 pt-20 text-white md:pb-16"
  >
    <h1>CONTACT</h1>
    <input v-model="first_name" class="text-black" />
    <input v-model="last_name" class="text-black" />
    <input v-model="email" class="text-black" />
    <input v-model="message" class="text-black" />
    <button @click="postForm">Send</button>
  </div>
</template>

<style scoped>
.image-shadow {
  box-shadow: 0.7em 0.7em 2em #000000a6;
}
</style>
