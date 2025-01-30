<script setup lang="ts">
import type { ZodError, ZodIssue } from "zod";
import { any } from "zod";
import { postPublicSendContactForm } from "~/client";
import { zContactForm } from "~/client/zod.gen";

const name = ref("");
const email = ref("");
const message = ref("");

const body = computed(() => ({
  name: name.value,
  email: email.value,
  message: message.value,
}));

const { status, error, execute } = postPublicSendContactForm({
  composable: "useAsyncData",
  asyncDataOptions: {
    server: false,
    immediate: false,
  },
  body,
});

const zodError: Ref<null | ZodIssue[]> = ref(null);

watch(status, (s) => {
  if (s == "success") {
    name.value = "";
    email.value = "";
    message.value = "";
  }
});

const postForm = async (e: Event) => {
  e.preventDefault();
  const res = zContactForm.safeParse(body.value);
  if (!res.success) {
    zodError.value = res.error.issues;
    return;
  }
  execute();
};
</script>

<template>
  <div class="bg-slate-900 px-4 pb-10 pt-20 text-white md:pb-16">
    <h1 class="oswald mb-6 text-center text-6xl tracking-wider sm:text-7xl">
      NOUS CONTACTER
    </h1>
    <div class="mx-auto max-w-screen-sm">
      <form class="flex w-full flex-col">
        <label class="lilita-one-regular mb-2 mt-4 text-xl">Nom</label>
        <input
          v-model="name"
          placeholder="Nom"
          class="rounded-lg p-2 text-black"
          required
        />
        <label class="lilita-one-regular mb-2 mt-4 text-xl">Email</label>
        <input
          v-model="email"
          placeholder="Email"
          type="email"
          class="rounded-lg p-2 text-black"
          required
        />
        <label class="lilita-one-regular mb-2 mt-4 text-xl">Message</label>
        <textarea
          v-model="message"
          placeholder="Votre message"
          class="min-h-32 rounded-lg p-2 text-black"
          required
        />
        <p v-if="error" class="mt-3 text-red-400">Erreur : {{ error.data }}</p>
        <li v-else-if="zodError">
          <ul v-for="(err, i) in zodError" :key="i">
            {{
              err.message
            }}
          </ul>
        </li>
        <p v-else-if="status == 'success'" class="mt-3 text-green-400">
          Message envoyé !
        </p>

        <button
          class="lilita-one-regular mx-auto mt-4 w-1/2 rounded-lg border-2 bg-[#81ccb5] p-2 text-2xl text-black hover:bg-[#8adac2]"
          :class="{
            'disabled cursor-default brightness-75': status == 'pending',
          }"
          @click="postForm"
        >
          Envoyer
        </button>
      </form>
    </div>
  </div>
</template>

<style scoped>
.image-shadow {
  box-shadow: 0.7em 0.7em 2em #000000a6;
}
</style>
