<script lang="ts">
  import { library, likes, player, ui } from "../lib/store.svelte";
  import { longDuration, tracksLabel } from "../lib/format";
  import Card from "../components/Card.svelte";
  import Cover from "../components/Cover.svelte";
  import Hero from "../components/Hero.svelte";
  import Icon from "../components/Icon.svelte";
  import TrackList from "../components/TrackList.svelte";

  let { id }: { id: string } = $props();
  const album = $derived(library.albumById.get(id));
  const isPlayingHere = $derived(player.playing && player.current?.albumId === id);
  const allLiked = $derived(!!album && album.tracks.every((t) => likes.has(t.id)));
  const more = $derived(
    album ? (library.artistByName(album.artist)?.albums.filter((a) => a.id !== id) ?? []) : [],
  );

  function playAlbum() {
    if (!album) return;
    if (player.current?.albumId === id) player.toggle();
    else player.playList(album.tracks, 0, `Альбом · ${album.title}`);
  }
  function likeAll() {
    if (!album) return;
    for (const t of album.tracks) if (likes.has(t.id) === allLiked) likes.toggle(t.id);
  }
</script>

{#if album}
  <Hero color={album.color} kind="Альбом" title={album.title}>
    {#snippet art()}
      <Cover cover={album.cover} seed={album.title} size={232} radius={8} shadow />
    {/snippet}
    {#snippet meta()}
      <b class="link" role="link" tabindex="0" onclick={() => ui.go({ name: "artist", name_: album.artist })} onkeydown={() => {}}>{album.artist}</b>
      {#if album.year}<span>• {album.year}</span>{/if}
      <span>• {tracksLabel(album.tracks.length)}, {longDuration(album.duration)}</span>
    {/snippet}
  </Hero>

  <div class="page actions-page">
    <div class="actions">
      <button class="play-fab" onclick={playAlbum} aria-label="Играть">
        <Icon name={isPlayingHere ? "pause" : "play"} size={24} />
      </button>
      <button class="icon-btn lg" onclick={() => player.playShuffled(album.tracks, `Альбом · ${album.title}`)} title="Перемешать">
        <Icon name="shuffle" size={26} />
      </button>
      <button class="icon-btn lg" class:active={allLiked} onclick={likeAll} title="Добавить все в любимые">
        <Icon name="heart" size={26} filled={allLiked} />
      </button>
    </div>

    <TrackList tracks={album.tracks} source={`Альбом · ${album.title}`} showCover={false} showAlbum={false} useTrackNo />

    {#if more.length}
      <section>
        <h2 class="section-title">Ещё от {album.artist}</h2>
        <div class="grid">
          {#each more as a (a.id)}
            <Card
              title={a.title}
              subtitle={`${a.year ?? "Альбом"}`}
              cover={a.cover}
              seed={a.title}
              onopen={() => ui.go({ name: "album", id: a.id })}
              onplay={() => player.playList(a.tracks, 0, `Альбом · ${a.title}`)}
            />
          {/each}
        </div>
      </section>
    {/if}
  </div>
{:else}
  <div class="page"><p>Альбом не найден.</p></div>
{/if}

<style>
  .actions-page {
    position: relative;
    z-index: 1;
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
  section {
    margin-top: 40px;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(176px, 1fr));
    gap: 4px;
    margin: 0 -12px;
  }
</style>
