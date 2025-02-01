<script setup lang="ts">
import { postPublicSendContactForm } from "~/client";
import { zContactForm } from "~/client/zod.gen";

const contactForm = ref({
  name: "",
  email: "",
  message: "",
});

const { status, error, execute } = postPublicSendContactForm({
  composable: "useAsyncData",
  asyncDataOptions: {
    server: false,
    immediate: false,
  },
  body: contactForm,
});

watch(status, (s) => {
  if (s == "success") {
    contactForm.value.name = "";
    contactForm.value.email = "";
    contactForm.value.message = "";
  }
});

const errors = ref({
  name: null as string | null,
  email: null as string | null,
  message: null as string | null,
});

const postForm = async (e: Event) => {
  e.preventDefault();

  // Clean errors
  for (const key of Object.keys(
    errors.value,
  ) as (keyof typeof errors.value)[]) {
    errors.value[key] = null;
  }

  const res = zContactForm.safeParse(contactForm.value);
  if (!res.success) {
    for (const key of Object.keys(
      errors.value,
    ) as (keyof typeof errors.value)[]) {
      const issue = res.error.issues.find((i) => i.path[0] == key);
      if (issue) {
        errors.value[key] = issue.message;
      }
    }
    console.error(res.error.message);

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
          v-model="contactForm.name"
          placeholder="Nom"
          class="rounded-lg p-2 text-black"
          required
        />
        <p v-if="errors.name" class="mt-3 text-red-400">{{ errors.name }}</p>
        <label class="lilita-one-regular mb-2 mt-4 text-xl">Email</label>
        <input
          v-model="contactForm.email"
          placeholder="Email"
          type="email"
          class="rounded-lg p-2 text-black"
          required
        />
        <p v-if="errors.email" class="mt-3 text-red-400">{{ errors.email }}</p>
        <label class="lilita-one-regular mb-2 mt-4 text-xl">Message</label>
        <textarea
          v-model="contactForm.message"
          placeholder="Votre message"
          class="min-h-32 rounded-lg p-2 text-black"
          required
        />
        <p v-if="errors.message" class="mt-3 text-red-400">
          {{ errors.message }}
        </p>
        <p v-if="error" class="mt-3 text-red-400">Erreur : {{ error.data }}</p>
        <p v-if="status == 'success'" class="mt-3 text-green-400">
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
