/**
 * Settings of the website, shared by every page and dialog (same async data
 * key): updating `settings` after a save updates them everywhere.
 */
export const useSettings = () => {
  const { data: settings } = getSettings({
    composable: "useAsyncData",
    key: "getSettings",
  });
  return { settings };
};
