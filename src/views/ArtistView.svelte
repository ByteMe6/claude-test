<script lang="ts">
  import { library, player, ui } from "../lib/store.svelte";
  import { plural, tracksLabel } from "../lib/format";
  import Card from "../components/Card.svelte";
  import Cover from "../components/Cover.svelte";
  import Hero from "../components/Hero.svelte";
  import Icon from "../components/Icon.svelte";
  import TrackList from "../components/TrackList.svelte";

  let { name }: { name: string } = $props();
  const artist = $derived(library.artistByName(name));
  let showAll = $state(false);
  const isPlayingHere = $derived(
    player.playing && player.current?.albumArtist.toLowerCase() === name.toLowerCase(),
  );
</script>

{#if artist}
  <Hero color={artist.color} kind="Исполнитель" title={artist.name}>
    {#snippet art()}
      <Cover cover={artist.cover} seed={artist.name} size={232} round shadow />
    {/snippet}
    {#snippet meta()}
      <span>{plural(artist.albums.length, ["альбом", "альбома", "альбомов"])} • {tracksLabel(artist.tracks.length)}</span>
    {/snippet}
  </Hero>

  <div class="page body">
    <div class="actions">
      <button
        class="play-fab"
        onclick={() => (isPlayingHere ? player.toggle() : player.playList(artist.tracks, 0, artist.name))}
        aria-label="Играть"
      >
        <Icon name={isPlayingHere ? "pause" : "play"} size={24} />
      </button>
      <button class="icon-btn lg" onclick={() => player.playShuffled(artist.tracks, artist.name)} title="Перемешать">
        <Icon name="shuffle" size={26} />
      </button>
    </div>

    <h2 class="section-title">Треки</h2>
    <TrackList tracks={showAll ? artist.tracks : artist.tracks.slice(0, 5)} source={artist.name} />
    {#if artist.tracks.length > 5}
      <button class="more" onclick={() => (showAll = !showAll)}>{showAll ? "Свернуть" : "Показать все"}</button>
    {/if}

    <section>
      <h2 class="section-title">Дискография</h2>
      <div class="grid">
        {#each artist.albums as a (a.id)}
          <Card
            title={a.title}
            subtitle={`${a.year ?? ""}${a.year ? " · " : ""}${tracksLabel(a.tracks.length)}`}
            cover={a.cover}
            seed={a.title}
            onopen={() => ui.go({ name: "album", id: a.id })}
            onplay={() => player.playList(a.tracks, 0, `Альбом · ${a.title}`)}
          />
        {/each}
      </div>
    </section>
  </div>
{:else}
  <div class="page"><p>Исполнитель не найден.</p></div>
{/if}

<style>
  .body {
    position: relative;
    z-index: 1;
  }
  .actions {
    display: flex;
    align-items: center;
    gap: 20px;
    margin-bottom: 28px;
  }
  .lg {
    width: 48px;
    height: 48px;
  }
  .more {
    margin: 8px 16px;
    color: var(--text-2);
    font-weight: 700;
    font-size: 13px;
  }
  .more:hover {
    color: var(--text);
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
