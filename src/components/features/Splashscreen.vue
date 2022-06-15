<script setup lang="ts">
import { onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const openSplashscreen = () => invoke('open_splashscreen')

onMounted(() => {
  openSplashscreen()
})
</script>

<template>
  <button @click="openSplashscreen">Trigger splashscreen</button>
</template>

<style></style>
