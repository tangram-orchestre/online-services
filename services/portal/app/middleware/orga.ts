/** Pages reserved to the Orga group: others land on their profile. */
export default defineNuxtRouteMiddleware(async () => {
  await currentUser();
  if (!useRoles().isOrga.value) {
    return navigateTo("/profile", { replace: true });
  }
});
