<script lang="ts" setup>
import { computed, watch } from "vue";
import { SnackBarType } from "../snack-bar-type";

const props = defineProps<{
  type: SnackBarType;
  open: boolean;
  timeout?: number;
}>();

const emit = defineEmits<{
  (e: "update:open", value: boolean): void;
}>();

var currentFadeOut: NodeJS.Timeout | undefined;

const color = computed(() => {
  switch (props.type) {
    case SnackBarType.error:
      return "red-400";
    case SnackBarType.success:
      return "green-500";
    default:
      return "white";
  }
});

watch(
  () => props.open,
  (newState) => {
    if (newState) {
      clearTimeout(currentFadeOut);
      currentFadeOut = setTimeout(
        () => {
          emit("update:open", false);
        },
        props.timeout ?? props.type == SnackBarType.error ? 5000 : 2500,
      );
    }
  },
);
</script>

<template>
  <Teleport to="#snackbar">
    <Transition>
      <div class="m-1 flex" v-if="open">
        <div class="rounded bg-neutral-800 p-2" :class="`ring-2 ring-${color}`">
          <p class="text-center" :class="'text-' + color">
            <slot></slot>
          </p>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style>
.v-enter-active,
.v-leave-active {
  transition: opacity 0.5s ease;
}

.v-enter-from {
  scale: 105%;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}

.v-enter-active {
  animation: bounce-in 0.8s;
}
.v-leave-active {
  animation: shrink-out 0.8s;
}

@keyframes bounce-in {
  0% {
    transform: scale(0);
  }
  50% {
    transform: scale(1.05);
  }
  100% {
    transform: scale(1);
  }
}

@keyframes shrink-out {
  0% {
    transform: scale(1);
  }
  100% {
    transform: scale(0);
  }
}
</style>
