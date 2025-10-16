<template>
    <div class="p-4">
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
            <div v-for="element in elements" :key="element.id"
                class="bg-black-100 p-3 rounded shadow hover:shadow-lg cursor-pointer flex flex-col items-center">
                <img :src="element.icon" alt="" class="w-16 h-16 mb-2" />
                <div class="text-sm font-medium text-center">{{ element.name }}</div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
interface Element {
    id: number;
    name: string;
    icon: string;
}

const elements: Element[] = [
    { id: 1, name: "Stone Block", icon: "/icons/stone.png" },
    { id: 2, name: "Diamond Sword", icon: "/icons/diamond_sword.png" },
    { id: 3, name: "Creeper", icon: "/icons/creeper.png" },
    // add more
];
</script>
