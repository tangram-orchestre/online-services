<script setup lang="ts">
const { current, set } = useNavBar();

const isLocked = useScrollLock(window);

const isOpen = ref(false);

watch(isOpen, (isOpen) => {
  isLocked.value = isOpen;
});

const indexToOrder = (index: number) => {
  if (index < 2) {
    return 0;
  } else {
    return 4;
  }
};

const sections = ["Missions", "Concerts", "Réseaux", "Contact"];
</script>

<template>
  <nav
    class="roboto-light fixed top-0 z-30 flex h-16 w-full items-center gap-8 bg-white p-4 text-center text-3xl uppercase text-gray-800 shadow-xl transition-all duration-500 min-[400px]:justify-center"
  >
    <div
      v-for="(section, index) in sections"
      :key="section"
      class="hidden cursor-pointer decoration-2 underline-offset-2 lg:block"
      :class="{ underline: index == current }"
      :style="`order: ${indexToOrder(index)}`"
      @click="set(index)"
    >
      {{ section }}
    </div>

    <div class="order-2 mx-4 flex flex-row items-center">
      <Icon class="triangle -rotate-90" name="mdi:triangle" />
      <div class="mx-4 text-4xl">TANGRAM</div>
      <Icon class="triangle rotate-90" name="mdi:triangle" />
    </div>

    <div
      class="nav-burger absolute right-6 order-4 lg:hidden"
      :class="{ 'nav-active': isOpen }"
      @click="isOpen = !isOpen"
    >
      <div class="nav-burger-1" />
      <div class="nav-burger-2" />
      <div class="nav-burger-3" />
    </div>
  </nav>
  <Transition name="slide">
    <div
      v-if="isOpen"
      class="roboto-light fixed top-16 z-20 flex w-full flex-col items-center justify-center border-b-4 bg-slate-900"
    >
      <div
        v-for="(section, index) in sections"
        :key="section"
        class="w-full cursor-pointer py-4 text-center text-3xl uppercase text-white decoration-2 underline-offset-2 hover:bg-slate-700"
        :class="{ 'bg-slate-700': index == current }"
        @click="
          set(index);
          isOpen = false;
        "
      >
        {{ section }}
      </div>
    </div>
  </Transition>
  <Transition>
    <Teleport to="body">
      <div
        v-if="isOpen"
        class="backdrop fixed z-10 size-full"
        @click="isOpen = false"
      />
    </Teleport>
  </Transition>
</template>

<style lang="scss" scoped>
.triangle {
  @apply hidden text-[#ffc303] min-[475px]:block;
}

$burger-bar-width: 50px;
$burger-bar-height: 6px;
$burger-bar-margin: 6px;
$animation-time: 200ms;

.nav-burger {
  cursor: pointer;
  &.nav-active {
    $translate: $burger-bar-height + $burger-bar-margin;
    .nav-burger-1 {
      transform: translateY($translate) rotate(45deg);
    }
    .nav-burger-2 {
      opacity: 0;
    }
    .nav-burger-3 {
      transform: translateY(-$translate) rotate(-45deg);
    }
  }
}

.nav-burger div {
  @apply bg-slate-900 rounded-full;

  width: $burger-bar-width;
  height: $burger-bar-height;
  margin: $burger-bar-margin;

  transition: all $animation-time ease;
}
/* we will explain what these classes do next! */
.slide-enter-active,
.slide-leave-active {
  transition: transform $animation-time ease;
}

.slide-enter-from,
.slide-leave-to {
  transform: translateY(-100%);
}

.backdrop {
  backdrop-filter: blur(5px);

  &.v-leave-active {
    pointer-events: none;
  }

  transition: all $animation-time;

  &.v-enter-from,
  &.v-leave-to {
    backdrop-filter: blur(0);
  }
}
</style>
