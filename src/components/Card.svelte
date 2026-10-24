<script lang="ts">
  import Cover from "./Cover.svelte";
  import Icon from "./Icon.svelte";

  let {
    title,
    subtitle,
    cover,
    seed,
    round = false,
    playing = false,
    onopen,
    onplay,
  }: {
    title: string;
    subtitle: string;
    cover: string | null;
    seed: string;
    round?: boolean;
    playing?: boolean;
    onopen: () => void;
    onplay: () => void;
  } = $props();
</script>

<div class="card" role="button" tabindex="0" onclick={onopen} onkeydown={(e) => e.key === "Enter" && onopen()}>
  <div class="art">
    <Cover {cover} {seed} size="100%" radius={8} {round} shadow />
    <button
      class="play-fab small"
      class:visible={playing}
      onclick={(e) => {
        e.stopPropagation();
        onplay();
      }}
      aria-label="Играть"
    >
      <Icon name={playing ? "pause" : "play"} size={20} />
    </button>
  </div>
  <p class="title ellipsis" class:center={round}>{title}</p>
  <p class="sub ellipsis" class:center={round}>{subtitle}</p>
</div>

<style>
  .card {
    padding: 12px;
    border-radius: 12px;
    cursor: pointer;
    transition: background 0.2s;
    min-width: 0;
    /* Off-screen cards skip layout and paint — keeps big grids cheap. */
    content-visibility: auto;
    contain-intrinsic-size: auto 260px;
  }
  .card:hover {
    background: var(--hover);
  }
  .art {
    position: relative;
    aspect-ratio: 1;
    margin-bottom: 12px;
  }
  .art :global(.cover) {
    aspect-ratio: 1;
    height: auto !important;
  }
  .small {
    position: absolute;
    right: 8px;
    bottom: 8px;
    width: 46px;
    height: 46px;
    opacity: 0;
    transform: translateY(8px);
    transition: opacity 0.2s, transform 0.25s var(--ease), filter 0.15s;
  }
  .card:hover .small,
  .small.visible {
    opacity: 1;
    transform: translateY(0);
  }
  .small:hover {
    transform: scale(1.06) !important;
  }
  .title {
    font-weight: 650;
    margin-bottom: 2px;
  }
  .sub {
    font-size: 13px;
    color: var(--text-2);
  }
  .center {
    text-align: center;
  }
</style>
