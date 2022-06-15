<script setup lang="ts">
import { onBeforeUnmount, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import * as monaco from 'monaco-editor'

const editorOptions = {
  minimap: { enabled: false },
  theme: 'vs-dark',
}

onMounted(() => {
  invoke('open_preview_window', { feature: 'editor' })

  monaco.editor.create(document.getElementById('editor-rust'), {
    ...editorOptions,
    model: monaco.editor.createModel(
      'use tauri:: Manager;\n\n// the payload type must implement `Serialize` and `Clone`.\n#[derive(Clone, serde:: Serialize)]\nstruct Payload {\n    message: String,\n}\n\nfn main() {\n    tauri:: Builder::default ()\n        .setup(| app | {\n            // listen to the `event-name` (emitted on any window)\n            let id = app.listen_global("event-name", | event | {\n                println!("got event-name with payload {:?}", event.payload());\n        });\n    // unlisten to the event using the `id` returned on the `listen_global` function\n    // an `once_global` API is also exposed on the `App` struct\n    app.unlisten(id);\n\n    // emit the `event-name` event to all webview windows on the frontend\n    app.emit_all("event-name", Payload { message: "Tauri is awesome!".into() }).unwrap();\n    Ok(())\n})\n    .run(tauri:: generate_context!())\n    .expect("failed to run app");\n}',
      'rust'
    ),
  })
  monaco.editor.create(document.getElementById('editor-js'), {
    ...editorOptions,
    model: monaco.editor.createModel(
      "import { emit, listen } from '@tauri-apps/api/event'\n\n// listen to the `click` event and get a function to remove the event listener\n// there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event\nconst unlisten = await listen('click', event => {\n  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)\n  // event.payload is the payload object\n})\n\n// emits the `click` event with the object payload\nemit('click', {\n  theMessage: 'Tauri is awesome!'\n})",
      'typescript'
    ),
  })
})

onBeforeUnmount(() => {
  invoke('close_preview_window')
})
</script>

<template>
  <div>Run</div>
  <div class="editors">
    <div class="editor" id="editor-rust"></div>
    <div class="editor" id="editor-js"></div>
  </div>
</template>

<style>
.editors {
  height: 100vh;
  display: flex;
}
.editor {
  width: 50%;
}
</style>
