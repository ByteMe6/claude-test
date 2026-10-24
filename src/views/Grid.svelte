<script lang="ts">
  // Albums and Artists overview pages share this layout.
  import { compare, library, player, ui } from "../lib/store.svelte";
  import { tracksLabel } from "../lib/format";
  import Card from "../components/Card.svelte";

  let { kind }: { kind: "albums" | "artists" } = $props();

  type Sort = "name" | "recent" | "year";
  let sort = $state<Sort>("name");

  const albums = $derived.by(() => {
    const list = [...library.albums];
    if (sort === "recent") list.sort((a, b) => b.added - a.added);
    else if (sort === "year") list.sort((a, b) => (b.year ?? 0) - (a.year ?? 0));
    else list.sort((a, b) => compare(a.title, b.title));
    return list;
  });
  const artists = $derived(library.artists);
</script>

<div class="page">
  <div class="head">
    <h1>{kind === "albums" ? "Альбомы" : "Исполнители"}</h1>
    {#if kind === "albums"}
      <div class="chips">
        <button class:on={sort === "name"} onclick={() => (sort = "name")}>По названию</button>
        <button class:on={sort === "recent"} onclick={() => (sort = "recent")}>Недавние</button>
        <button class:on={sort === "year"} onclick={() => (sort = "year")}>По году</button>
      </div>
    {/if}
  </div>

  <div class="grid">
    {#if kind === "albums"}
      {#each albums as a (a.id)}
        <Card
          title={a.title}
          subtitle={`${a.year ?? ""}${a.year ? " · " : ""}${a.artist}`}
          cover={a.cover}
          seed={a.title}
          playing={player.playing && player.current?.albumId === a.id}
          onopen={() => ui.go({ name: "album", id: a.id })}
          onplay={() =>
            player.current?.albumId === a.id ? player.toggle() : player.playList(a.tracks, 0, `Альбом · ${a.title}`)}
        />
      {/each}
    {:else}
      {#each artists as ar (ar.name)}
        <Card
          round
          title={ar.name}
          subtitle={tracksLabel(ar.tracks.length)}
          cover={ar.cover}
          seed={ar.name}
          onopen={() => ui.go({ name: "artist", name_: ar.name })}
          onplay={() => player.playShuffled(ar.tracks, ar.name)}
        />
      {/each}
    {/if}
  </div>
</div>

<style>
  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    margin-bottom: 20px;
    flex-wrap: wrap;
  }
  h1 {
    font-size: 32px;
    font-weight: 850;
    letter-spacing: -0.03em;
  }
  .chips {
    display: flex;
    gap: 8px;
  }
  .chips button {
    height: 32px;
    padding: 0 14px;
    border-radius: 999px;
    background: var(--hover-strong);
    font-size: 13px;
    font-weight: 600;
  }
  .chips button.on {
    background: var(--text);
    color: #0b0b0f;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(176px, 1fr));
    gap: 4px;
    margin: 0 -12px;
  }
</style>
