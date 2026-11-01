<script setup lang="ts">
defineProps<{
  title: string;
  message: string;
}>();

const emit = defineEmits<{ close: [confirmed: boolean] }>();
</script>

<template>
  <UModal :title="title" :description="message" :close="false">
    <template #footer>
      <div class="flex w-full justify-end gap-2">
        <UButton
          label="Annuler"
          color="neutral"
          variant="ghost"
          @click="emit('close', false)"
        />
        <UButton label="Confirmer" color="error" @click="emit('close', true)" />
      </div>
    </template>
  </UModal>
</template>
