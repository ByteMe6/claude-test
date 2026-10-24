<script lang="ts">
  import { library, player, ui } from "../lib/store.svelte";
  import { placeholderGradient, tracksLabel } from "../lib/format";
  import Card from "../components/Card.svelte";
  import Cover from "../components/Cover.svelte";
  import Icon from "../components/Icon.svelte";
  import TrackList from "../components/TrackList.svelte";

  const q = $derived(ui.search.trim().toLowerCase());
  const has = (s: string) => s.toLowerCase().includes(q);

  const tracks = $derived(q ? library.tracks.filter((t) => has(t.title) || has(t.artist) || has(t.album)) : []);
  const albums = $derived(q ? library.albums.filter((a) => has(a.title) || has(a.artist)) : []);
  const artists = $derived(q ? library.artists.filter((a) => has(a.name)) : []);

  const top = $derived.by(() => {
    if (artists[0] && artists[0].name.toLowerCase().startsWith(q)) return { kind: "artist" as const, v: artists[0] };
    if (albums[0] && albums[0].title.toLowerCase().startsWith(q)) return { kind: "album" as const, v: albums[0] };
    if (tracks[0]) return { kind: "track" as const, v: tracks[0] };
    return null;
  });

  const genres = $derived.by(() => {
    const map = new Map<string, number>();
    for (const t of library.tracks) if (t.genre) map.set(t.genre, (map.get(t.genre) ?? 0) + 1);
    return [...map.entries()].sort((a, b) => b[1] - a[1]).slice(0, 18);
  });
</script>

<div class="page">
  {#if !q}
    <h2 class="section-title">Жанры</h2>
    {#if genres.length}
      <div class="genres">
        {#each genres as [g, n]}
          <button class="genre" style:background={placeholderGradient(g)} onclick={() => (ui.search = g)}>
            <span>{g}</span>
            <small>{tracksLabel(n)}</small>
          </button>
        {/each}
      </div>
    {:else}
      <p class="hint">Начните вводить название трека, альбома или исполнителя.</p>
    {/if}
  {:else if !tracks.length && !albums.length && !artists.length}
    <div class="none">
      <h3>Ничего не нашлось по запросу «{ui.search}»</h3>
      <p>Проверьте написание или попробуйте другой запрос.</p>
    </div>
  {:else}
    <div class="top-row">
      {#if top}
        <section class="top">
          <h2 class="section-title">Лучший результат</h2>
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <div
            class="top-card"
            role="button"
            tabindex="0"
            onclick={() =>
              top.kind === "artist"
                ? ui.go({ name: "artist", name_: top.v.name })
                : top.kind === "album"
                  ? ui.go({ name: "album", id: top.v.id })
                  : ui.go({ name: "album", id: top.v.albumId })}
          >
            {#if top.kind === "artist"}
              <Cover cover={top.v.cover} seed={top.v.name} size={96} round shadow />
              <h3 class="ellipsis">{top.v.name}</h3>
              <span class="badge">Исполнитель</span>
            {:else if top.kind === "album"}
              <Cover cover={top.v.cover} seed={top.v.title} size={96} radius={6} shadow />
              <h3 class="ellipsis">{top.v.title}</h3>
              <span class="sub">Альбом · {top.v.artist}</span>
            {:else}
              <Cover cover={top.v.cover} seed={top.v.album} size={96} radius={6} shadow />
              <h3 class="ellipsis">{top.v.title}</h3>
              <span class="sub">Трек · {top.v.artist}</span>
            {/if}
            <button
              class="play-fab fab"
              aria-label="Играть"
              onclick={(e) => {
                e.stopPropagation();
                if (top.kind === "artist") player.playList(top.v.tracks, 0, top.v.name);
                else if (top.kind === "album") player.playList(top.v.tracks, 0, `Альбом · ${top.v.title}`);
                else player.playList(tracks, 0, `Поиск · ${ui.search}`);
              }}
            >
              <Icon name="play" size={22} />
            </button>
          </div>
        </section>
      {/if}
      {#if tracks.length}
        <section class="songs">
          <h2 class="section-title">Треки</h2>
          <TrackList tracks={tracks.slice(0, 4)} source={`Поиск · ${ui.search}`} showAlbum={false} />
        </section>
      {/if}
    </div>

    {#if artists.length}
      <section>
        <h2 class="section-title">Исполнители</h2>
        <div class="grid">
          {#each artists.slice(0, 12) as ar (ar.name)}
            <Card
              round
              title={ar.name}
              subtitle="Исполнитель"
              cover={ar.cover}
              seed={ar.name}
              onopen={() => ui.go({ name: "artist", name_: ar.name })}
              onplay={() => player.playShuffled(ar.tracks, ar.name)}
            />
          {/each}
        </div>
      </section>
    {/if}

    {#if albums.length}
      <section>
        <h2 class="section-title">Альбомы</h2>
        <div class="grid">
          {#each albums.slice(0, 18) as a (a.id)}
            <Card
              title={a.title}
              subtitle={`${a.year ?? ""}${a.year ? " · " : ""}${a.artist}`}
              cover={a.cover}
              seed={a.title}
              onopen={() => ui.go({ name: "album", id: a.id })}
              onplay={() => player.playList(a.tracks, 0, `Альбом · ${a.title}`)}
            />
          {/each}
        </div>
      </section>
    {/if}

    {#if tracks.length > 4}
      <section>
        <h2 class="section-title">Все треки</h2>
        <TrackList {tracks} source={`Поиск · ${ui.search}`} />
      </section>
    {/if}
  {/if}
</div>

<style>
  .genres {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 16px;
  }
  .genre {
    position: relative;
    height: 110px;
    padding: 16px;
    border-radius: 10px;
    text-align: left;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    transition: transform 0.2s var(--ease);
  }
  .genre:hover {
    transform: scale(1.02);
  }
  .genre span {
    font-size: 20px;
    font-weight: 800;
  }
  .genre small {
    color: rgba(255, 255, 255, 0.75);
  }
  .hint,
  .none p {
    color: var(--text-2);
  }
  .none {
    padding: 60px 0;
    text-align: center;
  }
  .none h3 {
    font-size: 22px;
    margin-bottom: 8px;
  }
  .top-row {
    display: grid;
    grid-template-columns: minmax(280px, 2fr) 3fr;
    gap: 24px;
    margin-bottom: 36px;
  }
  .top-card {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 14px;
    padding: 20px;
    border-radius: 12px;
    background: var(--panel);
    cursor: pointer;
    transition: background 0.2s;
  }
  .top-card:hover {
    background: var(--panel-2);
  }
  .top-card h3 {
    font-size: 30px;
    font-weight: 850;
    letter-spacing: -0.03em;
  }
  .badge {
    align-self: flex-start;
    padding: 4px 12px;
    border-radius: 999px;
    background: rgba(0, 0, 0, 0.35);
    font-size: 13px;
    font-weight: 700;
  }
  .sub {
    color: var(--text-2);
  }
  .fab {
    position: absolute;
    right: 20px;
    bottom: 20px;
    opacity: 0;
    transform: translateY(8px);
    transition: opacity 0.2s, transform 0.25s var(--ease);
  }
  .top-card:hover .fab {
    opacity: 1;
    transform: none;
  }
  .songs :global(.head) {
    display: none;
  }
  section {
    margin-bottom: 36px;
    min-width: 0;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(176px, 1fr));
    gap: 4px;
    margin: 0 -12px;
  }
</style>
