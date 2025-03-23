<script setup lang="ts">
const user = await currentUser();
</script>

<template>
  <v-container>
    <h1>Mon profil</h1>
    <div v-if="user" class="mt-2">
      <h3 class="mb-4">Informations personnelles</h3>

      <v-defaults-provider
        :defaults="{
          VTextField: {
            variant: 'outlined',
            disabled: true,
          },
        }"
      >
        <v-row>
          <v-col cols="12" sm="6">
            <v-text-field
              v-model="user.username"
              hide-details
              label="Nom d'utilisateur"
            />
          </v-col>
          <v-col cols="12" sm="6">
            <v-text-field v-model="user.email" hide-details label="Email" />
          </v-col>
        </v-row>
        <v-row>
          <v-col cols="12" sm="6">
            <v-text-field
              v-model="user.first_name"
              hide-details
              label="Prénom"
            />
          </v-col>
          <v-col>
            <v-text-field v-model="user.last_name" hide-details label="Nom" />
          </v-col>
        </v-row>
        <v-row>
          <v-col cols="12" sm="6">
            <v-text-field
              v-model="user.phone_number"
              hide-details
              label="Téléphone"
            />
          </v-col>
          <v-col>
            <v-btn
              color="primary"
              height="56"
              block
              href="https://auth.tangram-orchestre.fr/if/user/#/settings"
            >
              Modifier mes informations
            </v-btn>
          </v-col>
        </v-row>
      </v-defaults-provider>
    </div>

    <v-divider class="mt-4" />
  </v-container>
</template>
