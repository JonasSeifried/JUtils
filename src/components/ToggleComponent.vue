<script setup lang="ts">
import { onMounted, ref } from "vue";

const props = defineProps<{
  title: string;
  toolTip: string;
  initValue: boolean;
}>();

const emit = defineEmits<{
  (e: "toggle", newValue: boolean): void;
}>();

const checked = ref(false);

function toggle(event: Event) {
  console.log("checked=" + checked.value);

  emit("toggle", (event.target as HTMLInputElement)?.checked);
}

onMounted(() => {
  checked.value = props.initValue;
});
</script>

<template>
  <label
    class="relative flex w-max cursor-pointer select-none items-center"
    :title="props.toolTip"
  >
    <span class="mr-3 text-lg font-bold text-white">{{ props.title }}</span>
    <input
      @change="toggle"
      v-model="checked"
      type="checkbox"
      class="peer h-7 w-14 cursor-pointer appearance-none rounded-full border-2 border-fuchsia-700 bg-neutral-900 transition-colors checked:bg-fuchsia-950"
    />
    <span
      class="absolute right-7 h-7 w-7 transform rounded-full bg-fuchsia-700 transition-transform peer-checked:translate-x-7"
    />
  </label>
</template>
