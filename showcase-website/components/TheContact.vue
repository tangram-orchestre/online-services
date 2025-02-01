<script setup lang="ts">
import { postPublicSendContactForm } from "~/client";
import { zContactForm } from "~/client/zod.gen";

const contactForm = ref({
  name: "",
  email: "",
  message: "",
});

type FormKeys = keyof typeof contactForm.value;

const messageLength = computed(() => contactForm.value.message.length);

const {
  status,
  error: backendError,
  execute,
} = postPublicSendContactForm({
  composable: "useAsyncData",
  asyncDataOptions: {
    server: false,
    immediate: false,
  },
  body: contactForm,
});

const zodErrors = ref({
  name: null as string | null,
  email: null as string | null,
  message: null as string | null,
});

// Mark
const changed = ref({
  name: false,
  email: false,
  message: false,
});

// Reset all fields on succesful submition
watch(status, (s) => {
  if (s == "success") {
    for (const key of Object.keys(contactForm.value) as FormKeys[]) {
      contactForm.value[key] = "";
      changed.value[key] = false;
    }
  }
});

const checkForm = (isChange: boolean, keyToCheck: FormKeys | null) => {
  const keysToCheck = keyToCheck
    ? [keyToCheck]
    : (Object.keys(zodErrors.value) as FormKeys[]);

  // Clean errors
  for (const key of keysToCheck) {
    zodErrors.value[key] = null;
    if (isChange) {
      changed.value[key] = true;
    }
  }

  const res = zContactForm.safeParse(contactForm.value);
  if (res.success) {
    return true;
  }

  for (const key of keysToCheck) {
    const issue = res.error.issues.find((i) => i.path[0] == key);
    if (changed.value[key] && issue) {
      zodErrors.value[key] = issue.message;
    }
  }
  console.error(res.error.message);

  return false;
};

const postForm = async () => {
  if (!checkForm(true, null)) {
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
          @change="checkForm(true, 'name')"
          @input="checkForm(false, 'name')"
        />
        <p v-if="zodErrors.name" class="mt-2 text-red-400">
          {{ zodErrors.name }}
        </p>
        <label class="lilita-one-regular mb-2 mt-4 text-xl">Email</label>
        <input
          v-model="contactForm.email"
          placeholder="Email"
          type="email"
          class="rounded-lg p-2 text-black"
          required
          @change="checkForm(true, 'email')"
          @input="checkForm(false, 'email')"
        />
        <p v-if="zodErrors.email" class="mt-2 text-red-400">
          {{ zodErrors.email }}
        </p>
        <label class="lilita-one-regular mb-2 mt-4 text-xl">Message</label>
        <textarea
          v-model="contactForm.message"
          placeholder="Votre message"
          class="min-h-32 rounded-lg p-2 text-black"
          required
          @change="checkForm(true, 'message')"
          @input="checkForm(false, 'message')"
        />
        <div class="mt-2 flex">
          <p v-if="status == 'success'" class="text-green-400">
            Message envoyé !
          </p>
          <p v-else-if="status == 'pending'">Envoie en cours...</p>
          <div class="flex-col text-red-400">
            <div v-if="zodErrors.message">{{ zodErrors.message }}</div>
            <div v-if="backendError" class="">
              {{ backendError.data }}
            </div>
          </div>
          <div class="ml-auto">{{ messageLength }} / 2000 caractères</div>
        </div>

        <button
          class="lilita-one-regular mx-auto mt-4 w-1/2 rounded-lg border-2 bg-[#81ccb5] p-2 text-2xl text-black hover:bg-[#8adac2]"
          :class="{
            'disabled cursor-default brightness-75': status == 'pending',
          }"
          @click.prevent="postForm"
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
