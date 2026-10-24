<script lang="ts">
  import { likes, player, ui } from "../lib/store.svelte";
  import { time } from "../lib/format";
  import Cover from "./Cover.svelte";
  import Icon from "./Icon.svelte";
  import Slider from "./Slider.svelte";

  const t = $derived(player.current);
  let scrub = $state<number | null>(null);
  const volIcon = $derived(
    player.muted || player.volume === 0 ? "volume-mute" : player.volume < 0.5 ? "volume-low" : "volume",
  );
</script>

<footer class="bar">
  <div class="now">
    {#if t}
      <button class="cover-btn" onclick={() => (ui.nowPlayingOpen = true)} title="Открыть плеер">
        <Cover cover={t.cover} seed={t.album} size={56} radius={6} />
        <span class="expand"><Icon name="chevron-down" size={16} /></span>
      </button>
      <div class="meta">
        <button class="title ellipsis link" onclick={() => ui.go({ name: "album", id: t.albumId })}>{t.title}</button>
        <button class="artist ellipsis link" onclick={() => ui.go({ name: "artist", name_: t.albumArtist })}>{t.artist}</button>
      </div>
      <button class="icon-btn" class:active={likes.has(t.id)} onclick={() => likes.toggle(t.id)} title="Нравится">
        <Icon name="heart" size={18} filled={likes.has(t.id)} />
      </button>
    {/if}
  </div>

  <div class="center">
    <div class="controls">
      <button class="icon-btn" class:active={player.shuffle} onclick={() => player.toggleShuffle()} title="Перемешать">
        <Icon name="shuffle" size={18} />
        {#if player.shuffle}<i class="dot"></i>{/if}
      </button>
      <button class="icon-btn" onclick={() => player.prev()} title="Назад" disabled={!t}>
        <Icon name="prev" size={20} />
      </button>
      <button class="play" onclick={() => player.toggle()} title={player.playing ? "Пауза" : "Играть"}>
        <Icon name={player.playing ? "pause" : "play"} size={18} />
      </button>
      <button class="icon-btn" onclick={() => player.next()} title="Далее" disabled={!t}>
        <Icon name="next" size={20} />
      </button>
      <button
        class="icon-btn"
        class:active={player.repeat !== "off"}
        onclick={() => player.cycleRepeat()}
        title={player.repeat === "one" ? "Повтор трека" : player.repeat === "all" ? "Повтор очереди" : "Без повтора"}
      >
        <Icon name="repeat" size={18} />
        {#if player.repeat === "one"}<b class="one">1</b>{/if}
        {#if player.repeat !== "off"}<i class="dot"></i>{/if}
      </button>
    </div>
    <div class="progress">
      <span class="tnum time">{time(scrub ?? player.position)}</span>
      <Slider
        value={player.position}
        max={t?.duration ?? 1}
        smooth={player.playing}
        label="Позиция"
        oninput={(v) => (scrub = v)}
        oncommit={(v) => {
          scrub = null;
          player.seek(v);
        }}
      />
      <span class="tnum time">{time(t?.duration ?? 0)}</span>
    </div>
  </div>

  <div class="right">
    <button class="icon-btn" class:active={ui.queueOpen} onclick={() => ui.toggleQueue()} title="Очередь">
      <Icon name="queue" size={18} />
    </button>
    <button class="icon-btn" onclick={() => player.toggleMute()} title="Звук">
      <Icon name={volIcon} size={18} />
    </button>
    <div class="vol">
      <Slider
        value={player.muted ? 0 : player.volume}
        label="Громкость"
        oninput={(v) => player.setVolume(v)}
        oncommit={(v) => player.setVolume(v)}
      />
    </div>
    <button class="icon-btn" onclick={() => (ui.nowPlayingOpen = !ui.nowPlayingOpen)} title="Полноэкранный режим" disabled={!t}>
      <Icon name="expand" size={16} />
    </button>
  </div>
</footer>

<style>
  .bar {
    display: grid;
    grid-template-columns: minmax(180px, 1fr) minmax(320px, 2fr) minmax(180px, 1fr);
    align-items: center;
    gap: 16px;
    height: var(--player-h);
    padding: 0 16px 0 12px;
  }
  .now {
    display: flex;
    align-items: center;
    gap: 14px;
    min-width: 0;
  }
  .cover-btn {
    position: relative;
    border-radius: 6px;
  }
  .expand {
    position: absolute;
    top: 4px;
    right: 4px;
    display: grid;
    place-items: center;
    width: 22px;
    height: 22px;
    border-radius: 50%;
    background: rgba(0, 0, 0, 0.7);
    transform: rotate(180deg);
    opacity: 0;
    transition: opacity 0.15s;
  }
  .cover-btn:hover .expand {
    opacity: 1;
  }
  .meta {
    display: flex;
    flex-direction: column;
    min-width: 0;
    align-items: flex-start;
  }
  .meta button {
    max-width: 100%;
    text-align: left;
  }
  .title {
    font-weight: 600;
    font-size: 14.5px;
  }
  .artist {
    font-size: 12.5px;
    color: var(--text-2);
  }
  .center {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    max-width: 720px;
    width: 100%;
    justify-self: center;
  }
  .controls {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .controls .icon-btn {
    position: relative;
  }
  .dot {
    position: absolute;
    bottom: 1px;
    left: 50%;
    width: 4px;
    height: 4px;
    margin-left: -2px;
    border-radius: 50%;
    background: var(--accent);
  }
  .one {
    position: absolute;
    top: 6px;
    right: 4px;
    font-size: 9px;
    font-weight: 800;
    background: var(--panel);
    border-radius: 4px;
    padding: 0 2px;
    line-height: 1.2;
  }
  .play {
    display: grid;
    place-items: center;
    width: 38px;
    height: 38px;
    margin: 0 6px;
    border-radius: 50%;
    background: var(--text);
    color: #0b0b0f;
    transition: transform 0.15s;
  }
  .play:hover {
    transform: scale(1.07);
  }
  .play:active {
    transform: scale(0.95);
  }
  .progress {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
  }
  .progress :global(.slider) {
    flex: 1;
  }
  .time {
    min-width: 40px;
    font-size: 11.5px;
    color: var(--text-3);
    text-align: center;
  }
  .right {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 4px;
  }
  .vol {
    width: 110px;
    margin-right: 8px;
  }
</style>
