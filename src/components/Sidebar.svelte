<script lang="ts">
  import { library, likes, player, ui, type Route } from "../lib/store.svelte";
  import { tracksLabel } from "../lib/format";
  import Cover from "./Cover.svelte";
  import Icon from "./Icon.svelte";

  const nav: { route: Route; label: string; icon: string }[] = [
    { route: { name: "home" }, label: "Главная", icon: "home" },
    { route: { name: "search" }, label: "Поиск", icon: "search" },
  ];
  const libNav: { route: Route; label: string; icon: string }[] = [
    { route: { name: "songs" }, label: "Все треки", icon: "music" },
    { route: { name: "albums" }, label: "Альбомы", icon: "album" },
    { route: { name: "artists" }, label: "Исполнители", icon: "artist" },
  ];

  const recentAlbums = $derived([...library.albums].sort((a, b) => b.added - a.added).slice(0, 30));
  const is = (r: Route) => ui.route.name === r.name;
</script>

<aside class="sidebar">
  <div class="brand">
    <span class="logo"></span>
    <span class="name">Lyra</span>
  </div>

  <nav class="block">
    {#each nav as item}
      <button class="nav" class:active={is(item.route)} onclick={() => ui.go(item.route)}>
        <Icon name={item.icon} size={22} />
        {item.label}
      </button>
    {/each}
  </nav>

  <div class="block grow">
    <div class="block-head">
      <span><Icon name="library" size={20} /> Медиатека</span>
      <button class="icon-btn" title="Настройки медиатеки" onclick={() => ui.go({ name: "settings" })}>
        <Icon name="plus" size={18} />
      </button>
    </div>

    {#each libNav as item}
      <button class="nav small" class:active={is(item.route)} onclick={() => ui.go(item.route)}>
        <Icon name={item.icon} size={18} />
        {item.label}
      </button>
    {/each}

    <div class="list">
      <button class="entry" class:active={is({ name: "liked" })} onclick={() => ui.go({ name: "liked" })}>
        <span class="liked-tile"><Icon name="heart" size={18} filled /></span>
        <span class="meta">
          <span class="ellipsis t">Любимые треки</span>
          <span class="ellipsis s">Плейлист · {tracksLabel(likes.ids.size)}</span>
        </span>
      </button>
      {#each recentAlbums as a (a.id)}
        {@const playingHere = player.current?.albumId === a.id}
        <button
          class="entry"
          class:active={ui.route.name === "album" && ui.route.id === a.id}
          onclick={() => ui.go({ name: "album", id: a.id })}
          ondblclick={() => player.playList(a.tracks, 0, `Альбом · ${a.title}`)}
        >
          <Cover cover={a.cover} seed={a.title} size={44} radius={6} />
          <span class="meta">
            <span class="ellipsis t" class:accent={playingHere}>{a.title}</span>
            <span class="ellipsis s">Альбом · {a.artist}</span>
          </span>
        </button>
      {/each}
    </div>
  </div>

  <button class="nav small settings" class:active={is({ name: "settings" })} onclick={() => ui.go({ name: "settings" })}>
    <Icon name="settings" size={18} />
    Настройки
  </button>
</aside>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-height: 0;
    padding: 8px 0 8px 8px;
  }
  .brand {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 14px 16px 10px;
  }
  .logo {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    background: var(--brand);
    box-shadow: inset 0 0 0 7px rgba(11, 11, 15, 0.85), 0 0 18px rgba(236, 72, 153, 0.35);
  }
  .name {
    font-size: 20px;
    font-weight: 800;
    letter-spacing: -0.03em;
  }
  .block {
    background: var(--panel);
    border-radius: var(--radius-lg);
    padding: 8px;
  }
  .grow {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }
  .block-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 4px 6px 12px;
    color: var(--text-2);
    font-weight: 700;
  }
  .block-head span {
    display: inline-flex;
    align-items: center;
    gap: 10px;
  }
  .nav {
    display: flex;
    align-items: center;
    gap: 16px;
    width: 100%;
    height: 44px;
    padding: 0 12px;
    border-radius: 8px;
    color: var(--text-2);
    font-weight: 700;
    font-size: 15px;
    transition: color 0.15s, background 0.15s;
  }
  .nav.small {
    height: 38px;
    gap: 14px;
    font-size: 14px;
    font-weight: 600;
  }
  .nav:hover {
    color: var(--text);
  }
  .nav.active {
    color: var(--text);
    background: var(--hover);
  }
  .settings {
    background: var(--panel);
    border-radius: var(--radius-lg);
  }
  .list {
    margin-top: 8px;
    padding-top: 8px;
    border-top: 1px solid var(--border);
    overflow-y: auto;
    flex: 1;
    min-height: 0;
  }
  .entry {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    padding: 6px;
    border-radius: 8px;
    text-align: left;
  }
  .entry:hover {
    background: var(--hover);
  }
  .entry.active {
    background: var(--hover-strong);
  }
  .liked-tile {
    display: grid;
    place-items: center;
    width: 44px;
    height: 44px;
    flex: none;
    border-radius: 6px;
    background: linear-gradient(135deg, #4f46e5, #a78bfa 60%, #e9d5ff);
  }
  .meta {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .t {
    font-weight: 500;
  }
  .t.accent {
    color: var(--accent);
  }
  .s {
    font-size: 12.5px;
    color: var(--text-2);
  }
</style>
