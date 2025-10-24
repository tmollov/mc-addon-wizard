<!-- /src/views/SettingsView.vue -->
<template>
    <div class="p-6 text-gray-200 space-y-8">
        <h1 class="text-3xl font-bold mb-4">Settings</h1>

        <!-- General Section -->
        <section class="bg-gray-800/60 backdrop-blur-md rounded-2xl p-6 shadow-lg">
            <h2 class="text-xl font-semibold mb-4 text-blue-400">General</h2>
            <div class="grid grid-cols-2 gap-4">
                <div>
                    <label class="block text-sm mb-1">Project Name</label>
                    <input v-model="settings.name" type="text" class="input" placeholder="My Addon" />
                </div>

                <div>
                    <label class="block text-sm mb-1">Identifier</label>
                    <input v-model="settings.identifier" type="text" class="input" placeholder="my_addon" />
                </div>

                <div>
                    <label class="block text-sm mb-1">Author</label>
                    <input v-model="settings.author" type="text" class="input" placeholder="Your name" />
                </div>

                <div>
                    <label class="block text-sm mb-1">Version</label>
                    <input v-model="settings.version" type="text" class="input" placeholder="1.0.0" />
                </div>

                <div class="col-span-2">
                    <label class="block text-sm mb-1">Description</label>
                    <textarea v-model="settings.description" rows="2" class="input"></textarea>
                </div>
            </div>
        </section>

        <!-- App Preferences -->
        <section class="bg-gray-800/60 rounded-2xl p-6 shadow-lg">
            <h2 class="text-xl font-semibold mb-4 text-blue-400">App Preferences</h2>
            <div class="flex flex-col gap-4">

                <div class="flex items-center justify-between">
                    <label class="text-sm">Auto Save</label>
                    <input type="checkbox" v-model="settings.autoSave" class="accent-blue-500 w-5 h-5" />
                </div>

                <div>
                    <label class="text-sm block mb-1">Theme</label>
                    <select v-model="settings.theme" class="input">
                        <option value="system">System</option>
                        <option value="dark">Dark</option>
                        <option value="light">Light</option>
                    </select>
                </div>

                <div>
                    <label class="text-sm block mb-1">Default Export Path</label>
                    <input v-model="settings.exportPath" type="text" class="input" placeholder="~/Documents/Addons/" />
                </div>
            </div>
        </section>

        <div class="flex gap-4">
            <button @click="saveSettings" class="btn-primary cursor-pointer">💾 Save</button>
            <button @click="resetSettings" class="btn-danger cursor-pointer">🗑 Reset</button>
        </div>
    </div>
</template>

<script setup>
import { useSettingsStore } from '../store/settings.js'
import { onMounted } from 'vue'

const settings = useSettingsStore()

const saveSettings = () => settings.save()
const resetSettings = () => settings.reset()

onMounted(() => settings.load())
</script>

<style scoped>
@import 'tailwindcss';

.input {
    @apply bg-gray-700 text-gray-100 rounded-lg w-full px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500;
}

.btn-primary {
    @apply bg-blue-600 hover:bg-blue-700 rounded-lg px-4 py-2 font-semibold transition;
}

.btn-danger {
    @apply bg-red-600 hover:bg-red-700 rounded-lg px-4 py-2 font-semibold transition;
}
</style>
