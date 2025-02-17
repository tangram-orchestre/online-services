<script setup lang="ts">
import { ClientOnly } from "#components";
import { ref } from "vue";
import type { GetPublicAltchaChallengeData } from "~/client";
import { client } from "~/client/client.gen";

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
</script>

<template>
  <!-- Configure your `challengeurl` and remove the `test` attribute, see docs: https://altcha.org/docs/website-integration/#using-altcha-widget -->
  <ClientOnly>
    <altcha-widget
      ref="altchaWidget"
      style="--altcha-max-width: 100%"
      :challengeurl="client.getConfig().baseURL + altcha_url"
      @statechange="onStateChange"
    />
  </ClientOnly>
</template>
