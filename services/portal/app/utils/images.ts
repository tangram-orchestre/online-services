/** URL of an uploaded image (served by the public API, cached forever). */
export const imageUrl = (id: string) =>
  `${useRuntimeConfig().public.api_base_url}/public/images/${id}`;
