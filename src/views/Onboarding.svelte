<script lang="ts">
  import { pickFolder } from "../lib/folders";
  import { library } from "../lib/store.svelte";
  import Icon from "../components/Icon.svelte";
</script>

<div class="onboarding">
  <div class="disc"></div>
  <h1>Ваша музыка — здесь</h1>
  <p>Выберите папку с музыкой. Lyra прочитает теги и обложки и соберёт медиатеку — всё локально, без аккаунтов и интернета.</p>
  <button class="pill primary" onclick={pickFolder} disabled={library.scanning}>
    {#if library.scanning}
      <Icon name="refresh" size={18} class="spin" /> Сканирую…
    {:else}
      <Icon name="folder" size={18} /> Выбрать папку
    {/if}
  </button>
  <span class="formats">MP3 · FLAC · OGG · M4A/AAC · ALAC · WAV · AIFF</span>
</div>

<style>
  .onboarding {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 16px;
    min-height: calc(100vh - 220px);
    padding: 32px;
    text-align: center;
  }
  .disc {
    width: 140px;
    height: 140px;
    margin-bottom: 12px;
    border-radius: 50%;
    background:
      radial-gradient(circle, #0b0b0f 0 14%, transparent 15% 100%),
      repeating-radial-gradient(circle, rgba(0, 0, 0, 0.25) 0 2px, transparent 2px 7px),
      var(--brand);
    box-shadow: 0 20px 60px rgba(236, 72, 153, 0.25);
    animation: spin 8s linear infinite;
  }
  h1 {
    font-size: 34px;
    font-weight: 850;
    letter-spacing: -0.03em;
  }
  p {
    max-width: 460px;
    color: var(--text-2);
    font-size: 15px;
  }
  .pill {
    height: 48px;
    padding: 0 28px;
    font-size: 15px;
    margin-top: 8px;
  }
  .formats {
    font-size: 12px;
    color: var(--text-3);
    letter-spacing: 0.04em;
  }
  :global(.spin) {
    animation: spin 1s linear infinite;
  }
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
