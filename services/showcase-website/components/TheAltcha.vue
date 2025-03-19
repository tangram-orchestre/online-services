<script setup lang="ts">
import { ClientOnly } from "#components";
import { ref } from "vue";
import type { GetPublicAltchaChallengeData } from "~/client";
import { client } from "~/client/client.gen";

declare global {
  interface Window {
    altchaCustomFetch: (url: string, init: RequestInit) => Promise<Response>;
  }
}

const altcha_url: GetPublicAltchaChallengeData["url"] =
  "/public/altcha_challenge";

// Importing altcha package will introduce a new element <altcha-widget>
if (import.meta.browser) {
  import("altcha");
}

const altchaWidget = ref<HTMLElement | null>(null);
const model = defineModel<unknown>();

const onStateChange = (ev: CustomEvent | Event) => {
  if ("detail" in ev) {
    const { payload, state } = ev.detail;
    if (state === "verified" && payload) {
      model.value = JSON.parse(atob(payload));
    } else {
      model.value = null;
    }
  }
};

const altchaCustomFetch = (url: string, init: RequestInit) => {
  return fetch(url, {
    ...init,
    credentials: "include", // Include cookies with the request
  });
};

// Make the function available globally
onMounted(() => {
  window.altchaCustomFetch = altchaCustomFetch;
});
</script>

<template>
  <!-- Configure your `challengeurl` and remove the `test` attribute, see docs: https://altcha.org/docs/website-integration/#using-altcha-widget -->
  <ClientOnly>
    <altcha-widget
      ref="altchaWidget"
      style="--altcha-max-width: 100%"
      :challengeurl="client.getConfig().baseURL + altcha_url"
      customfetch="altchaCustomFetch"
      @statechange="onStateChange"
    />
  </ClientOnly>
</template>
