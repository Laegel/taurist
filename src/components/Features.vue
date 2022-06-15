<script setup lang="ts">
import { shallowRef, onBeforeUnmount } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

import Preview from './Preview.vue'

let feature = shallowRef({ current: null })

const setCurrentFeature = (newFeature) => {
  feature.value = { current: newFeature }

  invoke('open_preview_window', { feature: newFeature.key })
}

const features = [
  { key: '', label: 'Making Your Own CLI' },
  { key: '', label: 'Calling Rust from the frontend' },
  { key: '', label: 'Events' },
  { key: 'menu', label: 'Window Menu' },
  { key: 'multiwindow', label: 'Multiwindow' },
  { key: 'splashscreen', label: 'Splashscreen' },
  { key: 'systemtray', label: 'System Tray' },
  {
    key: 'window',
    label: 'Window Customization',
  },
]

feature.value = { current: features[0] }

onBeforeUnmount(() => {
  invoke('close_preview_window')
})
</script>

<template>
  <div class="features">
    <ul>
      <li v-for="feature in features" v-on:click="setCurrentFeature(feature)">
        {{ feature.label }}
      </li>
    </ul>
    <!-- <Preview>
      <component
        v-if="feature.current"
        v-bind:is="feature.current.component"
      ></component>
    </Preview> -->
  </div>
</template>

<style>
ul li {
  list-style-type: none;
}

/* .features {
  display: flex;
  flex-direction: row;
  height: 100vh;
}

.features > * {
  width: 50%;
  height: 100%;
} */
</style>
