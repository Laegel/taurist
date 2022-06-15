import { invoke } from "@tauri-apps/api/tauri"

export const removeSystemTray = () => invoke('remove_system_tray')
