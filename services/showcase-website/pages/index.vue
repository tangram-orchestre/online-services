<script setup lang="ts">
import TheContact from "~/components/TheContact.vue";
</script>

<template>
  <Head>
    <Title>Tangram Orchestre</Title>
  </Head>

  <NavBarTop />

  <div class="animate min-h-screen overflow-hidden">
    <div
      class="heading mx-auto mb-20 mt-40 grid max-w-4xl justify-items-center ease-in-out"
    >
      <div class="logo">
        <TheLogo class="size-full" />
      </div>
      <div
        class="title-container flex flex-col items-center justify-center pt-4 drop-shadow-2xl lg:pt-0"
      >
        <div
          class="title roboto-thin text-[calc(clamp(1px,15vw,6rem))] leading-[1.067em]"
        >
          TANGRAM
        </div>
        <div
          class="sub-title oswald text-center text-[calc(clamp(1px,3vw,1.3rem))] tracking-wide"
        >
          Orchestre amateur pour Chef⸱fe⸱s en herbe
        </div>
      </div>
    </div>

    <main>
      <NavBarSections>
        <TheMissions />
        <TheConcerts />
        <div>
          <TheLinks />
          <TheCagnotte />
        </div>
        <TheContact />
      </NavBarSections>
    </main>

    <TheFooter />
  </div>
</template>

<style lang="scss" scoped>
$animate-in-title-time: 1500ms;
$animate-in-title-duration: 1000ms;
$animate-in-main-time: $animate-in-title-time + 700ms;
$animate-in-main-duration: 1000ms;

.logo {
  min-width: 150px;
  @media (min-width: 300px) {
    height: 180px;
  }
  @media (min-width: 400px) {
    height: 290px;
  }
}

@keyframes grid-animation {
  from {
    grid-template-columns: 100% 0;
  }
  to {
    grid-template-columns: 40% 60%;
  }
}

@keyframes opacity-animation {
  from {
    opacity: 0;
  }
  to {
    opacity: 100%;
  }
}

@media (min-width: 1024px) {
  .heading {
    max-width: 950px;
    grid-template-columns: 100% 0;
    .animate & {
      animation: grid-animation $animate-in-title-duration ease-in-out
        $animate-in-title-time forwards;
    }
  }
}

.title-container {
  opacity: 0;
  .animate & {
    animation: opacity-animation $animate-in-title-duration ease-in-out
      $animate-in-title-time forwards;
  }
}

main {
  opacity: 0;
  .animate & {
    animation: opacity-animation $animate-in-main-duration ease-in-out
      $animate-in-main-time forwards;
  }
}
</style>
