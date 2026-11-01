import { UButton } from "#components";

interface SortableColumn {
  getIsSorted: () => false | "asc" | "desc";
  toggleSorting: (desc?: boolean) => void;
}

/** Table header rendering a button that toggles the column sorting. */
export const sortableHeader =
  (label: string) =>
  ({ column }: { column: SortableColumn }) => {
    const sorted = column.getIsSorted();
    return h(UButton, {
      label,
      color: "neutral",
      variant: "ghost",
      class: "-mx-2.5",
      trailingIcon:
        sorted === "asc"
          ? "i-lucide-arrow-up-narrow-wide"
          : sorted === "desc"
            ? "i-lucide-arrow-down-wide-narrow"
            : "i-lucide-arrow-up-down",
      onClick: () => column.toggleSorting(sorted === "asc"),
    });
  };
