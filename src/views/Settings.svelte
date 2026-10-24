<script lang="ts">
  import { pickFolder } from "../lib/folders";
  import { demoMode, library } from "../lib/store.svelte";
  import { longDuration, plural, tracksLabel } from "../lib/format";
  import Icon from "../components/Icon.svelte";

  const shortcuts: [string, string][] = [
    ["Пробел", "Играть / пауза"],
    ["Ctrl + →  /  Ctrl + ←", "Следующий / предыдущий трек"],
    ["→  /  ←", "Перемотка ±5 секунд"],
    ["Ctrl + ↑  /  Ctrl + ↓", "Громкость"],
    ["M", "Выключить звук"],
    ["Ctrl + K  или  /", "Поиск"],
    ["Alt + ←  /  Alt + →", "Назад / вперёд"],
    ["Esc", "Закрыть полноэкранный плеер"],
  ];
</script>

<div class="page">
  <h1>Настройки</h1>

  <section>
    <div class="row-head">
      <div>
        <h2>Папки с музыкой</h2>
        <p class="dim">Lyra следит за этими папками. Изменения подхватываются при пересканировании.</p>
      </div>
      <div class="btns">
        <button class="pill" onclick={() => library.rescan()} disabled={library.scanning}>
          <Icon name="refresh" size={16} class={library.scanning ? "spin" : ""} />
          {library.scanning ? "Сканирую…" : "Пересканировать"}
        </button>
        <button class="pill primary" onclick={pickFolder} disabled={library.scanning}>
          <Icon name="plus" size={16} /> Добавить папку
        </button>
      </div>
    </div>

    <div class="folders">
      {#each library.folders as f (f)}
        <div class="folder">
          <Icon name="folder" size={20} />
          <span class="ellipsis path">{f}</span>
          <button class="icon-btn" title="Убрать из медиатеки" onclick={() => library.removeFolder(f)} disabled={library.scanning}>
            <Icon name="trash" size={16} />
          </button>
        </div>
      {:else}
        <p class="dim">Папки ещё не добавлены.</p>
      {/each}
    </div>
  </section>

  <section>
    <h2>Медиатека</h2>
    <div class="stats">
      <div><b>{library.tracks.length}</b><span>{tracksLabel(library.tracks.length).split(" ")[1]}</span></div>
      <div><b>{library.albums.length}</b><span>{plural(library.albums.length, ["альбом", "альбома", "альбомов"]).split(" ")[1]}</span></div>
      <div><b>{library.artists.length}</b><span>{plural(library.artists.length, ["исполнитель", "исполнителя", "исполнителей"]).split(" ")[1]}</span></div>
      <div><b>{longDuration(library.totalDuration)}</b><span>музыки</span></div>
    </div>
  </section>

  <section>
    <h2>Горячие клавиши</h2>
    <div class="keys">
      {#each shortcuts as [k, d]}
        <kbd>{k}</kbd><span>{d}</span>
      {/each}
    </div>
  </section>

  <section>
    <h2>О программе</h2>
    <p class="dim">
      Lyra 0.1 — нативное приложение на Tauri 2 + Rust. Звук декодируется в Rust (symphonia) и идёт напрямую
      в звуковую карту, интерфейс рисуется системным WebView — без встроенного Chromium.
      {#if demoMode}<br /><b>Сейчас открыт демо-режим в браузере с тестовыми данными.</b>{/if}
    </p>
  </section>
</div>

<style>
  h1 {
    font-size: 32px;
    font-weight: 850;
    letter-spacing: -0.03em;
    margin-bottom: 28px;
  }
  section {
    max-width: 860px;
    margin-bottom: 40px;
  }
  h2 {
    font-size: 18px;
    font-weight: 750;
    margin-bottom: 6px;
  }
  .dim {
    color: var(--text-2);
  }
  .row-head {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: 16px;
    flex-wrap: wrap;
    margin-bottom: 16px;
  }
  .btns {
    display: flex;
    gap: 8px;
  }
  .pill:disabled {
    opacity: 0.6;
  }
  .folders {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .folder {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 8px 10px 16px;
    border-radius: 10px;
    background: var(--panel);
    color: var(--text-2);
  }
  .path {
    flex: 1;
    color: var(--text);
    font-family: ui-monospace, "JetBrains Mono", "SF Mono", Menlo, monospace;
    font-size: 13px;
  }
  .stats {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 12px;
    margin-top: 12px;
  }
  .stats div {
    display: flex;
    flex-direction: column;
    padding: 18px;
    border-radius: 12px;
    background: var(--panel);
  }
  .stats b {
    font-size: 26px;
    font-weight: 800;
    letter-spacing: -0.02em;
  }
  .stats span {
    color: var(--text-2);
  }
  .keys {
    display: grid;
    grid-template-columns: max-content 1fr;
    gap: 10px 24px;
    align-items: center;
    margin-top: 12px;
  }
  kbd {
    font-family: inherit;
    font-size: 12.5px;
    padding: 4px 10px;
    border-radius: 6px;
    background: var(--panel-2);
    border: 1px solid var(--border);
    border-bottom-width: 2px;
  }
</style>
