<script setup lang="ts">
import LayoutMain from './components/LayoutMain.vue'
import LayoutPreview from './components/LayoutPreview.vue'

const isPreview = !!location.hash
const previewRoute = location.hash.split('-')

</script>

<template>
  <LayoutPreview v-if="isPreview" v-bind:feature="previewRoute[1]" />
  <LayoutMain v-else />
</template>

<style>
:root {
  font-family: 'Rubik', sans-serif;
  background-color: rgb(27, 27, 29);
  color: rgb(218, 221, 225);
}
</style>
