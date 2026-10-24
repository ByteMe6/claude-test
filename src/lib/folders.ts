import { demoMode, library, ui } from "./store.svelte";

export async function pickFolder() {
  if (demoMode) {
    ui.toast("В демо-режиме браузера выбор папки недоступен");
    return;
  }
  const { open } = await import("@tauri-apps/plugin-dialog");
  const dir = await open({ directory: true, multiple: false, title: "Папка с музыкой" });
  if (typeof dir === "string") {
    await library.addFolder(dir);
    ui.toast(`Готово: ${library.tracks.length} треков в медиатеке`);
  }
}
