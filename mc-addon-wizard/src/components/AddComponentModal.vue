<!-- AddComponentModal.vue -->
<template>
  <div v-if="open" class="fixed inset-0 bg-black/60 flex items-center justify-center z-50">
    <div class="bg-gray-800 rounded-2xl p-6 w-full max-w-lg space-y-4" style="height: 70vh;">
      <div class="flex justify-between items-center">
        <h2 class="text-xl font-semibold">Add Component</h2>
        <button @click="close" class="text-gray-400 hover:text-white">✕</button>
      </div>

      <input v-model="searchQuery" placeholder="Search component..."
        class="w-full px-3 py-2 rounded bg-gray-900 text-white border border-gray-700" />

      <!-- Category buttons -->
      <div class="flex flex-wrap gap-2 mb-4">
        <button v-for="cat in categories" :key="cat" @click="selectedCategory = cat" :class="[
          'px-3 py-1 rounded-md transition-all cursor-pointer',
          selectedCategory === cat
            ? 'bg-blue-600 text-white'
            : 'bg-gray-700 text-gray-300 hover:bg-gray-600',
        ]">
          {{ cat }}
        </button>
      </div>


      <div class="max-h-115 overflow-y-auto mt-3 space-y-1">
        <button v-for="comp in filteredComponents" :key="comp.key" @click="select(comp)"
          class="w-full text-left px-3 py-2 rounded hover:bg-gray-700 transition">
          <span class="font-medium text-lime-400">{{ comp.key }}</span>
          <p class="text-sm text-gray-400">{{ comp.desc }}</p>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed, ref, defineModel } from 'vue'
import json from '/src/json-templates/item_components.json';

const props = defineProps({
  open: Boolean,
  components: Array
})
const emits = defineEmits(['close', 'add'])

const searchQuery = ref("");
const categories = ["All", "Boolean", "Number", "String", "Object"]; // Array
const selectedCategory = ref("All");

const filteredComponents = computed(() => {
  let list = json;

  if (selectedCategory.value !== "All") {
    list = list.filter((c) => {
      switch (selectedCategory.value) {
        case "Boolean":
          return c.type === "boolean";
        case "Number":
          return c.type === "number";
        case "String":
          return c.type === "string";
        case "Object":
          return c.type === "object";
        default: return true;
      }
    });
  }

  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase();
    list = list.filter((c) => c.key.toLowerCase().includes(query));
  }

  return list;
});

const close = () => emits('close')
const select = (comp) => emits('add', comp)

</script>
