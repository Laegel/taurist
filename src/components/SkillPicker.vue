<script setup lang="ts">
import { Skill } from '@/api/config'

const emit = defineEmits(['pick'])
const triggerPick = (key) => emit('pick', key)

const skills = {
  [Skill.Rust]: 'I know Rust',
  [Skill.JS]: 'I know JavaScript/TypeScript',
  [Skill.Both]: 'I know both of them',
  [Skill.None]: 'I do not know any of them',
}
</script>

<template>
  <div class="skillpicker">
    Looks like this is the first time you use Taurist! For improved experience, the app needs to know a bit more about your background:
  <ul>
    <li v-for="(value, key) in skills" @click="triggerPick(key)">
      {{ value }}
    </li>
  </ul>
  </div>
</template>

<style lang="stylus">
.skillpicker
  
  li
    cursor pointer

    &:hover
      text-decoration underline


</style>
