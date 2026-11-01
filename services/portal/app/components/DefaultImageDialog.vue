<script setup lang="ts">
/** Edit the image of concerts without one of their own, room or venue. */
const toast = useToast();
const { settings } = useSettings();

const shown = ref(false);
const imageId = ref<string | undefined>();
const saving = ref(false);
const saveError = ref<string | null>(null);

const open = () => {
  imageId.value = settings.value?.default_concert_image_id;
  saveError.value = null;
  shown.value = true;
};
defineExpose({ open });

const save = async () => {
  // The default can be replaced, not removed.
  if (!imageId.value) return;
  const newSettings = { default_concert_image_id: imageId.value };
  saving.value = true;
  try {
    await putSettings({
      composable: "$fetch",
      body: newSettings,
      onResponseError: onResponseErrorHandler("image", true, (errorMsg) => {
        saveError.value = errorMsg;
      }),
    });
    settings.value = newSettings;
    shown.value = false;
    toast.add({ title: "Image par défaut enregistrée", color: "success" });
  } catch {
    // Reported by onResponseError.
  } finally {
    saving.value = false;
  }
};
</script>

<template>
  <UModal
    v-model:open="shown"
    title="Image par défaut"
    description="Illustre sur le site vitrine les concerts sans image, ni photo de leur salle ou de leur lieu."
  >
    <template #body>
      <div class="flex flex-col gap-3">
        <ImageInput v-model="imageId" alt="Image par défaut des concerts" />
        <p v-if="!imageId" class="text-sm text-muted">
          Choisissez une image : il en faut toujours une par défaut.
        </p>
        <p v-if="saveError" class="text-sm text-error">{{ saveError }}</p>
      </div>
    </template>
    <template #footer>
      <div class="flex w-full justify-end gap-2">
        <UButton
          label="Annuler"
          color="neutral"
          variant="ghost"
          @click="shown = false"
        />
        <UButton
          label="Enregistrer"
          :disabled="!imageId"
          :loading="saving"
          @click="save()"
        />
      </div>
    </template>
  </UModal>
</template>
