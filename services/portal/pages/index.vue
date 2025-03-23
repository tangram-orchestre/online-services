<script setup lang="ts">
import { getUsersMe } from "~/client";

const { data } = getUsersMe({
  composable: "useAsyncData",
});
</script>

<template>
  <h1>Mon profil</h1>
  <div v-if="data" class="mt-2">
    <h3 class="mb-2">Informations personnelles</h3>
    <v-row>
      <v-col cols="12" sm="6">
        <v-text-field
          v-model="data.username"
          hide-details
          label="Nom d'utilisateur"
          readonly
        />
      </v-col>
      <v-col cols="12" sm="6">
        <v-text-field
          v-model="data.email"
          hide-details
          label="Email"
          readonly
        />
      </v-col>
    </v-row>
    <v-row>
      <v-col cols="12" sm="6">
        <v-text-field
          v-model="data.first_name"
          hide-details
          label="Prénom"
          readonly
        />
      </v-col>
      <v-col>
        <v-text-field
          v-model="data.last_name"
          hide-details
          label="Nom"
          readonly
        />
      </v-col>
    </v-row>
    <v-row>
      <v-col cols="12" sm="6">
        <v-text-field
          v-model="data.phone_number"
          hide-details
          label="Téléphone"
          readonly
        />
      </v-col>
      <v-col>
        <v-btn
          color="primary"
          block
          href="https://auth.tangram-orchestre.fr/if/user/#/settings"
        >
          Modifier mes informations
        </v-btn>
      </v-col>
    </v-row>
  </div>
</template>
