<script setup lang="ts">
/**
 * An uploaded image, by id. A picked file is uploaded right away: the backend
 * re-encodes it, and deletes it after a day unless it is saved somewhere.
 */
/** Absent when there is no image. */
const imageId = defineModel<string | undefined>({ required: true });

defineProps<{
  /** Text of the image, for screen readers. */
  alt: string;
}>();

const file = ref<File | null>(null);
const uploading = ref(false);
const error = ref<string | null>(null);

watch(file, async (picked) => {
  if (!picked) return;
  error.value = null;
  uploading.value = true;
  try {
    const image = await postImage({
      composable: "$fetch",
      body: { file: picked },
      onResponseError: onResponseErrorHandler("image", true, (errorMsg) => {
        error.value = errorMsg;
      }),
    });
    imageId.value = image.id;
  } catch {
    // Reported by onResponseError.
  } finally {
    uploading.value = false;
    file.value = null;
  }
});
</script>

<template>
  <div class="flex flex-col gap-2">
    <div v-if="imageId" class="flex flex-wrap items-end gap-3">
      <img
        :src="imageUrl(imageId)"
        :alt="alt"
        class="aspect-video w-56 rounded-md border border-default object-cover"
      />
      <div class="flex gap-1">
        <UFileUpload
          v-slot="{ open }"
          v-model="file"
          accept="image/jpeg,image/png,image/webp"
          :reset="true"
        >
          <UButton
            label="Remplacer"
            icon="i-lucide-image-up"
            color="neutral"
            variant="outline"
            :loading="uploading"
            @click="open()"
          />
        </UFileUpload>
        <UButton
          label="Retirer"
          icon="i-lucide-x"
          color="neutral"
          variant="ghost"
          :disabled="uploading"
          @click="imageId = undefined"
        />
      </div>
    </div>
    <UFileUpload
      v-else
      v-model="file"
      accept="image/jpeg,image/png,image/webp"
      :label="uploading ? 'Envoi en cours…' : 'Déposez une image ici'"
      description="JPEG, PNG ou WebP, 15 Mo maximum"
      icon="i-lucide-image-up"
      :disabled="uploading"
      class="min-h-36 w-full"
    />
    <p v-if="error" class="text-sm text-error">{{ error }}</p>
  </div>
</template>
