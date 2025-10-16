<template>
    <aside class="flex flex-col bg-gray-800 text-white w-64 min-h-screen p-4">
        <div class="text-2xl font-bold mb-6">Categories</div>
        <nav class="flex-1">
            <ul>
                <li v-for="item in items" :key="item.id" class="mb-2">
                    <button class="btn w-full in-active" :class="activeId === item.id ? 'active' : ''"
                        @click="activeId = item.id">{{
                            item.name
                        }}</button>
                </li>
            </ul>
        </nav>
        <div class="mt-auto text-sm text-gray-400">
            Version 0.1.0
        </div>
    </aside>
</template>

<script setup lang="ts">
import { ref } from 'vue'
const items = [
    { id: 1, name: 'One' },
    { id: 2, name: 'Two' },
    { id: 3, name: 'Three' }
]

const activeId = ref(0);

</script>

<style scoped>
/* optional: custom scrollbar */
aside::-webkit-scrollbar {
    width: 6px;
}

aside::-webkit-scrollbar-thumb {
    background-color: rgba(255, 255, 255, 0.2);
    border-radius: 3px;
}

button {
    background-color: #626262;
}

.active {
    background-color: #C5C5C5;
}
</style>
