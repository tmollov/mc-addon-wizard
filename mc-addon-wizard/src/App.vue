<template>
  <div class="min-h-screen flex flex-col bg-neutral-950 text-white">
    <Header />

    <!-- Main Content -->
    <main class="flex-1 mt-16">
      <RouterView />
    </main>

    <Footer />
  </div>
</template>

<script setup lang="ts">
import Footer from './components/Footer.vue';
import Header from './components/Header.vue';

</script>
