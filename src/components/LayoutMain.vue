<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { appWindow } from '@tauri-apps/api/window'
import { getConfig, setConfig } from '@/api/config'

import SkillPicker from './SkillPicker.vue'
import Challenges from './Challenges.vue'
import Features from './Features.vue'

const views = {
  Features,
  Challenges,
}

const config = ref()
const router = ref('Features')

const setView = (key) => {
  router.value = key
}

onMounted(() => {
  document
    .getElementById('titlebar-minimize')
    .addEventListener('click', () => appWindow.minimize())
  document
    .getElementById('titlebar-maximize')
    .addEventListener('click', () => appWindow.toggleMaximize())
  document
    .getElementById('titlebar-close')
    .addEventListener('click', () => appWindow.close())

  getConfig()
    .then((storedConfig) => {
      config.value = storedConfig
    })
    .catch(() => {
      config.value = {}
    })
})

const setSkillConfig = (skill: string) => {
  setConfig({ skill }).then((storedConfig) => {
    config.value = storedConfig
  })
}
</script>

<template>
  <div data-tauri-drag-region draggable="true" class="titlebar">
    <div class="titlebar-button" id="titlebar-minimize">
      <img
        src="https://api.iconify.design/mdi:window-minimize.svg"
        alt="minimize"
      />
    </div>
    <div class="titlebar-button" id="titlebar-maximize">
      <img
        src="https://api.iconify.design/mdi:window-maximize.svg"
        alt="maximize"
      />
    </div>
    <div class="titlebar-button" id="titlebar-close">
      <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
    </div>
  </div>
  <div v-if="config?.skill">
    <div class="content">
      <nav>
        <a class="navbar__brand" href="/">
          <div class="navbar__logo">
            <img
              src="https://tauri.studio/meta/tauri_logo_pride_dark.svg"
              alt="Tauri Logo"
              class="themedImage_ToTc themedImage--dark_i4oU"
            /></div
        ></a>
        <ul class="nav">
          <li v-for="(_, key) in views" @click="setView(key)">
            {{ key }}
          </li>
        </ul>
      </nav>
      <component v-bind:is="views[router]" />
    </div>
  </div>
  <div v-else-if="config"><SkillPicker @pick="setSkillConfig" /></div>
  <div v-else>3</div>
</template>

<style>
.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}
.titlebar-button:hover {
  background: #5bbec3;
}
nav {
  display: flex;
  background-color: #242526;
  height: 57px;
  padding: 8px 15px;
  align-items: center;
}
.navbar__logo img {
  height: 30px;
  margin-right: 15px;
}
nav ul li {
  cursor: pointer;
  display: inline-block;
  padding: 0 11px;
  transition: 0.2s;
}
nav ul li:hover {
  color: #65d3ea;
}
.content {
  height: calc(100vh - 30px);
  overflow: scroll;
}
</style>
