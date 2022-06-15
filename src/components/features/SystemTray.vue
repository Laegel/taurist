<script setup lang="ts">
import { removeSystemTray } from '@/api/systray'
</script>

<template>
  <button @click="removeSystemTray">Hide systray</button>
</template>

<style></style>
