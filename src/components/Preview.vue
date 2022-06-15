<script setup lang="ts">
import { ref, onMounted } from 'vue'

import { Commands, Events, WindowCustomization } from './features'

const { feature } = defineProps(['feature'])
const cpt = {
  render() {
    return 'nope'
  },
}
const features = [
  { key: '', label: 'Making Your Own CLI', component: cpt },
  { key: '', label: 'Calling Rust from the frontend', component: Commands },
  { key: '', label: 'Events', component: Events },
  { key: 'menu', label: 'Window Menu', component: cpt },
  { key: '', label: 'Multiwindow', component: cpt },
  { key: '', label: 'Tauri Plugins', component: cpt },
  { key: '', label: 'Splashscreen', component: cpt },
  { key: '', label: 'System Tray', component: cpt },
  {
    key: 'window',
    label: 'Window Customization',
    component: WindowCustomization,
  },
]
</script>

<template>
  <div class="preview">
    <component v-bind:is="features.find(ft => ft.key === feature).component" />
  </div>
</template>

<style>
.preview {
  border: 1px solid #606770;
  margin: 10px;
  border-radius: 2px;
}
</style>
