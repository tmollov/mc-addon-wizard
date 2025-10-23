<template>
  <div class="min-h-screen bg-gray-950 text-gray-100 p-6">
    <!-- Page title -->
    <h1 class="text-3xl font-bold mb-8">Add New Element</h1>

    <!-- Category sections -->
    <div v-for="(category, i) in categories" :key="i" class="mb-10">
      <!-- Category header -->
      <div class="flex items-center mb-4">
        <h2 class="text-xl font-semibold text-emerald-400">{{ category.name }}</h2>
        <div class="flex-grow border-b border-gray-700 ml-4"></div>
      </div>

      <!-- Cards grid -->
      <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        <div v-for="(item, j) in category.items" :key="j" @click="goToCreator(item.route)"
          class="group cursor-pointer bg-gray-900 border border-gray-800 hover:border-emerald-500 rounded-2xl p-5 transition-all hover:-translate-y-1 hover:shadow-lg">
          <div class="flex items-center justify-between mb-3">
            <!-- Icon -->
            <div class="p-2 bg-gray-800 rounded-xl group-hover:bg-emerald-500 transition-all">
              <component :is="item.icon" class="w-6 h-6 text-emerald-400 group-hover:text-white" />
            </div>
          </div>

          <!-- Title & Description -->
          <h3 class="text-lg font-semibold mb-1">{{ item.title }}</h3>
          <p class="text-sm text-gray-400">{{ item.desc }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router';
import { ImageIcon, BoxIcon, MousePointerClickIcon } from 'lucide-vue-next';

const router = useRouter()

const goToCreator = (route) => {
  router.push(route)
}

// Example data — you can expand or load dynamically later
const categories = [
  {
    name: 'Items',
    items: [
      { title: '2D Item', desc: 'Basic flat item, like a sword or tool.', icon: ImageIcon, route: '/add/item/2d' },
      { title: '3D Item', desc: 'Custom modeled item (uses geometry).', icon: BoxIcon, route: '/create/item-3d' },
    ],
  },
  {
    name: 'Entities',
    items: [
      { title: 'Mob', desc: 'A living creature, with AI and animations.', icon: BoxIcon, route: '/create/entity-mob' },
      { title: 'NPC', desc: 'A static or interactive non-player character.', icon: MousePointerClickIcon, route: '/create/entity-npc' },
    ],
  },
]
</script>
