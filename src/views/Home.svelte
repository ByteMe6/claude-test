<script lang="ts">
  import { library, likes, player, ui } from "../lib/store.svelte";
  import { greeting, tracksLabel } from "../lib/format";
  import Card from "../components/Card.svelte";
  import Cover from "../components/Cover.svelte";
  import Icon from "../components/Icon.svelte";
  import Onboarding from "./Onboarding.svelte";

  const recent = $derived([...library.albums].sort((a, b) => b.added - a.added));
  const quick = $derived(recent.slice(0, 7));
  const artists = $derived(
    [...library.artists].sort((a, b) => b.tracks.length - a.tracks.length).slice(0, 12),
  );
  const shuffle = $derived(
    // A stable "discover" pick per day.
    [...library.albums]
      .map((a, i) => ({ a, k: Math.sin(i * 9301 + new Date().getDate() * 49297) }))
      .sort((x, y) => x.k - y.k)
      .slice(0, 12)
      .map((x) => x.a),
  );
</script>

{#if library.loaded && library.tracks.length === 0}
  <Onboarding />
{:else}
  <div class="home" style:--c={player.current?.color ?? "110,80,200"}>
    <div class="glow"></div>
    <div class="page">
      <h1 class="hello">{greeting()}</h1>

      <div class="quick">
        <button class="tile" onclick={() => ui.go({ name: "liked" })}>
          <span class="liked"><Icon name="heart" size={22} filled /></span>
          <span class="tile-title">Любимые треки</span>
          <span
            class="tile-play play-fab"
            role="button"
            tabindex="-1"
            onclick={(e) => {
              e.stopPropagation();
              player.playList(likes.tracks, 0, "Любимые треки");
            }}
            onkeydown={() => {}}
          >
            <Icon name="play" size={18} />
          </span>
        </button>
        {#each quick as a (a.id)}
          <button class="tile" onclick={() => ui.go({ name: "album", id: a.id })}>
            <Cover cover={a.cover} seed={a.title} size={64} radius={0} />
            <span class="tile-title ellipsis">{a.title}</span>
            <span
              class="tile-play play-fab"
              role="button"
              tabindex="-1"
              onclick={(e) => {
                e.stopPropagation();
                player.playList(a.tracks, 0, `Альбом · ${a.title}`);
              }}
              onkeydown={() => {}}
            >
              <Icon name="play" size={18} />
            </span>
          </button>
        {/each}
      </div>

      <section>
        <h2 class="section-title">Недавно добавленные</h2>
        <div class="grid">
          {#each recent.slice(0, 12) as a (a.id)}
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
        </div>
      </section>

      <section>
        <h2 class="section-title">Ваши исполнители</h2>
        <div class="grid">
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
        </div>
      </section>

      {#if library.albums.length > 12}
        <section>
          <h2 class="section-title">Сегодня стоит послушать</h2>
          <div class="grid">
            {#each shuffle as a (a.id)}
              <Card
                title={a.title}
                subtitle={a.artist}
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
  </div>
{/if}

<style>
  .home {
    position: relative;
    margin-top: -64px;
    padding-top: 64px;
  }
  .glow {
    position: absolute;
    inset: 0 0 auto 0;
    height: 340px;
    background: linear-gradient(180deg, rgba(var(--c), 0.45), transparent);
    transition: background 0.8s;
    pointer-events: none;
  }
  .page {
    position: relative;
  }
  .hello {
    font-size: 32px;
    font-weight: 800;
    letter-spacing: -0.03em;
    margin: 8px 0 20px;
  }
  .quick {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 10px;
    margin-bottom: 40px;
  }
  .tile {
    position: relative;
    display: flex;
    align-items: center;
    gap: 14px;
    height: 64px;
    border-radius: 6px;
    overflow: hidden;
    background: rgba(255, 255, 255, 0.08);
    text-align: left;
    transition: background 0.2s;
  }
  .tile:hover {
    background: rgba(255, 255, 255, 0.16);
  }
  .liked {
    display: grid;
    place-items: center;
    width: 64px;
    height: 64px;
    flex: none;
    background: linear-gradient(135deg, #4f46e5, #a78bfa 60%, #e9d5ff);
  }
  .tile-title {
    flex: 1;
    font-weight: 700;
    padding-right: 56px;
  }
  .tile-play {
    position: absolute;
    right: 10px;
    width: 40px;
    height: 40px;
    opacity: 0;
    transition: opacity 0.2s, transform 0.2s;
  }
  .tile:hover .tile-play {
    opacity: 1;
  }
  section {
    margin-bottom: 36px;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(176px, 1fr));
    grid-auto-rows: auto;
    gap: 4px;
    margin: 0 -12px;
  }
</style>
