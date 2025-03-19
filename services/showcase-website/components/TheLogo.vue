<!-- eslint-disable tailwindcss/no-custom-classname -->
<template>
  <div class="outer mx-auto flex items-center justify-center">
    <div class="inner shape-logo overflow-visible">
      <div class="big-triangle-1"><div /></div>
      <div class="square"><div /></div>
      <div class="big-triangle-2"><div /></div>
      <div class="small-triangle-2"><div /></div>
      <div class="parallelogram"><div /></div>
      <div class="medium-triangle"><div /></div>
      <div class="small-triangle-1"><div /></div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.outer {
  width: 100%;
  aspect-ratio: 1.2 / 1;
  max-height: 100%;

  // Filter disables transform-style so apply on parent
  filter: drop-shadow(0 0 0.2em rgba(0, 0, 0, 0.3));
}

.inner {
  position: relative;
  max-width: 54%;
  max-height: 65%;
  height: 100%;
  aspect-ratio: 1 / 1;
  transform-style: preserve-3d;
}

.inner > div {
  position: absolute;
  transform-style: preserve-3d;

  width: 100%;
  height: 100%;
}

.inner > div > div {
  position: absolute;

  width: 100%;
  height: 100%;
}

.big-triangle-1 {
  & > div {
    background-color: #6d9f02;
    clip-path: polygon(0 100%, 100% 100%, 50% 50%);
    transform-origin: 50% 75%;
  }
}
.square {
  & > div {
    background-color: #81ccb5;
    clip-path: polygon(50% 50%, 25% 25%, 50% 0%, 75% 25%);
    transform-origin: 50% 25%;
  }
}
.big-triangle-2 {
  & > div {
    background-color: #ffc303;
    clip-path: polygon(0 0, 0 100%, 50% 50%);
    transform-origin: 25% 50%;
  }
}
.small-triangle-2 {
  & > div {
    background-color: #ff81ac;
    clip-path: polygon(0 0, 50% 0%, 25% 25%);
    transform-origin: 25% 12.5%;
  }
}
.parallelogram {
  & > div {
    background-color: #ff5b62;
    clip-path: polygon(100% 100%, 75% 75%, 75% 25%, 100% 50%);
    transform-origin: 87.5% 62.5%;
  }
}
.medium-triangle {
  & > div {
    background-color: #ffc3cb;
    clip-path: polygon(50% 0, 100% 0, 100% 50%);
    transform-origin: 87.5% 12.5%;
  }
}
.small-triangle-1 {
  & > div {
    background-color: #fe9225;
    clip-path: polygon(75% 25%, 50% 50%, 75% 75%);
    transform-origin: 62.5% 50%;
  }
}

@mixin anim($list) {
  $time_offset: 0;
  @each $name, $t in $list {
    @include kf($name, $t, $time_offset);
    $time_offset: $time_offset + 0.15;
  }
}

@mixin kf($name, $transform, $time-offset) {
  $offset: translate(0, 42%);
  $animation-length: 0.8s;

  .#{$name} > div {
    transform: $offset;
  }

  .shape-logo {
    $shadow: drop-shadow(0 0 10px rgba(0, 0, 0, 0.3));
    $perspective: translateZ(100px + $time-offset) scale(1.1);

    .animate & .#{$name} > div {
      animation: #{$name}-animation $animation-length ease-in-out forwards;
      animation-delay: $time-offset + s;
    }
    .skip-animation & .#{$name} > div {
      animation-delay: 0ms;
      animation-duration: 500ms;
    }

    @keyframes #{$name}-animation {
      from {
        transform: $offset;
      }
      30% {
        transform: $offset $perspective;
      }
      80% {
        transform: $transform $perspective;
      }
      to {
        transform: $transform;
      }
    }

    .animate & .#{$name} {
      animation: #{$name}-animation-filter $animation-length ease-in-out
        forwards;
      animation-delay: $time-offset + s;
    }

    .skip-animation & .#{$name} {
      animation-delay: 0ms;
      animation-duration: 500ms;
    }

    @keyframes #{$name}-animation-filter {
      30% {
        filter: $shadow;
        transform: translateZ(100px - $time-offset);
      }
      80% {
        filter: $shadow;
        transform: translateZ(100px - $time-offset);
      }
    }
  }
}

@include anim(
  (
    small-triangle-2: translate(88.5%, 22.5%) rotate(90deg),
    big-triangle-2: translate(-15%, -50%) rotate(90deg),
    medium-triangle: translate(13.5%, 5%) rotate(90deg + 45deg),
    small-triangle-1: translate(-52.5%, -12.5%) rotate(90deg),
    square: translate(5%, 10%) rotate(45deg),
    parallelogram: translate(1%, -15%),
    big-triangle-1: translate(5%, -4.6%) rotate(45deg),
  )
);
</style>
