<template>
  <div class="p-6 space-y-6">
    <h1 class="text-2xl font-bold">Create 2D Item</h1>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <!-- Left: form -->
      <div class=" space-y-4 bg-gray-800 p-4 rounded-2xl">
        <!-- Identifier -->
        <div>
          <label class="block text-sm text-gray-300 mb-1">Identifier</label>
          <input v-model="form.identifier" placeholder="my:item_name"
            class="w-full px-3 py-2 rounded bg-gray-900 text-white border border-gray-700" />
        </div>

        <!-- Category -->
        <div>
          <label class="block text-sm text-gray-300 mb-1">Category</label>
          <select v-model="form.category"
            class="w-full px-3 py-2 rounded bg-gray-900 text-white border border-gray-700">
            <option value="Items">Items</option>
            <option value="Equipment">Equipment</option>
            <option value="Nature">Nature</option>
            <option value="Construction">Construction</option>
          </select>
        </div>

        <!-- Icon -->
        <div>
          <label class="block text-sm text-gray-300 mb-1">Icon Path</label>
          <input v-model="form.icon" placeholder="textures/items/my_item"
            class="w-full px-3 py-2 rounded bg-gray-900 text-white border border-gray-700" />
        </div>

        <!-- Components -->
        <div class="pt-3 border-t border-gray-700">
          <h3 class="font-semibold text-gray-200 mb-2">Components</h3>

          <button @click="showModal = true"
            class="bg-lime-600 hover:bg-lime-700 text-white px-3 py-2 rounded-lg text-sm">
            + Add Component
          </button>

          <div v-if="addedComponents.length" class="mt-3 space-y-2">
            <div v-for="comp in addedComponents" :key="comp.key" class="bg-gray-700 p-2 rounded-lg">
              <div class="flex justify-between items-center">
                <span class="font-medium text-lime-400">{{ comp.key }}</span>
                <button @click="removeComponent(comp.key)"
                  class="text-gray-400 hover:text-white text-sm">Remove</button>
              </div>

              <!-- basic inputs for simple types -->
              <div v-if="comp.type === 'boolean'" class="mt-1 flex items-center gap-2">
                <input type="checkbox" v-model="comp.value" class="accent-lime-500" />
                <label class="text-sm text-gray-300">Enabled</label>
              </div>

              <div v-if="comp.type === 'number'" class="mt-1">
                <input type="number" v-model.number="comp.value"
                  class="w-full px-2 py-1 rounded bg-gray-900 text-white border border-gray-700" />
              </div>

              <div v-if="comp.type === 'object'" class="mt-1 text-sm text-gray-400">
                <p>⚠ Complex component — editing UI coming soon</p>
              </div>
            </div>
          </div>

          <!-- Modal -->
          <AddComponentModal :open="showModal" @close="showModal = false" @add="addComponent" />
        </div>


      </div>

      <!-- Right: preview -->
      <div class=" bg-gray-900 p-4 rounded-2xl">
        <h2 class="text-xl font-semibold mb-3">JSON Preview</h2>
        <pre class="text-sm bg-gray-800 p-3 rounded overflow-x-auto text-green-400">
{{ generatedJson }}
        </pre>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed, reactive, ref } from 'vue'
import AddComponentModal from '../components/AddComponentModal.vue'
import json from '/src/json-templates/item_components.json';

const showModal = ref(false)
const addedComponents = reactive([])
const form = reactive({
  identifier: 'my:item_name',
  category: 'Items',
  stack_size: 64,
  icon: 'textures/items/my_item',
  components: {},
  nutrition: 4,
  durability: 100
})

const addComponent = (comp) => {
  if (!addedComponents.find(c => c.key === comp.key)) {
    addedComponents.push({ ...comp, value: comp.type === 'boolean' ? true : '' })
  }
  showModal.value = false
}

const removeComponent = (key) => {
  const i = addedComponents.findIndex(c => c.key === key)
  if (i !== -1) addedComponents.splice(i, 1)
}

const generatedJson = () => {
  const base = {
    format_version: '1.20.10',
    'minecraft:item': {
      description: {
        identifier: form.identifier,
        category: form.category,
        icon: form.icon
      },
      components: {
        'minecraft:max_stack_size': form.stack_size
      }
    }
  }
};

</script>
