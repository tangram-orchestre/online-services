import { getUsersMe, type User } from "~/client";

export default async function (): Promise<globalThis.Ref<User>> {
  const currentUser = useState<User>("currentUser");
  if (!currentUser.value) {
    console.log("Fetching current user");
    currentUser.value = await getUsersMe({ composable: "$fetch" });
  }
  return currentUser;
}
