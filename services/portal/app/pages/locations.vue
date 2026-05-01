<script setup lang="ts">
import type { GetDummyErrors, BadRequestReason } from "#hey-api/types.gen";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import { zNewLocation } from "#hey-api/zod.gen";
import { toast } from "vuetify-sonner";

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
  const onResponseError = (e: {
    response: { status: number; _data: BadRequestReason };
  }) => {
    if (e.response._data && e.response.status === 400) {
      const d = e.response._data as GetDummyErrors[400];
      if (d.type === "UniqueViolation") {
        saveLocationError.value = "Un lieu avec ces informations existe déjà";
      } else {
        saveLocationError.value = "Erreur inconnue";
      }
    } else {
      saveLocationError.value = `Erreur ${e.response.status}`;
    }
    refresh();
  };

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
    toast.success("Lieu enregistré avec succès");
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
      })
        .then(() => {
          if (locations.value) {
            locations.value = locations.value.filter((l) => l.id !== id);
          }
          toast.success("Lieu supprimé avec succès");
        })
        .catch(() => {
          refresh();
        });
    }
  });
};
</script>

<template>
  <v-sheet border rounded>
    <v-data-table
      v-if="locations"
      :loading="status == 'pending'"
      :items="locations"
      :headers="[
        { title: 'Nom', key: 'name' },
        { title: 'Ville', key: 'city' },
        { title: 'Code postal', key: 'zipcode' },
        { title: 'Adresse', key: 'street' },
        { title: 'Actions', value: 'actions', align: 'end' },
      ]"
      :hide-default-footer="locations.length < 11"
      :sort-by="[{ key: 'city', order: 'asc' }]"
      item-value="id"
    >
      <template #top>
        <v-toolbar flat>
          <v-toolbar-title>
            <v-icon
              color="medium-emphasis"
              icon="mdi-map-marker"
              size="x-small"
              start
            />
            Lieux
          </v-toolbar-title>

          <v-btn
            class="me-2"
            prepend-icon="mdi-plus"
            rounded="lg"
            text="Nouveau lieu"
            border
            @click="
              () => {
                handleReset();
                saveLocationId = null;
                locationDialogShown = true;
              }
            "
          />
        </v-toolbar>
      </template>

      <template #item.actions="{ item }">
        <div class="d-flex ga-2 justify-end">
          <v-icon
            color="medium-emphasis"
            icon="mdi-pencil"
            size="small"
            @click="
              () => {
                name = item.name;
                city = item.city;
                zipcode = item.zipcode;
                street = item.street;
                saveLocationId = item.id;
                locationDialogShown = true;
              }
            "
          />
          <v-icon
            color="medium-emphasis"
            icon="mdi-delete"
            size="small"
            @click="deleteLocation(item.id, item.name)"
          />
        </div>
      </template>
    </v-data-table>
  </v-sheet>

  <v-dialog v-model="locationDialogShown" max-width="450" :modal="true">
    <v-form @submit.prevent="submit">
      <v-card
        :title="saveLocationId === null ? 'Nouveau lieu' : 'Modifier lieu'"
      >
        <v-card-text>
          <v-text-field
            v-model="name"
            v-bind="nameProps"
            label="Nom"
            :error-messages="errors.name"
            autofocus
          />
          <v-text-field
            v-model="city"
            v-bind="cityProps"
            label="Ville"
            :error-messages="errors.city"
          />
          <v-text-field
            v-model="zipcode"
            v-bind="zipcodeProps"
            label="Code postal"
            :error-messages="errors.zipcode"
          />
          <v-text-field
            v-model="street"
            v-bind="streetProps"
            label="Adresse"
            :error-messages="errors.street"
          />
          <div v-if="saveLocationError" class="text-error">
            {{ saveLocationError }}
          </div>
        </v-card-text>

        <v-card-actions>
          <v-btn @click="() => (locationDialogShown = false)">Annuler</v-btn>
          <v-btn :loading="isSubmitting" type="submit">Enregistrer</v-btn>
        </v-card-actions>
      </v-card>
    </v-form>
  </v-dialog>
</template>
