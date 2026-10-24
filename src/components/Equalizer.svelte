<script lang="ts">
  let { playing = true }: { playing?: boolean } = $props();
</script>

<span class="eq" class:paused={!playing} aria-label="Сейчас играет">
  <i></i><i></i><i></i><i></i>
</span>

<style>
  .eq {
    display: inline-flex;
    align-items: flex-end;
    gap: 2px;
    width: 14px;
    height: 14px;
  }
  i {
    flex: 1;
    background: var(--accent);
    border-radius: 1px;
    animation: bounce 0.9s ease-in-out infinite;
    transform-origin: bottom;
  }
  i:nth-child(2) {
    animation-delay: -0.3s;
  }
  i:nth-child(3) {
    animation-delay: -0.6s;
  }
  i:nth-child(4) {
    animation-delay: -0.15s;
  }
  .paused i {
    animation-play-state: paused;
  }
  @keyframes bounce {
    0%,
    100% {
      height: 25%;
    }
    50% {
      height: 100%;
    }
  }
</style>
