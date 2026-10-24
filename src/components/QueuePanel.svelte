<script lang="ts">
  import { player, ui } from "../lib/store.svelte";
  import { time } from "../lib/format";
  import Cover from "./Cover.svelte";
  import Equalizer from "./Equalizer.svelte";
  import Icon from "./Icon.svelte";

  // The queue can be huge (whole library) — rendering the next 100 is plenty.
  const next = $derived(player.upNext.slice(0, 100));
</script>

<aside class="queue">
  <header>
    <h3>Очередь</h3>
    <button class="icon-btn" onclick={() => ui.toggleQueue()} title="Закрыть"><Icon name="x" size={18} /></button>
  </header>

  <div class="scroll">
    {#if player.current}
      {@const t = player.current}
      <p class="label">Сейчас играет</p>
      <div class="item current">
        <Cover cover={t.cover} seed={t.album} size={44} radius={4} />
        <div class="meta">
          <span class="title ellipsis">{t.title}</span>
          <span class="sub ellipsis">{t.artist}</span>
        </div>
        <Equalizer playing={player.playing} />
      </div>

      {#if next.length}
        <div class="label row">
          <span class="ellipsis">Далее{player.source ? ` · ${player.source}` : ""}</span>
          <button class="clear" onclick={() => player.clearUpNext()}>Очистить</button>
        </div>
        {#each next as q, i (q.id + ":" + i)}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <div class="item" role="button" tabindex="-1" ondblclick={() => player.jumpTo(i)} oncontextmenu={(e) => ui.trackMenu(e, q)}>
            <button class="cover-play" onclick={() => player.jumpTo(i)} aria-label="Играть">
              <Cover cover={q.cover} seed={q.album} size={44} radius={4} />
              <span class="overlay"><Icon name="play" size={16} /></span>
            </button>
            <div class="meta">
              <span class="title ellipsis">{q.title}</span>
              <span class="sub ellipsis">{q.artist} · {time(q.duration)}</span>
            </div>
            <button class="icon-btn rm" onclick={() => player.removeFromUpNext(i)} title="Убрать">
              <Icon name="x" size={16} />
            </button>
          </div>
        {/each}
        {#if player.upNext.length > next.length}
          <p class="more">и ещё {player.upNext.length - next.length}…</p>
        {/if}
      {:else}
        <p class="empty">Дальше ничего нет. Добавьте треки через правый клик → «Добавить в очередь».</p>
      {/if}
    {:else}
      <p class="empty">Очередь пуста. Включите альбом или трек.</p>
    {/if}
  </div>
</aside>

<style>
  .queue {
    display: flex;
    flex-direction: column;
    min-height: 0;
    background: var(--panel);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 10px 6px 18px;
  }
  h3 {
    font-size: 16px;
    font-weight: 700;
  }
  .scroll {
    overflow-y: auto;
    padding: 0 8px 16px;
    flex: 1;
  }
  .label {
    padding: 14px 10px 8px;
    font-size: 13px;
    font-weight: 700;
    color: var(--text-2);
  }
  .label.row {
    display: flex;
    justify-content: space-between;
    gap: 8px;
  }
  .clear {
    flex: none;
    color: var(--text-3);
    font-size: 12px;
    font-weight: 600;
  }
  .clear:hover {
    color: var(--text);
  }
  .item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 6px 8px;
    border-radius: 8px;
  }
  .item:hover {
    background: var(--hover);
  }
  .current .title {
    color: var(--accent);
  }
  .meta {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }
  .title {
    font-weight: 500;
  }
  .sub {
    font-size: 12.5px;
    color: var(--text-2);
  }
  .cover-play {
    position: relative;
  }
  .overlay {
    position: absolute;
    inset: 0;
    display: grid;
    place-items: center;
    background: rgba(0, 0, 0, 0.5);
    border-radius: 4px;
    opacity: 0;
  }
  .item:hover .overlay {
    opacity: 1;
  }
  .rm {
    opacity: 0;
  }
  .item:hover .rm {
    opacity: 1;
  }
  .empty,
  .more {
    padding: 16px 10px;
    color: var(--text-3);
    font-size: 13px;
  }
</style>
