/** Pages reserved to the Orga group: others land on their profile. */
export default defineNuxtRouteMiddleware(async () => {
  const user = await currentUser();
  if (!user.value.groups.includes("Orga")) {
    return navigateTo("/profile", { replace: true });
  }
});
