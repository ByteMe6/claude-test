<script lang="ts">
  import { library, likes, player, ui } from "../lib/store.svelte";
  import { time } from "../lib/format";
  import Cover from "./Cover.svelte";
  import Icon from "./Icon.svelte";
  import Slider from "./Slider.svelte";

  const t = $derived(player.current);
  const bg = $derived(t ? library.coverUrl(t.cover) : null);
  let scrub = $state<number | null>(null);
</script>

{#if t}
  <div class="np" style:--c={t.color ?? "120,120,140"}>
    {#if bg}
      <div class="bg" style:background-image="url('{bg}')"></div>
    {/if}
    <div class="shade"></div>

    <button class="icon-btn close" onclick={() => (ui.nowPlayingOpen = false)} title="Свернуть">
      <Icon name="chevron-down" size={24} />
    </button>

    <div class="content">
      {#key t.id}
        <div class="art">
          <Cover cover={t.cover} seed={t.album} size="min(52vh, 480px)" radius={14} shadow />
        </div>
      {/key}

      <div class="info">
        <div class="titles">
          <h1 class="ellipsis">{t.title}</h1>
          <button class="artist link" onclick={() => ui.go({ name: "artist", name_: t.albumArtist })}>{t.artist}</button>
          <button class="album link" onclick={() => ui.go({ name: "album", id: t.albumId })}>{t.album}{t.year ? ` · ${t.year}` : ""}</button>
        </div>
        <button class="icon-btn big" class:active={likes.has(t.id)} onclick={() => likes.toggle(t.id)}>
          <Icon name="heart" size={26} filled={likes.has(t.id)} />
        </button>
      </div>

      <div class="progress">
        <Slider
          value={player.position}
          max={t.duration}
          smooth={player.playing}
          label="Позиция"
          oninput={(v) => (scrub = v)}
          oncommit={(v) => {
            scrub = null;
            player.seek(v);
          }}
        />
        <div class="times tnum"><span>{time(scrub ?? player.position)}</span><span>-{time(t.duration - (scrub ?? player.position))}</span></div>
      </div>

      <div class="controls">
        <button class="icon-btn" class:active={player.shuffle} onclick={() => player.toggleShuffle()}><Icon name="shuffle" size={22} /></button>
        <button class="icon-btn big" onclick={() => player.prev()}><Icon name="prev" size={30} /></button>
        <button class="play" onclick={() => player.toggle()}>
          <Icon name={player.playing ? "pause" : "play"} size={30} />
        </button>
        <button class="icon-btn big" onclick={() => player.next()}><Icon name="next" size={30} /></button>
        <button class="icon-btn" class:active={player.repeat !== "off"} onclick={() => player.cycleRepeat()}>
          <Icon name="repeat" size={22} />
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .np {
    position: fixed;
    inset: 0;
    z-index: 50;
    display: grid;
    place-items: center;
    background: rgb(var(--c));
    overflow: hidden;
    animation: rise 0.35s var(--ease);
  }
  @keyframes rise {
    from {
      opacity: 0;
      transform: translateY(24px);
    }
  }
  .bg {
    position: absolute;
    inset: -80px;
    background-size: cover;
    background-position: center;
    filter: blur(70px) saturate(1.4);
    opacity: 0.75;
    transform: scale(1.1);
  }
  .shade {
    position: absolute;
    inset: 0;
    background: radial-gradient(ellipse at 50% 30%, rgba(0, 0, 0, 0.15), rgba(0, 0, 0, 0.65));
  }
  .close {
    position: absolute;
    top: 20px;
    left: 20px;
    z-index: 2;
    width: 44px;
    height: 44px;
    color: var(--text);
    background: rgba(0, 0, 0, 0.25);
  }
  .content {
    position: relative;
    z-index: 1;
    width: min(52vh, 480px);
    display: flex;
    flex-direction: column;
    gap: 22px;
  }
  .art {
    animation: pop 0.45s var(--ease);
  }
  @keyframes pop {
    from {
      opacity: 0;
      transform: scale(0.94);
    }
  }
  .info {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  .titles {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }
  h1 {
    max-width: 100%;
    font-size: 28px;
    font-weight: 800;
    letter-spacing: -0.02em;
  }
  .artist {
    font-size: 17px;
    color: rgba(255, 255, 255, 0.85);
  }
  .album {
    font-size: 13px;
    color: rgba(255, 255, 255, 0.6);
    margin-top: 2px;
  }
  .big {
    width: 48px;
    height: 48px;
    color: var(--text);
  }
  .times {
    display: flex;
    justify-content: space-between;
    margin-top: 4px;
    font-size: 12px;
    color: rgba(255, 255, 255, 0.7);
  }
  .controls {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .controls .icon-btn {
    color: rgba(255, 255, 255, 0.85);
  }
  .controls .icon-btn.active {
    color: var(--text);
    background: rgba(255, 255, 255, 0.15);
  }
  .play {
    display: grid;
    place-items: center;
    width: 72px;
    height: 72px;
    border-radius: 50%;
    background: var(--text);
    color: #0b0b0f;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.35);
    transition: transform 0.15s;
  }
  .play:hover {
    transform: scale(1.06);
  }
</style>
