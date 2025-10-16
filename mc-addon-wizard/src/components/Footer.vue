<template>
  <footer class="bg-neutral-900 text-neutral-400 border-t border-neutral-800">
    <div class="container mx-auto px-6 py-6 flex flex-col md:flex-row items-center justify-between text-sm">
      <p class="mb-2 md:mb-0">
        © {{ new Date().getFullYear() }} MC Addon Wizard. All rights reserved.
      </p>

      <div class="flex gap-4">
        <RouterLink to="/privacy" class="hover:text-green-400 transition-colors">Privacy</RouterLink>
        <RouterLink to="/terms" class="hover:text-green-400 transition-colors">Terms</RouterLink>
        <a href="https://github.com" target="_blank" class="hover:text-green-400 transition-colors">GitHub</a>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
// Static footer for now
</script>
