import ConfirmDialog from "~/components/ConfirmDialog.vue";

/** Ask the user to confirm an action, resolves to false when dismissed. */
export async function confirmDialog(
  title: string,
  message: string,
): Promise<boolean> {
  const modal = useOverlay().create(ConfirmDialog, { destroyOnClose: true });
  return (await modal.open({ title, message })) ?? false;
}
