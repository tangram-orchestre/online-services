import type { User } from "#hey-api/types.gen";

export default async function (): Promise<globalThis.Ref<User>> {
  const currentUser = useState<User>("currentUser");
  if (!currentUser.value) {
    currentUser.value = await getUsersMe({ composable: "$fetch" });
  }
  return currentUser;
}
