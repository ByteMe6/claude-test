<script lang="ts">
  let {
    value,
    max = 1,
    oncommit,
    oninput,
    smooth = false,
    label = "",
  }: {
    value: number;
    max?: number;
    /** Called once when the user releases the handle. */
    oncommit: (v: number) => void;
    /** Called continuously while dragging. */
    oninput?: (v: number) => void;
    /** Animate between value updates (for the playback position). */
    smooth?: boolean;
    label?: string;
  } = $props();

  let track: HTMLDivElement;
  let dragging = $state(false);
  let dragValue = $state(0);

  const shown = $derived(dragging ? dragValue : value);
  const pct = $derived(max > 0 ? Math.min(100, Math.max(0, (shown / max) * 100)) : 0);

  function valueAt(e: PointerEvent) {
    const r = track.getBoundingClientRect();
    return Math.min(1, Math.max(0, (e.clientX - r.left) / r.width)) * max;
  }

  function down(e: PointerEvent) {
    if (e.button !== 0) return;
    track.setPointerCapture(e.pointerId);
    dragging = true;
    dragValue = valueAt(e);
    oninput?.(dragValue);
  }
  function move(e: PointerEvent) {
    if (!dragging) return;
    dragValue = valueAt(e);
    oninput?.(dragValue);
  }
  function up(e: PointerEvent) {
    if (!dragging) return;
    dragging = false;
    oncommit(valueAt(e));
  }
  function key(e: KeyboardEvent) {
    const step = max / 20;
    if (e.key === "ArrowRight" || e.key === "ArrowUp") oncommit(Math.min(max, value + step));
    else if (e.key === "ArrowLeft" || e.key === "ArrowDown") oncommit(Math.max(0, value - step));
    else return;
    e.preventDefault();
    e.stopPropagation();
  }
</script>

<div
  class="slider"
  class:dragging
  class:smooth={smooth && !dragging}
  bind:this={track}
  role="slider"
  tabindex="0"
  aria-label={label}
  aria-valuemin={0}
  aria-valuemax={max}
  aria-valuenow={Math.round(shown)}
  onpointerdown={down}
  onpointermove={move}
  onpointerup={up}
  onpointercancel={() => (dragging = false)}
  onkeydown={key}
  style:--pct="{pct}%"
>
  <div class="rail"><div class="fill"></div></div>
  <div class="knob"></div>
</div>

<style>
  .slider {
    position: relative;
    height: 16px;
    display: flex;
    align-items: center;
    cursor: pointer;
    touch-action: none;
    outline: none;
  }
  .rail {
    position: relative;
    width: 100%;
    height: 4px;
    border-radius: 4px;
    background: rgba(255, 255, 255, 0.16);
    overflow: hidden;
  }
  .fill {
    position: absolute;
    inset: 0;
    width: var(--pct);
    background: var(--text);
    border-radius: 4px;
  }
  .knob {
    position: absolute;
    left: var(--pct);
    width: 12px;
    height: 12px;
    margin-left: -6px;
    border-radius: 50%;
    background: var(--text);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.5);
    opacity: 0;
    transform: scale(0.6);
    transition: opacity 0.15s, transform 0.15s;
  }
  .smooth .fill {
    transition: width 0.25s linear;
  }
  .smooth .knob {
    transition: left 0.25s linear, opacity 0.15s, transform 0.15s;
  }
  .slider:hover .fill,
  .slider.dragging .fill,
  .slider:focus-visible .fill {
    background: var(--accent);
  }
  .slider:hover .knob,
  .slider.dragging .knob,
  .slider:focus-visible .knob {
    opacity: 1;
    transform: scale(1);
  }
</style>
