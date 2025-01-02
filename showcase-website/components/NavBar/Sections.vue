<script lang="ts"></script>

<template>
  <NavBarSection v-for="(child, index) in $slots?.default()" :key="index">
    <component :is="child" />
  </NavBarSection>
</template>
