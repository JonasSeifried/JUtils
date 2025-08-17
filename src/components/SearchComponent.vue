<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from "vue";

export type SearchItem = {
  id: string;
  displayName: string;
};

const props = defineProps<{
  items: SearchItem[];
}>();

const emit = defineEmits<{
  (e: "item-selected", value: SearchItem): void;
}>();

const searchTerm = ref("");
const filteredItems = ref<SearchItem[]>(props.items);
const isDropdownOpen = ref(false);
const containerRef = ref<HTMLElement | null>(null);

watch(searchTerm, (newSearchTerm) => {
  if (newSearchTerm) {
    // Filter the items based on the search term (case-insensitive).
    const results = props.items.filter((item) =>
      item.displayName.toLowerCase().includes(newSearchTerm.toLowerCase())
    );
    filteredItems.value = results;
    // Open the dropdown if there's a search term.
  } else {
    // If the search term is empty, clear the filtered items and close the dropdown.
    filteredItems.value = props.items;
  }
});

const handleItemClick = (item: SearchItem) => {
  searchTerm.value = item.displayName;
  isDropdownOpen.value = false;
  emit("item-selected", item);
};

const handleClickOutside = (event: MouseEvent) => {
  if (
    containerRef.value &&
    !containerRef.value.contains(event.target as Node)
  ) {
    isDropdownOpen.value = false;
  }
};

onMounted(() => {
  document.addEventListener("mousedown", handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener("mousedown", handleClickOutside);
});
</script>

<template>
  <div class="flex items-center justify-center">
    <div ref="containerRef" class="w-full max-w-md relative">
      <!-- Search input container -->
      <div class="relative">
        <input
          type="text"
          placeholder="Search for an app..."
          class="ring-2 ring-primary bg-neutral-800 rounded-lg focus:outline-none w-full p-2 transition-all duration-200 font-inter text-text focus-within:hover:scale-105"
          v-model="searchTerm"
          @focus="isDropdownOpen = true"
        />
      </div>

      <!-- Dropdown container -->
      <div
        v-if="isDropdownOpen && filteredItems.length > 0"
        class="absolute top-full w-full mt-0.5 bg-background-light border-2 border-t-0 b border-primary rounded-lg shadow-xl z-10 transition-all duration-300 transform opacity-0 animate-fade-in-up"
      >
        <ul
          scroll="no"
          class="py-2 max-h-64 z overflow-y-auto overflow-ellipsis overflow-x-hidden"
        >
          <li
            v-for="(item, index) in filteredItems"
            :key="index"
            @click="handleItemClick(item)"
            class="px-4 py-2 cursor-pointer hover:bg-background transition-colors duration-150 font-inter w-inherit text-text"
          >
            {{ item.displayName }}
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<style scoped>
.animate-fade-in-up {
  animation: moveUp 0.3s ease-out forwards, fadeIn 0.5s ease-in-out forwards;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}
@keyframes moveUp {
  from {
    transform: translateY(-50px) scale(0.75);
  }
  to {
    transform: translateY(0) scale(1);
  }
}
</style>
