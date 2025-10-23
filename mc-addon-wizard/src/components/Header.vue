<template>
    <header class="fixed top-0 left-0 w-full h-16 bg-gray-900 text-white shadow-md z-50">
        <nav class="max-w-7xl mx-auto flex items-center justify-between px-6 py-3">
            <!-- Logo -->
            <div class="flex items-center flex-4 gap-2">
                <router-link to="/" class="flex items-center hover:text-green-400 transition">
                    <img src="/icon.png" alt="MC Addon Wizard" class="w-8 h-8 mr-2" />
                    <span class="text-lg font-semibold tracking-wide">Add-On Wizard</span>
                </router-link>
            </div>


            <!-- Action Button -->
            <div class="flex flex-4 items-center">
                <div class="mx-auto">
                    <router-link to="/add" custom v-slot="{ navigate }">
                        <button @click="navigate" class="minecraft-green btn text-black font-bold rounded-xl">
                            Add Element
                        </button>
                    </router-link>
                </div>
            </div>

            <!-- Navigation Links -->
            <div class="flex flex-4 flex-row-reverse items-end">
                <ul class="items-center gap-8 text-sm font-medium">
                    <router-link to="/settings" class=" hover:text-green-400 transition">Settings</router-link>
                </ul>
            </div>

        </nav>
    </header>
</template>

<style scoped>
.minecraft-green {
    background-color: #6BBE44;
}
</style>

<script setup lang="ts">
// no logic yet
</script>
