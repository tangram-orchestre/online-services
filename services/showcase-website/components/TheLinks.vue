<script setup lang="ts">
const links = [
  {
    url: "https://www.facebook.com/profile.php?id=100085621177297",
    label: "Facebook",
    icon: "uil:facebook",
    class: "bg-[#0866ff]",
  },
  {
    url: "https://www.instagram.com/tangram.orchestre?utm_source=ig_web_button_share_sheet&igsh=ZDNlZDc0MzIxNw==",
    label: "Instagram",
    icon: "uil:instagram",
    class: "bg-gradient-to-bl from-[#833ab4] via-[#fd1d1d] to-[#fcb045]",
  },
  {
    url: "https://www.youtube.com/@tangramorchestre",
    label: "Chaîne Youtube",
    icon: "uil:youtube",
    class: "bg-[#ff0033]",
  },
];
</script>

<!-- eslint-disable tailwindcss/no-custom-classname -->
<template>
  <div class="bg-slate-900 px-4 pb-10 pt-20 text-white md:pb-16">
    <h1 class="oswald mb-8 text-center text-6xl tracking-wider sm:text-7xl">
      NOUS SUIVRE
    </h1>
    <div
      class="container mx-auto flex flex-col items-center justify-center gap-x-20 gap-y-4 pt-6 text-[8rem] sm:flex-row md:text-[10rem]"
    >
      <a
        v-for="link in links"
        :key="link.url"
        class="block leading-none hover:brightness-125"
        :href="link.url"
        :aria-label="link.label"
      >
        <Icon :name="link.icon" :class="link.class" />
      </a>
    </div>
  </div>
</template>

<style scoped>
.image-shadow {
  box-shadow: 0.7em 0.7em 2em #000000a6;
}
</style>
