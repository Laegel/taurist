<script setup lang="ts">
import { onMounted, shallowRef } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const openWindows = shallowRef([])

const openMultiwindow = (key) => invoke('open_multiwindow').then(() => openWindows.value.push(key))


</script>

<template>
  <button @click="openMultiwindow(1)">Open Multiwindow 1</button>
</template>

<style></style>
