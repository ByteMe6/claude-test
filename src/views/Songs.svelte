<script lang="ts">
  import type { Track } from "../lib/api";
  import { compare, library, likes, player } from "../lib/store.svelte";
  import { longDuration, tracksLabel } from "../lib/format";
  import Icon from "../components/Icon.svelte";
  import TrackList, { type SortKey } from "../components/TrackList.svelte";

  let { liked = false }: { liked?: boolean } = $props();

  let sort = $state<{ key: SortKey; desc: boolean } | null>(null);
  let filter = $state("");

  const base = $derived(liked ? likes.tracks : library.tracks);
  const tracks = $derived.by(() => {
    const q = filter.trim().toLowerCase();
    let list = q
      ? base.filter((t) => `${t.title} ${t.artist} ${t.album}`.toLowerCase().includes(q))
      : base;
    if (sort) {
      const { key, desc } = sort;
      const cmp: (a: Track, b: Track) => number =
        key === "duration"
          ? (a, b) => a.duration - b.duration
          : key === "added"
            ? (a, b) => a.added - b.added
            : (a, b) => compare(a[key], b[key]);
      list = [...list].sort((a, b) => (desc ? -cmp(a, b) : cmp(a, b)));
    }
    return list;
  });
  const total = $derived(tracks.reduce((s, t) => s + t.duration, 0));
  const source = $derived(liked ? "Любимые треки" : "Все треки");
  const playingHere = $derived(player.playing && player.source === source);

  function onsort(key: SortKey) {
    if (sort?.key !== key) sort = { key, desc: key === "added" };
    else if (sort.desc === (key === "added")) sort = { key, desc: !sort.desc };
    else sort = null;
  }
</script>

<div class="wrap" class:liked>
  <header class="head">
    {#if liked}
      <div class="tile"><Icon name="heart" size={72} filled /></div>
    {/if}
    <div>
      <span class="kind">{liked ? "Плейлист" : "Медиатека"}</span>
      <h1>{liked ? "Любимые треки" : "Все треки"}</h1>
      <p class="meta">{tracksLabel(tracks.length)}{total ? `, ${longDuration(total)}` : ""}</p>
    </div>
  </header>

  <div class="page body">
    <div class="actions">
      <button
        class="play-fab"
        disabled={!tracks.length}
        onclick={() => (playingHere ? player.toggle() : player.playList(tracks, 0, source))}
        aria-label="Играть"
      >
        <Icon name={playingHere ? "pause" : "play"} size={24} />
      </button>
      <button class="icon-btn lg" onclick={() => player.playShuffled(tracks, source)} title="Перемешать">
        <Icon name="shuffle" size={26} />
      </button>
      <label class="filter">
        <Icon name="search" size={16} />
        <input placeholder="Фильтр" bind:value={filter} />
      </label>
    </div>

    {#if tracks.length}
      <TrackList {tracks} {source} {sort} {onsort} showAdded={!liked} />
    {:else if liked}
      <div class="empty">
        <Icon name="heart" size={44} />
        <h3>Здесь будут любимые треки</h3>
        <p>Нажимайте на сердечко рядом с треком, чтобы сохранить его сюда.</p>
      </div>
    {:else}
      <p class="empty">Ничего не найдено.</p>
    {/if}
  </div>
</div>

<style>
  .wrap {
    --c: 80, 80, 100;
  }
  .wrap.liked {
    --c: 80, 56, 200;
  }
  .head {
    display: flex;
    align-items: flex-end;
    gap: 28px;
    margin-top: -64px;
    padding: 96px 32px 28px;
    background: linear-gradient(180deg, rgba(var(--c), 0.85), rgba(var(--c), 0.35));
  }
  .tile {
    display: grid;
    place-items: center;
    width: 200px;
    height: 200px;
    border-radius: 8px;
    background: linear-gradient(135deg, #4f46e5, #a78bfa 60%, #e9d5ff);
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.45);
  }
  .kind {
    font-size: 13px;
    font-weight: 600;
  }
  h1 {
    font-size: 64px;
    font-weight: 900;
    letter-spacing: -0.045em;
    line-height: 1.05;
    margin: 4px 0 10px;
  }
  .meta {
    color: rgba(255, 255, 255, 0.8);
  }
  .body {
    background: linear-gradient(180deg, rgba(var(--c), 0.2), transparent 240px);
  }
  .actions {
    display: flex;
    align-items: center;
    gap: 20px;
    margin-bottom: 20px;
  }
  .lg {
    width: 48px;
    height: 48px;
  }
  .filter {
    margin-left: auto;
    display: flex;
    align-items: center;
    gap: 8px;
    height: 36px;
    padding: 0 12px;
    border-radius: 6px;
    color: var(--text-2);
    background: var(--hover);
  }
  .filter input {
    width: 180px;
    border: 0;
    outline: 0;
    background: none;
  }
  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    padding: 48px;
    color: var(--text-2);
    text-align: center;
  }
  .empty h3 {
    color: var(--text);
    font-size: 22px;
  }
</style>
