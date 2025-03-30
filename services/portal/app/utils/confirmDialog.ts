import type ConfirmDialog from "~/components/ConfirmDialog.vue";

let dialog: null | InstanceType<typeof ConfirmDialog> = null;

export function setConfirmDialogInstance(
  instance: InstanceType<typeof ConfirmDialog>,
) {
  dialog = instance;
}

export async function confirmDialog(
  title: string,
  message: string,
): Promise<boolean> {
  if (!dialog) {
    throw new Error("ConfirmDialog instance not set");
  }

  return dialog.open(title, message);
}
