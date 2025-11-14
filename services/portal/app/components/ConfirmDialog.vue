<template>
  <v-dialog
    v-model="dialog"
    :max-width="options.width"
    :style="{ zIndex: options.zIndex }"
    @keydown.esc="cancel"
  >
    <v-card>
      <v-toolbar dark :color="options.color" dense flat>
        <v-toolbar-title class="white--text">{{ title }}</v-toolbar-title>
      </v-toolbar>
      <v-card-text v-show="!!message" class="pa-4">{{ message }}</v-card-text>
      <v-card-actions class="pt-0">
        <v-spacer />
        <v-btn color="grey" text @click="cancel">Cancel</v-btn>
        <v-btn color="error" text @click="agree">Yes</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>
<script setup lang="ts">
interface ConfirmDialogOptions {
  color?: string;
  width?: number;
  zIndex?: number;
}

const dialog = ref(false);
const message = ref<string | null>(null);
const title = ref<string | null>(null);
const options = reactive<ConfirmDialogOptions>({
  color: "primary",
  width: 290,
  zIndex: 200,
});

let resolveFn: ((value: boolean) => void) | null = null;

function open(
  newTitle: string,
  newMessage: string,
  newOptions?: ConfirmDialogOptions,
): Promise<boolean> {
  dialog.value = true;
  title.value = newTitle;
  message.value = newMessage;
  Object.assign(options, newOptions);
  return new Promise<boolean>((resolve) => {
    resolveFn = resolve;
  });
}

function agree() {
  if (resolveFn) {
    resolveFn(true);
    resolveFn = null;
  }
  dialog.value = false;
}

function cancel() {
  if (resolveFn) {
    resolveFn(false);
    resolveFn = null;
  }
  dialog.value = false;
}

watch(dialog, (newVal) => {
  if (!newVal && resolveFn !== null) {
    cancel();
  }
});

// Expose open for parent usage
defineExpose({ open });
</script>
