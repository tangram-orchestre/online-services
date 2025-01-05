<script setup lang="ts">
import { breakpointsTailwind } from "@vueuse/core";
import type ConcertCard from "./ConcertCard.vue";

const breakpoints = useBreakpoints(breakpointsTailwind);

const displayedConcertCount = ref(0);
const displayedConcertManuallyChanged = ref(false);

const adjustDisplayedConcertCount = () => {
  if (displayedConcertManuallyChanged.value) {
    return;
  }

  if (breakpoints.xl.value) {
    displayedConcertCount.value = 7;
  } else if (breakpoints.lg.value) {
    displayedConcertCount.value = 5;
  } else if (breakpoints.sm.value) {
    displayedConcertCount.value = 3;
  } else {
    displayedConcertCount.value = 3;
  }
};

useEventListener("resize", adjustDisplayedConcertCount);
onMounted(adjustDisplayedConcertCount);

const isLocked = useScrollLock(window);

interface FocusedConcert {
  index: number;
  element: HTMLDivElement;
  leaving: boolean;
}

const focus: Ref<FocusedConcert | null> = ref(null);
const nextFocus: Ref<number | null> = ref(null);
const focusedCoordinates = ref({
  top: "0",
  right: "0",
  bottom: "0",
  left: "0",
});

const updateFocusedSource = (element: HTMLDivElement) => {
  const rect = element.getBoundingClientRect();
  focusedCoordinates.value.top = rect.top + "px";
  focusedCoordinates.value.right = window.innerWidth - rect.right + "px";
  focusedCoordinates.value.bottom = window.innerHeight - rect.bottom + "px";
  focusedCoordinates.value.left = rect.left + "px";
};

const concertRefs = useTemplateRefsList<InstanceType<typeof ConcertCard>>();
const focusConcert = (index: number) => {
  if (focus.value) {
    nextFocus.value = index;
  } else {
    const element = concertRefs.value[index].$el as HTMLDivElement;

    updateFocusedSource(element);
    focus.value = { index, element, leaving: false };
  }
};

watch(focus, (focus) => {
  isLocked.value = focus != null;

  if (focus === null && nextFocus.value != null) {
    const index = nextFocus.value;
    nextFocus.value = null;
    focusConcert(index);
  }
});

useEventListener("resize", () => {
  if (focus.value) {
    updateFocusedSource(focus.value.element);
  }
});

const concerts: Array<Concert> = [
  {
    date: "25 janvier 2025",
    location: "Mairie du XIV<sup>ème</sup>",
    image: "/images/concerts/2025-01-25.png",
    url: {
      kind: "Tickets Unavailable",
      target: undefined,
    },
  },
  {
    date: "7 décembre 2024",
    location: "Pavillon de la Sirène",
    image: "/images/concerts/2024-12-07.png",
    url: {
      kind: "Tickets",
      target:
        "https://www.helloasso.com/associations/tangram/evenements/tangram-concert-a-la-sirene-le-7-12-2024",
    },
  },
  {
    date: "28 juin 2024",
    location: "Temple Paris Batignolles",
    image: "/images/concerts/2024-06-28.png",
    url: {
      kind: "Video",
      target:
        "https://www.youtube.com/watch?v=bRKj0I16uIs&list=PLaKZ2HpaR66HlhepL8kLuCIE-NtkGLos0",
    },
    program: [
      { name: "FFXIV - Endcaller", direction: "Guillaume" },
      { name: "Undertale", direction: "Selma / Théophile" },
      { name: "Vaïana", direction: "Zélie" },
      { name: "Medley James Newton Howard", direction: "Ryadh" },
      { name: "Pirate des Caraïbes", direction: "Rindra / Korantin" },
      { name: "Peter Pan", direction: "Cédric" },
      { name: "Pelléas et Mélisande", direction: "Anice" },
      { name: "Medley ABBA", direction: "Max" },
      { name: "Peau d'Âne", direction: "Simon" },
      { name: "Le Château dans le Ciel", direction: "Yumi / Rindra" },
      { name: "Le Roi Lion", direction: "Arthur" },
      { name: "OSS 117", direction: "Alexis" },
    ],
  },
  {
    date: "28 janvier 2024",
    location: "Mairie du XIV<sup>ème</sup>",
    image: "/images/concerts/2024-01-24.png",
    program: [
      { name: "The Lord of the Rings", direction: "Arthur" },
      { name: "Alice au pays des merveilles", direction: "Cédric" },
      { name: "Pokémon", direction: "Etienne" },
      { name: "Narnia - The Battle", direction: "Ryadh" },
      { name: "Hollow knight", direction: "Noémie" },
      { name: "Kung Fu Panda", direction: "Simon" },
      { name: "Gladiator", direction: "Rindra" },
      { name: "Unbreakable", direction: "Gaétan" },
      { name: "Violet Evergarden", direction: "Intila" },
      { name: "Titina", direction: "Yumi" },
      { name: "Hyper-espace", direction: "Anice" },
      { name: "Lord of the Dance", direction: "Anice" },
    ],
  },
  {
    date: "15 décembre 2023",
    location: "MPAA Saint Germain",
    image: "/images/concerts/2023-12-15.png",
    url: {
      kind: "Video",
      target: "https://www.youtube.com/watch?v=aLhweV3tKvc",
    },
    program: [
      { name: "The Lord of the Rings", direction: "Arthur" },
      { name: "Alice au pays des merveilles", direction: "Cédric" },
      { name: "Pokémon", direction: "Etienne" },
      { name: "Narnia - The Battle", direction: "Ryadh" },
      { name: "Hollow knight", direction: "Noémie" },
      { name: "Kung Fu Panda", direction: "Simon" },
      { name: "Gladiator", direction: "Rindra" },
      { name: "Unbreakable", direction: "Gaétan" },
      { name: "Titina", direction: "Yumi" },
      { name: "Hyper-espace", direction: "Anice" },
    ],
  },
  {
    date: "29 juin 2023",
    location: "Lycée Rodin",
    image: "/images/concerts/2023-06-29.png",
    url: {
      kind: "Video",
      target: "https://www.youtube.com/watch?v=2NE-AwF7v2A",
    },
    program: [
      { name: "Chicken Run", direction: "Simon" },
      { name: "Nausicaa", direction: "Cédric" },
      { name: "Octopath Traveler", direction: "Intila" },
      { name: "Cowboy Bebop", direction: "Anice" },
      { name: "Across the stars", direction: "Thibaud" },
      { name: "Narnia", direction: "Noémie" },
      { name: "Le Parrain", direction: "Yumi" },
      { name: "Sinbad", direction: "Ryadh" },
      { name: "Star Trek", direction: "Luc" },
      { name: "Legends Never Die", direction: "Arthur" },
    ],
  },
  {
    date: "21 janvier 2023",
    location: "Maison du Japon",
    image: "/images/concerts/2022-01-21.png",
    program: [
      { name: "Cinema Paradiso", direction: "Simon" },
      { name: "Doctor Who", direction: "Amaury" },
      { name: "How to train your dragon", direction: "Gaetan" },
      { name: "Kiki", direction: "Thibaud" },
      { name: "Le vent se lève", direction: "Cédric" },
      { name: "Lord of the Dance", direction: "Emilie" },
      { name: "Star Wars VII", direction: "Tanguy / Rindra" },
      { name: "Cowboy Bebop", direction: "Yumi" },
      { name: "Tintin", direction: "Arthur" },
      { name: "Final Fantasy X", direction: "Arthur" },
    ],
  },
];
</script>

<template>
  <div ref="el" class="mx-auto max-w-screen-2xl px-8 py-16">
    <h1 class="oswald text-center text-6xl tracking-wider sm:text-7xl">
      NOS CONCERTS
    </h1>

    <Transition>
      <Teleport to="body">
        <div
          v-if="focus && !focus.leaving"
          class="backdrop fixed z-10 size-full"
          @click="focus.leaving = true"
        />
      </Teleport>
    </Transition>
    <Transition name="focus" @after-leave="focus = null">
      <Teleport to="body">
        <div
          v-if="focus && !focus.leaving"
          class="focused-container z-20"
          :class="{ 'small-card': !concerts[focus.index].program }"
          @click="focus.leaving = true"
        >
          <div class="focused-container-inner">
            <ConcertCard
              :concert="concerts[focus.index]"
              class="backface-hidden absolute size-full overflow-hidden"
            />
            <div
              class="backface-hidden back relative size-full rounded-3xl border-4 bg-slate-900 p-4 text-white shadow-xl"
            >
              <div
                class="absolute right-4 top-4 z-20 flex size-12 cursor-pointer flex-col items-center justify-center text-5xl hover:text-gray-300"
              >
                <Icon name="ic:cancel" />
              </div>
              <div
                class="no-scrollbar flex size-full grow flex-col overflow-auto"
              >
                <div
                  v-if="concerts[focus.index].program"
                  class="mb-4 flex h-full flex-col gap-8"
                >
                  <h2 class="lilita-one-regular mt-8 text-5xl">Programme</h2>
                  <ul class="raleway flex grow flex-col">
                    <div
                      v-for="program in concerts[focus.index].program"
                      :key="program.name"
                      class="my-3"
                    >
                      <p class="text-3xl">{{ program.name }}</p>
                      <p v-if="program.direction">
                        Dirigé par {{ program.direction }}
                      </p>
                    </div>
                  </ul>
                </div>
                <div v-else class="flex h-full flex-col justify-center">
                  <h2 class="lilita-one-regular text-4xl sm:text-5xl">
                    Programme à venir...
                  </h2>
                </div>
              </div>
            </div>
          </div>
        </div>
      </Teleport>
    </Transition>

    <div
      class="mt-12 grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
    >
      <ConcertCard
        v-for="(concert, index) in concerts.slice(0, displayedConcertCount + 2)"
        v-show="index < displayedConcertCount"
        :ref="concertRefs.set"
        :key="concert.date"
        :concert="concert"
        :class="{
          'opacity-0 transition-none': index === focus?.index,
        }"
        class="cursor-pointer"
        @click="focusConcert(index)"
      />

      <div
        v-if="displayedConcertCount < concerts.length"
        class="lilita-one-regular flex min-h-44 flex-col items-center justify-center text-center text-[5rem]"
      >
        <button
          class="hover:slate-800 flex size-32 items-center justify-center gap-2 rounded-full bg-slate-900 text-white hover:bg-slate-950"
          aria-label="Montrer plus de concerts"
          @click="
            displayedConcertCount += 2;
            displayedConcertManuallyChanged = true;
          "
        >
          <div
            v-for="i in [0, 1, 2]"
            :key="i"
            class="size-4 rounded-full bg-white"
          />
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.no-scrollbar::-webkit-scrollbar {
  display: none;
}

/* Hide scrollbar for IE, Edge and Firefox */
.no-scrollbar {
  -ms-overflow-style: none; /* IE and Edge */
  scrollbar-width: none; /* Firefox */
}

.focused-container {
  position: fixed;
  perspective: 1000px;
  top: 10%;
  right: 0;
  bottom: 10%;
  left: 0;
  max-width: 600px;
  margin: 0 auto;
}

.small-card {
  top: 40%;
  bottom: 40%;
}

@media (max-width: 600px) {
  .focused-container {
    right: 5%;
    left: 5%;
  }
}

.focused-container-inner {
  position: relative;
  width: 100%;
  height: 100%;
  text-align: center;
  transition: transform 0.8s;
  transform-style: preserve-3d;
}

.focused-container .focused-container-inner {
  transform: rotateY(180deg);
}

/* Style the back side */
.back {
  transform: rotateY(180deg);
}

.backface-hidden {
  *,
  & {
    transform-style: preserve-3d;
    backface-visibility: hidden;
  }
}

$enter-animate-time: 600ms;
$leave-animate-time: 400ms;
$flip-ratio: 0.7;
$enter-flip-time: $enter-animate-time * $flip-ratio;
$leave-flip-time: $leave-animate-time * $flip-ratio;
$flip-reverse-delay: $leave-animate-time - $leave-flip-time;

.focus-enter-active {
  transition: all $enter-animate-time ease-in-out;
}
.focus-leave-active {
  transition: all $leave-animate-time ease-in-out;
}
.focus-enter-active.focused-container .focused-container-inner {
  transition: all $enter-flip-time ease-in-out;
}
.focus-leave-active.focused-container .focused-container-inner {
  transition: all $leave-flip-time $flip-reverse-delay ease-in-out;
}

.focus-enter-from,
.focus-leave-to {
  top: v-bind("focusedCoordinates.top");
  right: v-bind("focusedCoordinates.right");
  bottom: v-bind("focusedCoordinates.bottom");
  left: v-bind("focusedCoordinates.left");

  &.focused-container .focused-container-inner {
    transform: none;
  }
}

.backdrop {
  backdrop-filter: blur(5px);

  &.v-leave-active {
    pointer-events: none;
  }

  &.v-enter-from {
    transition: all $enter-animate-time;
    backdrop-filter: blur(0);
  }
  &.v-leave-to {
    transition: all $leave-animate-time;
    backdrop-filter: blur(0);
  }
}
</style>
