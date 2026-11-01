import { client } from "#hey-api/client.gen";

/**
 * Configures the API client. In a plugin rather than in `app.vue`, so that it
 * is ready for route middleware, which run before the app setup.
 */
export default defineNuxtPlugin(() => {
  const runtimeConfig = useRuntimeConfig();
  client.setConfig({
    baseURL: runtimeConfig.public.api_base_url,
    credentials: "include",
  });
});
