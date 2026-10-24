<script lang="ts">
  import { ui } from "../lib/store.svelte";
  import Icon from "./Icon.svelte";

  let menuEl: HTMLDivElement | undefined = $state();
  let pos = $state({ x: 0, y: 0 });

  // Keep the context menu inside the window.
  $effect(() => {
    const m = ui.menu;
    if (!m || !menuEl) return;
    const r = menuEl.getBoundingClientRect();
    pos = {
      x: Math.min(m.x, window.innerWidth - r.width - 8),
      y: Math.min(m.y, window.innerHeight - r.height - 8),
    };
  });

  function close() {
    ui.menu = null;
  }
</script>

<svelte:window onclick={close} onblur={close} onresize={close} onkeydown={(e) => e.key === "Escape" && close()} />

{#if ui.menu}
  <div class="menu" bind:this={menuEl} style:left="{pos.x}px" style:top="{pos.y}px" role="menu">
    {#each ui.menu.items as item}
      <button
        role="menuitem"
        class:danger={item.danger}
        onclick={() => {
          close();
          item.action();
        }}
      >
        {#if item.icon}<Icon name={item.icon} size={16} />{/if}
        {item.label}
      </button>
    {/each}
  </div>
{/if}

<div class="toasts">
  {#each ui.toasts as t (t.id)}
    <div class="toast">{t.text}</div>
  {/each}
</div>

<style>
  .menu {
    position: fixed;
    z-index: 100;
    min-width: 220px;
    padding: 4px;
    border-radius: 8px;
    background: #24242c;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.55);
    border: 1px solid var(--border);
    animation: fade 0.12s ease-out;
  }
  .menu button {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    padding: 10px 12px;
    border-radius: 4px;
    text-align: left;
    color: var(--text);
  }
  .menu button:hover {
    background: var(--hover-strong);
  }
  .menu .danger {
    color: #f87171;
  }
  .toasts {
    position: fixed;
    bottom: calc(var(--player-h) + 20px);
    left: 50%;
    z-index: 120;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    transform: translateX(-50%);
    pointer-events: none;
  }
  .toast {
    padding: 10px 18px;
    border-radius: 8px;
    background: var(--text);
    color: #0b0b0f;
    font-weight: 600;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.45);
    animation: fade 0.2s ease-out;
  }
  @keyframes fade {
    from {
      opacity: 0;
      transform: translateY(6px);
    }
  }
</style>
