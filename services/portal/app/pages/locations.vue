<script setup lang="ts">
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import { zNewLocation } from "#hey-api/zod.gen";
import type { TableColumn } from "@nuxt/ui";
import type { Location } from "#hey-api/types.gen";

const toast = useToast();
const formId = useId();

const {
  data: locations,
  refresh,
  status,
} = getLocations({
  composable: "useAsyncData",
  key: "getLocations",
});

const { handleSubmit, handleReset, defineField, errors, isSubmitting } =
  useForm({
    validationSchema: toTypedSchema(zNewLocation),
  });

const [name, nameProps] = defineField("name");
const [city, cityProps] = defineField("city");
const [zipcode, zipcodeProps] = defineField("zipcode");
const [street, streetProps] = defineField("street");

const locationDialogShown = ref(false);
const saveLocationId = ref<number | null>(null);
const saveLocationError = ref<string | null>(null);

watch(locationDialogShown, (shown) => {
  if (shown) {
    saveLocationError.value = null;
  }
});

const saveLocation = (id: number | null) => {
  const onResponseError = onResponseErrorHandler("lieu", false, (errorMsg) => {
    saveLocationError.value = errorMsg;
  });

  const newValue = {
    name: name.value!,
    city: city.value!,
    zipcode: zipcode.value!,
    street: street.value!,
  };

  let promise;

  if (id !== null) {
    promise = putLocation({
      composable: "$fetch",
      body: { id, ...newValue },
      onResponseError,
    }).then(() => {
      if (locations.value) {
        locations.value = locations.value.map((l) =>
          l.id === id ? { ...l, ...newValue } : l,
        );
      }
    });
  } else {
    promise = postLocation({
      composable: "$fetch",
      body: newValue,
      onResponseError,
    }).then((created) => {
      if (locations.value) {
        locations.value = [...locations.value, created];
      }
    });
  }

  promise.then(() => {
    locationDialogShown.value = false;
    toast.add({ title: "Lieu enregistré avec succès", color: "success" });
  });
};

const submit = handleSubmit(() => {
  saveLocation(saveLocationId.value);
});

const deleteLocation = (id: number, locationName: string) => {
  confirmDialog(
    "Confirmer la suppression",
    `Êtes-vous sûr de vouloir supprimer le lieu « ${locationName} » ?`,
  ).then((confirmed) => {
    if (confirmed) {
      deleteLocationByLocationId({
        composable: "$fetch",
        path: { location_id: id },
        onResponseError: onResponseErrorHandler("lieu", false, (errorMsg) => {
          toast.add({ title: errorMsg, color: "error" });
          refresh();
        }),
      }).then(() => {
        if (locations.value) {
          locations.value = locations.value.filter((l) => l.id !== id);
        }
        toast.add({ title: "Lieu supprimé avec succès", color: "success" });
      });
    }
  });
};

const newLocation = () => {
  handleReset();
  saveLocationId.value = null;
  locationDialogShown.value = true;
};

const editLocation = (location: Location) => {
  name.value = location.name;
  city.value = location.city;
  zipcode.value = location.zipcode;
  street.value = location.street;
  saveLocationId.value = location.id;
  locationDialogShown.value = true;
};

const columns: TableColumn<Location>[] = [
  { accessorKey: "name", header: sortableHeader("Nom") },
  { accessorKey: "city", header: sortableHeader("Ville") },
  { accessorKey: "zipcode", header: sortableHeader("Code postal") },
  { accessorKey: "street", header: "Adresse" },
  { id: "actions", meta: { class: { td: "text-right" } } },
];
const sorting = ref([{ id: "city", desc: false }]);
</script>

<template>
  <div class="mb-3 flex items-center gap-1">
    <h1 class="flex items-center gap-2 text-xl font-semibold">
      <UIcon name="i-lucide-map-pin" class="text-muted" />
      Lieux
    </h1>
    <UButton
      icon="i-lucide-plus"
      color="neutral"
      variant="ghost"
      aria-label="Nouveau lieu"
      @click="newLocation()"
    />
  </div>

  <UCard :ui="{ body: 'p-0 sm:p-0' }">
    <UTable
      v-model:sorting="sorting"
      :data="locations ?? []"
      :columns="columns"
      :loading="status == 'pending'"
      empty="Aucun lieu"
    >
      <template #actions-cell="{ row }">
        <div class="flex justify-end gap-1">
          <UButton
            icon="i-lucide-pencil"
            color="neutral"
            variant="ghost"
            size="sm"
            aria-label="Modifier le lieu"
            @click="editLocation(row.original)"
          />
          <UButton
            icon="i-lucide-trash-2"
            color="neutral"
            variant="ghost"
            size="sm"
            aria-label="Supprimer le lieu"
            @click="deleteLocation(row.original.id, row.original.name)"
          />
        </div>
      </template>
    </UTable>
  </UCard>

  <UModal
    v-model:open="locationDialogShown"
    :title="saveLocationId === null ? 'Nouveau lieu' : 'Modifier le lieu'"
  >
    <template #body>
      <form :id="formId" class="flex flex-col gap-4" @submit.prevent="submit">
        <UFormField label="Nom" :error="errors.name">
          <UInput v-model="name" v-bind="nameProps" autofocus class="w-full" />
        </UFormField>
        <UFormField label="Ville" :error="errors.city">
          <UInput v-model="city" v-bind="cityProps" class="w-full" />
        </UFormField>
        <UFormField label="Code postal" :error="errors.zipcode">
          <UInput v-model="zipcode" v-bind="zipcodeProps" class="w-full" />
        </UFormField>
        <UFormField label="Adresse" :error="errors.street">
          <UInput v-model="street" v-bind="streetProps" class="w-full" />
        </UFormField>
        <p v-if="saveLocationError" class="text-sm text-error">
          {{ saveLocationError }}
        </p>
      </form>
    </template>

    <template #footer>
      <div class="flex w-full justify-end gap-2">
        <UButton
          label="Annuler"
          color="neutral"
          variant="ghost"
          @click="locationDialogShown = false"
        />
        <UButton
          label="Enregistrer"
          type="submit"
          :form="formId"
          :loading="isSubmitting"
        />
      </div>
    </template>
  </UModal>
</template>
