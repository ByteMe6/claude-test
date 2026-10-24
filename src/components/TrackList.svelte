<script lang="ts" module>
  export type SortKey = "title" | "album" | "added" | "duration" | "artist";
</script>

<script lang="ts">
  import { getContext, untrack } from "svelte";
  import type { Track } from "../lib/api";
  import { likes, player, ui } from "../lib/store.svelte";
  import { relativeDate, time } from "../lib/format";
  import Cover from "./Cover.svelte";
  import Equalizer from "./Equalizer.svelte";
  import Icon from "./Icon.svelte";

  let {
    tracks,
    source = "",
    showCover = true,
    showAlbum = true,
    showAdded = false,
    useTrackNo = false,
    sort = null,
    onsort,
  }: {
    tracks: Track[];
    source?: string;
    showCover?: boolean;
    showAlbum?: boolean;
    showAdded?: boolean;
    useTrackNo?: boolean;
    sort?: { key: SortKey; desc: boolean } | null;
    onsort?: (key: SortKey) => void;
  } = $props();

  const ROW = 56;
  const OVERSCAN = 12;
  const getScroller = getContext<() => HTMLElement | undefined>("scroller");

  let listEl: HTMLDivElement | undefined = $state();
  let range = $state({ start: 0, end: 40 });
  let selected = $state<string | null>(null);

  function update() {
    const sc = getScroller?.();
    if (!sc || !listEl) return;
    const top = listEl.getBoundingClientRect().top - sc.getBoundingClientRect().top;
    const first = Math.floor(-top / ROW) - OVERSCAN;
    const last = Math.ceil((-top + sc.clientHeight) / ROW) + OVERSCAN;
    const start = Math.max(0, first);
    const end = Math.min(tracks.length, Math.max(start, last));
    if (start !== range.start || end !== range.end) range = { start, end };
  }

  $effect(() => {
    const sc = getScroller?.();
    if (!sc) return;
    const ro = new ResizeObserver(update);
    ro.observe(sc);
    sc.addEventListener("scroll", update, { passive: true });
    return () => {
      ro.disconnect();
      sc.removeEventListener("scroll", update);
    };
  });

  $effect(() => {
    tracks;
    untrack(update);
  });

  const visible = $derived(tracks.slice(range.start, range.end));
  const columns = $derived(
    ["44px", "minmax(180px, 5fr)", showAlbum && "minmax(120px, 3fr)", showAdded && "minmax(100px, 2fr)", "40px", "64px"]
      .filter(Boolean)
      .join(" "),
  );

  function play(i: number) {
    const t = tracks[i];
    if (player.isCurrent(t.id)) {
      player.toggle();
      return;
    }
    player.playList(tracks, i, source);
  }

  function sortIcon(key: SortKey) {
    if (sort?.key !== key) return "";
    return sort.desc ? "↓" : "↑";
  }
</script>

<div class="tracklist" style:--cols={columns}>
  <div class="row head">
    <span class="num">#</span>
    <button class="h" disabled={!onsort} onclick={() => onsort?.("title")}>Название {sortIcon("title")}</button>
    {#if showAlbum}
      <button class="h" disabled={!onsort} onclick={() => onsort?.("album")}>Альбом {sortIcon("album")}</button>
    {/if}
    {#if showAdded}
      <button class="h" disabled={!onsort} onclick={() => onsort?.("added")}>Добавлено {sortIcon("added")}</button>
    {/if}
    <span></span>
    <button class="h dur" disabled={!onsort} onclick={() => onsort?.("duration")} title="Длительность">
      {sortIcon("duration")}<Icon name="clock" size={16} />
    </button>
  </div>

  <div class="body" bind:this={listEl} style:height="{tracks.length * ROW}px">
    <div style:transform="translateY({range.start * ROW}px)">
      {#each visible as t, vi (t.id)}
        {@const i = range.start + vi}
        {@const current = player.isCurrent(t.id)}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <div
          class="row"
          class:current
          class:selected={selected === t.id}
          role="row"
          tabindex="-1"
          onclick={() => (selected = t.id)}
          ondblclick={() => play(i)}
          oncontextmenu={(e) => {
            selected = t.id;
            ui.trackMenu(e, t);
          }}
        >
          <span class="num tnum">
            {#if current}
              <span class="idx"><Equalizer playing={player.playing} /></span>
            {:else}
              <span class="idx">{useTrackNo ? (t.trackNo ?? i + 1) : i + 1}</span>
            {/if}
            <button class="hover-play" onclick={(e) => { e.stopPropagation(); play(i); }} aria-label="Играть">
              <Icon name={current && player.playing ? "pause" : "play"} size={16} />
            </button>
          </span>
          <span class="title-cell">
            {#if showCover}
              <Cover cover={t.cover} seed={t.album} size={40} radius={4} />
            {/if}
            <span class="meta">
              <span class="title ellipsis">{t.title}</span>
              <span class="artist ellipsis">
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <span class="link" onclick={(e) => { e.stopPropagation(); ui.go({ name: "artist", name_: t.albumArtist }); }}>{t.artist}</span>
              </span>
            </span>
          </span>
          {#if showAlbum}
            <span class="dim ellipsis">
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <span class="link" onclick={(e) => { e.stopPropagation(); ui.go({ name: "album", id: t.albumId }); }}>{t.album}</span>
            </span>
          {/if}
          {#if showAdded}
            <span class="dim ellipsis">{relativeDate(t.added)}</span>
          {/if}
          <span>
            <button
              class="like"
              class:on={likes.has(t.id)}
              onclick={(e) => { e.stopPropagation(); likes.toggle(t.id); }}
              aria-label="Нравится"
            >
              <Icon name="heart" size={16} filled={likes.has(t.id)} />
            </button>
          </span>
          <span class="dim tnum dur">{time(t.duration)}</span>
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .tracklist {
    --row: 56px;
  }
  .row {
    display: grid;
    grid-template-columns: var(--cols);
    align-items: center;
    gap: 16px;
    height: var(--row);
    padding: 0 16px;
    border-radius: 8px;
    color: var(--text-2);
  }
  .row:not(.head):hover {
    background: var(--hover);
  }
  .row.selected {
    background: var(--hover-strong) !important;
  }
  .head {
    height: 40px;
    margin-bottom: 8px;
    border-radius: 0;
    border-bottom: 1px solid var(--border);
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--text-3);
  }
  .h {
    text-align: left;
    font-size: inherit;
    letter-spacing: inherit;
    text-transform: inherit;
    color: inherit;
  }
  .h:not(:disabled):hover {
    color: var(--text);
  }
  .h:disabled {
    cursor: default;
  }
  .dur {
    justify-self: end;
    display: inline-flex;
    gap: 4px;
    align-items: center;
  }
  .num {
    position: relative;
    display: grid;
    place-items: center end;
    padding-right: 6px;
    font-size: 15px;
  }
  .hover-play {
    position: absolute;
    inset: 0;
    display: none;
    place-items: center end;
    padding-right: 4px;
    color: var(--text);
  }
  .row:hover .hover-play {
    display: grid;
  }
  .row:hover .idx {
    visibility: hidden;
  }
  .title-cell {
    display: flex;
    align-items: center;
    gap: 12px;
    min-width: 0;
  }
  .meta {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .title {
    color: var(--text);
    font-size: 15px;
    font-weight: 500;
  }
  .current .title {
    color: var(--accent);
  }
  .artist {
    font-size: 13px;
  }
  .dim {
    font-size: 13.5px;
  }
  .like {
    display: grid;
    place-items: center;
    width: 32px;
    height: 32px;
    color: var(--text-3);
    opacity: 0;
    transition: opacity 0.15s, color 0.15s, transform 0.15s;
  }
  .row:hover .like,
  .like.on {
    opacity: 1;
  }
  .like:hover {
    color: var(--text);
    transform: scale(1.1);
  }
  .like.on {
    color: var(--accent);
  }
</style>
