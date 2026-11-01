import type { User } from "#hey-api/types.gen";

/**
 * Roles of the current user (loaded by `app.vue`). Orga members see the
 * organisation pages, only Bureau members can change them.
 */
export const useRoles = () => {
  const user = useState<User | undefined>("currentUser");
  const inGroup = (group: string) =>
    computed(() => user.value?.groups.includes(group) ?? false);
  return { isOrga: inGroup("Orga"), canEdit: inGroup("Bureau") };
};
