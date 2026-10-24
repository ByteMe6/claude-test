<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    color = null,
    kind,
    title,
    art,
    meta,
  }: {
    color?: string | null;
    kind: string;
    title: string;
    art: Snippet;
    meta: Snippet;
  } = $props();

  // Long titles get a smaller font so they fit in two lines.
  const size = $derived(title.length > 40 ? 40 : title.length > 22 ? 56 : 76);
</script>

<header class="hero" style:--c={color ?? "88,88,110"}>
  <div class="art">{@render art()}</div>
  <div class="text">
    <span class="kind">{kind}</span>
    <h1 style:font-size="{size}px">{title}</h1>
    <div class="meta">{@render meta()}</div>
  </div>
</header>

<style>
  .hero {
    position: relative;
    display: flex;
    align-items: flex-end;
    gap: 28px;
    padding: 84px 32px 28px;
    margin-top: -64px;
    background: linear-gradient(180deg, rgba(var(--c), 0.9), rgba(var(--c), 0.45));
  }
  .hero::after {
    content: "";
    position: absolute;
    left: 0;
    right: 0;
    top: 100%;
    height: 240px;
    background: linear-gradient(180deg, rgba(var(--c), 0.28), transparent);
    pointer-events: none;
  }
  .art {
    flex: none;
    width: 232px;
    height: 232px;
    animation: pop 0.4s var(--ease);
  }
  @keyframes pop {
    from {
      opacity: 0;
      transform: scale(0.96);
    }
  }
  .text {
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .kind {
    font-size: 13px;
    font-weight: 600;
  }
  h1 {
    font-weight: 900;
    letter-spacing: -0.045em;
    line-height: 1.02;
    margin-bottom: 8px;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-shadow: 0 2px 24px rgba(0, 0, 0, 0.2);
  }
  .meta {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 6px;
    font-size: 14px;
    color: rgba(255, 255, 255, 0.85);
  }
  .meta :global(b) {
    color: var(--text);
  }
</style>
