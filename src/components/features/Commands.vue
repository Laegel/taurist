<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

onMounted(() => {
  invoke('my_custom_command', {
    number: 42,
  })
    .then((res) =>
      console.log(`Message: ${res.message}, Other Val: ${res.other_val}`)
    )
    .catch((e) => console.error(e))
})
</script>

<template>
Commands
</template>