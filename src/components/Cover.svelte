<script lang="ts">
  import { library } from "../lib/store.svelte";
  import { placeholderGradient } from "../lib/format";
  import Icon from "./Icon.svelte";

  let {
    cover,
    seed = "",
    size = 48,
    radius = 6,
    round = false,
    shadow = false,
  }: {
    cover: string | null;
    seed?: string;
    size?: number | string;
    radius?: number;
    round?: boolean;
    shadow?: boolean;
  } = $props();

  const url = $derived(library.coverUrl(cover));
  let failed = $state(false);
  $effect(() => {
    url;
    failed = false;
  });
  const dim = $derived(typeof size === "number" ? `${size}px` : size);
</script>

<div
  class="cover"
  class:shadow
  style:width={dim}
  style:height={dim}
  style:border-radius={round ? "50%" : `${radius}px`}
  style:background={url && !failed ? undefined : placeholderGradient(seed)}
>
  {#if url && !failed}
    <img src={url} alt="" loading="lazy" decoding="async" draggable="false" onerror={() => (failed = true)} />
  {:else}
    <Icon name={round ? "artist" : "music"} size={typeof size === "number" ? Math.max(14, size * 0.36) : 40} />
  {/if}
</div>

<style>
  .cover {
    position: relative;
    flex: none;
    overflow: hidden;
    display: grid;
    place-items: center;
    color: rgba(255, 255, 255, 0.55);
    background: var(--panel-2);
  }
  .cover.shadow {
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);
  }
  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }
</style>
