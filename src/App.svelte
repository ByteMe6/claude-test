<script lang="ts">
  import { onMount, setContext } from "svelte";
  import { demoMode, library, player, ui } from "./lib/store.svelte";
  import Icon from "./components/Icon.svelte";
  import NowPlaying from "./components/NowPlaying.svelte";
  import Overlays from "./components/Overlays.svelte";
  import PlayerBar from "./components/PlayerBar.svelte";
  import QueuePanel from "./components/QueuePanel.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  import AlbumView from "./views/AlbumView.svelte";
  import ArtistView from "./views/ArtistView.svelte";
  import Grid from "./views/Grid.svelte";
  import Home from "./views/Home.svelte";
  import Search from "./views/Search.svelte";
  import Settings from "./views/Settings.svelte";
  import Songs from "./views/Songs.svelte";

  let scroller: HTMLElement | undefined = $state();
  let searchInput: HTMLInputElement | undefined = $state();
  let scrolled = $state(false);
  setContext("scroller", () => scroller);

  onMount(async () => {
    await library.init();
    await player.init();
  });

  // The whole UI tints itself with the current cover's color.
  $effect(() => {
    document.documentElement.style.setProperty("--accent-rgb", accentFrom(player.current?.color));
  });

  // New page → back to top.
  $effect(() => {
    ui.route;
    scroller?.scrollTo({ top: 0 });
  });

  /** Brightens the cover color so it stays readable as text/icon accent on dark. */
  function accentFrom(rgb: string | null | undefined): string {
    if (!rgb) return "167, 139, 250";
    let [r, g, b] = rgb.split(",").map(Number);
    const max = Math.max(r, g, b, 1);
    const k = Math.max(1, 215 / max);
    [r, g, b] = [r, g, b].map((c) => Math.min(255, Math.round(c * k + (255 - c * k) * 0.15)));
    return `${r}, ${g}, ${b}`;
  }

  function onSearchInput() {
    if (ui.route.name !== "search") ui.go({ name: "search" });
  }

  function onKey(e: KeyboardEvent) {
    const target = e.target as HTMLElement;
    const typing = target.tagName === "INPUT" || target.tagName === "TEXTAREA";
    const mod = e.ctrlKey || e.metaKey;

    if ((mod && e.key.toLowerCase() === "k") || (!typing && e.key === "/")) {
      e.preventDefault();
      searchInput?.focus();
      searchInput?.select();
      return;
    }
    if (e.key === "Escape") {
      if (typing) target.blur();
      ui.nowPlayingOpen = false;
      return;
    }
    if (e.altKey && e.key === "ArrowLeft") return ui.back();
    if (e.altKey && e.key === "ArrowRight") return ui.forward();
    if (typing) return;

    const actions: Record<string, () => void> = {
      " ": () => player.toggle(),
      ArrowRight: () => (mod ? player.next() : player.seekBy(5)),
      ArrowLeft: () => (mod ? player.prev() : player.seekBy(-5)),
      ArrowUp: () => mod && player.setVolume(player.volume + 0.05),
      ArrowDown: () => mod && player.setVolume(player.volume - 0.05),
      m: () => player.toggleMute(),
    };
    const action = actions[e.key];
    if (action) {
      e.preventDefault();
      action();
    }
  }
</script>

<svelte:window onkeydown={onKey} onmouseup={(e) => (e.button === 3 ? ui.back() : e.button === 4 ? ui.forward() : null)} />

<div class="app" class:with-queue={ui.queueOpen}>
  <Sidebar />

  <main bind:this={scroller} onscroll={() => (scrolled = (scroller?.scrollTop ?? 0) > 40)}>
    <div class="topbar" class:scrolled>
      <div class="nav">
        <button class="icon-btn round" disabled={!ui.canBack} onclick={() => ui.back()} title="Назад">
          <Icon name="chevron-left" size={20} />
        </button>
        <button class="icon-btn round" disabled={!ui.canForward} onclick={() => ui.forward()} title="Вперёд">
          <Icon name="chevron-right" size={20} />
        </button>
      </div>
      <label class="search" class:focus={ui.route.name === "search"}>
        <Icon name="search" size={18} />
        <input
          bind:this={searchInput}
          bind:value={ui.search}
          oninput={onSearchInput}
          onfocus={onSearchInput}
          placeholder="Что хотите послушать?"
          spellcheck="false"
        />
        {#if ui.search}
          <button class="clear" onclick={() => (ui.search = "")} aria-label="Очистить"><Icon name="x" size={16} /></button>
        {/if}
      </label>
      <div class="right">
        {#if demoMode}<span class="demo">демо</span>{/if}
        {#if library.scanning}<span class="scan"><Icon name="refresh" size={14} class="spin" /> Сканирование…</span>{/if}
      </div>
    </div>

    <div class="view">
      {#if ui.route.name === "home"}
        <Home />
      {:else if ui.route.name === "songs"}
        <Songs />
      {:else if ui.route.name === "liked"}
        {#key "liked"}<Songs liked />{/key}
      {:else if ui.route.name === "albums"}
        <Grid kind="albums" />
      {:else if ui.route.name === "artists"}
        <Grid kind="artists" />
      {:else if ui.route.name === "album"}
        {#key ui.route.id}<AlbumView id={ui.route.id} />{/key}
      {:else if ui.route.name === "artist"}
        {#key ui.route.name_}<ArtistView name={ui.route.name_} />{/key}
      {:else if ui.route.name === "search"}
        <Search />
      {:else if ui.route.name === "settings"}
        <Settings />
      {/if}
    </div>
  </main>

  {#if ui.queueOpen}
    <QueuePanel />
  {/if}

  <div class="player"><PlayerBar /></div>
</div>

{#if ui.nowPlayingOpen}
  <NowPlaying />
{/if}
<Overlays />

<style>
  .app {
    display: grid;
    grid-template-columns: var(--sidebar-w) minmax(0, 1fr);
    grid-template-rows: minmax(0, 1fr) var(--player-h);
    height: 100vh;
    gap: 0 8px;
    background: var(--bg);
  }
  .app.with-queue {
    grid-template-columns: var(--sidebar-w) minmax(0, 1fr) var(--queue-w);
  }
  .app > :global(.queue) {
    margin: 8px 8px 0 0;
  }
  main {
    position: relative;
    margin-top: 8px;
    border-radius: var(--radius-lg);
    background: var(--panel);
    overflow-y: auto;
    overflow-x: hidden;
    overscroll-behavior: contain;
  }
  .app:not(.with-queue) main {
    margin-right: 8px;
  }
  .player {
    grid-column: 1 / -1;
  }
  .topbar {
    position: sticky;
    top: 0;
    z-index: 10;
    display: grid;
    grid-template-columns: 1fr minmax(240px, 420px) 1fr;
    align-items: center;
    gap: 16px;
    height: 64px;
    padding: 0 24px;
    transition: background 0.25s, box-shadow 0.25s;
  }
  .topbar.scrolled {
    background: color-mix(in srgb, var(--panel) 88%, transparent);
    backdrop-filter: blur(16px) saturate(1.4);
    box-shadow: 0 1px 0 var(--border);
  }
  .nav {
    display: flex;
    gap: 8px;
  }
  .round {
    width: 32px;
    height: 32px;
    background: rgba(0, 0, 0, 0.45);
    color: var(--text);
  }
  .search {
    display: flex;
    align-items: center;
    gap: 10px;
    height: 44px;
    padding: 0 14px 0 16px;
    border-radius: 999px;
    background: #1f1f26;
    color: var(--text-2);
    border: 1px solid transparent;
    transition: border-color 0.15s, background 0.15s;
  }
  .search:hover {
    background: #26262e;
  }
  .search:focus-within,
  .search.focus {
    border-color: rgba(255, 255, 255, 0.3);
  }
  .search input {
    flex: 1;
    min-width: 0;
    border: 0;
    outline: 0;
    background: none;
    font-size: 14.5px;
    color: var(--text);
  }
  .search input::placeholder {
    color: var(--text-3);
  }
  .clear {
    display: grid;
    place-items: center;
    color: var(--text-2);
  }
  .right {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
  }
  .demo,
  .scan {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    height: 28px;
    padding: 0 12px;
    border-radius: 999px;
    background: rgba(0, 0, 0, 0.4);
    color: var(--text-2);
    font-size: 12px;
    font-weight: 600;
  }
  .demo {
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.1em;
  }
  .view {
    position: relative;
  }
</style>
