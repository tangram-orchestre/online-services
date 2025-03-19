import { ref } from "vue";

const current = ref(0);
const elements = new Array<Ref<HTMLElement | null>>();
const states = new Array<boolean>();

function update() {
  for (let i = states.length - 1; i >= 0; i--) {
    if (states[i]) {
      current.value = i;
      break;
    }
  }
}

export function useNavBarSection(target: Ref<HTMLElement | null>) {
  elements.push(target);
  states.push(false);
  const index = elements.length - 1;

  onMounted(() => {
    useIntersectionObserver(
      target,
      ([entry], _observerElement) => {
        const visible = entry?.isIntersecting || false;
        states[index] = visible;
        update();
      },
      { rootMargin: "-50% 0px" },
    );
  });
}

export function useNavBar() {
  const { arrivedState } = useScroll(document);

  // When we reach bottom of page, consider that the last section is focused,
  // even if it's not detected by intersectionObserver (happens when section is
  // smaller than 50% of the screen).
  watch(arrivedState, (val) => {
    states[states.length - 1] = val.bottom;
    update();
  });

  return {
    current,
    set: (index: number) => {
      const elementBoundingRect =
        elements[index].value?.getBoundingClientRect();
      if (elementBoundingRect) {
        let yTarget = elementBoundingRect.top + window.scrollY;

        // Ensure the detection zone will be inside the section
        const yDetection = yTarget + window.innerHeight / 2;
        const yEndOfSection = yTarget + elementBoundingRect.height;
        const yOffset = Math.max(yDetection - yEndOfSection + 10, 62);
        yTarget -= yOffset;

        // Special handling for penultimate element
        // If yTarget would cause the window to hit bottom of the page, avoid it
        // to not trigger last section detection (see above).
        if (index == states.length - 2) {
          const maxYScroll =
            window.document.documentElement.scrollHeight - window.innerHeight;
          if (yTarget > maxYScroll) {
            yTarget = maxYScroll - 100;
          }
        }

        window.scrollTo({ top: yTarget, behavior: "smooth" });
      }
    },
  };
}
