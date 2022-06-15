import { invoke } from "@tauri-apps/api/tauri"

export enum Skill {
  Rust = 'Rust',
  JS = 'JS',
  Both = 'Both',
  None = 'None',
}

interface Config {
  skill: Skill;
}

export const getConfig = () => invoke<Config>('get_config')

export const setConfig = (config: Config) => invoke('set_config', { config })
