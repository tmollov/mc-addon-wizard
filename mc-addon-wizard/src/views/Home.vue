<script setup lang="ts">
import ElementsGrid from '../components/ElementsGrid.vue';
import SideBar from '../components/SideBar.vue';

</script>

<!-- <script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg.value = await invoke("greet", { name: name.value });
}
</script> -->

<template>
    <div class="flex">
        <SideBar />
        <main class="flex-1 bg-gray-50">
            <ElementsGrid />
        </main>
    </div>
</template>