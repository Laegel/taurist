<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { emit, listen } from '@tauri-apps/api/event'

let unlistener

onMounted(async () => {
  // listen to the `click` event and get a function to remove the event listener
  // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
  unlistener = await listen('pong', (event) => {
    console.log('received pong event!')

    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
  })

  // emits the `click` event with the object payload
  emit('ping', {
    theMessage: 'Tauri is awesome!',
  })
})

onBeforeUnmount(() => {
  unlistener()
})
</script>

<template>Events</template>
