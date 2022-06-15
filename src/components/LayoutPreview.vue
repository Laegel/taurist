<script setup lang="ts">
import { ref, onMounted } from 'vue'

import { Commands, Events, Multiwindow, Splashscreen, SystemTray, WindowCustomization } from './features'

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
  { key: 'multiwindow', label: 'Multiwindow', component: Multiwindow },
  { key: 'splashscreen', label: 'Splashscreen', component: Splashscreen },
  { key: 'systemtray', label: 'System Tray', component: SystemTray },
  { key: 'window', label: 'Window Customization', component: WindowCustomization },
]
</script>

<template>
  <div class="preview">
    <component v-bind:is="features.find(ft => ft.key === feature).component" />
  </div>
</template>

