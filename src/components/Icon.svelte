<script lang="ts" module>
  // Lucide-style 24×24 stroke icons, inlined so nothing is loaded at runtime.
  const PATHS: Record<string, string> = {
    home: '<path d="M3 10.5 12 3l9 7.5V20a1 1 0 0 1-1 1h-5v-6h-6v6H4a1 1 0 0 1-1-1z"/>',
    search: '<circle cx="11" cy="11" r="7"/><path d="m20 20-3.5-3.5"/>',
    library: '<path d="M4 4v16M9 4v16"/><path d="m14 4.5 5 15"/>',
    music: '<path d="M9 18V5l12-2v13"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="16" r="3"/>',
    album: '<circle cx="12" cy="12" r="9"/><circle cx="12" cy="12" r="2.5"/>',
    artist: '<circle cx="12" cy="8" r="4"/><path d="M4 21a8 8 0 0 1 16 0"/>',
    heart: '<path d="M12 20s-7.5-4.6-9.3-9.4C1.5 7.3 3.7 4 7 4c2 0 3.6 1.1 5 3 1.4-1.9 3-3 5-3 3.3 0 5.5 3.3 4.3 6.6C19.5 15.4 12 20 12 20z"/>',
    play: '<path d="M7 4.5v15a1 1 0 0 0 1.5.86l12.4-7.5a1 1 0 0 0 0-1.72L8.5 3.64A1 1 0 0 0 7 4.5z" fill="currentColor" stroke="none"/>',
    pause: '<rect x="5.5" y="4" width="4.5" height="16" rx="1.2" fill="currentColor" stroke="none"/><rect x="14" y="4" width="4.5" height="16" rx="1.2" fill="currentColor" stroke="none"/>',
    next: '<path d="M5 5.2v13.6a.8.8 0 0 0 1.2.7L16 13v6h2.5V5H16v6L6.2 4.5a.8.8 0 0 0-1.2.7z" fill="currentColor" stroke="none"/>',
    prev: '<path d="M19 5.2v13.6a.8.8 0 0 1-1.2.7L8 13v6H5.5V5H8v6l9.8-6.5a.8.8 0 0 1 1.2.7z" fill="currentColor" stroke="none"/>',
    shuffle: '<path d="M3 7h3.5c2 0 3.2.9 4.3 2.6l2.4 4.8c1.1 1.7 2.3 2.6 4.3 2.6H21"/><path d="M3 17h3.5c1.3 0 2.2-.4 3-1.1M14.5 8.1c.8-.7 1.7-1.1 3-1.1H21"/><path d="m18 4 3 3-3 3M18 14l3 3-3 3"/>',
    repeat: '<path d="M17 2l4 4-4 4"/><path d="M3 11V9a3 3 0 0 1 3-3h15"/><path d="M7 22l-4-4 4-4"/><path d="M21 13v2a3 3 0 0 1-3 3H3"/>',
    volume: '<path d="M11 5 6 9H3v6h3l5 4z" fill="currentColor"/><path d="M15.5 8.5a5 5 0 0 1 0 7M18.5 5.5a9 9 0 0 1 0 13"/>',
    "volume-low": '<path d="M11 5 6 9H3v6h3l5 4z" fill="currentColor"/><path d="M15.5 8.5a5 5 0 0 1 0 7"/>',
    "volume-mute": '<path d="M11 5 6 9H3v6h3l5 4z" fill="currentColor"/><path d="m16 9 5 6M21 9l-5 6"/>',
    queue: '<path d="M3 6h13M3 12h13M3 18h8"/><path d="M18 14v7l4-3.5z" fill="currentColor"/>',
    expand: '<path d="M15 3h6v6M9 21H3v-6M21 3l-7 7M3 21l7-7"/>',
    "chevron-left": '<path d="m15 18-6-6 6-6"/>',
    "chevron-right": '<path d="m9 18 6-6-6-6"/>',
    "chevron-down": '<path d="m6 9 6 6 6-6"/>',
    folder: '<path d="M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>',
    plus: '<path d="M12 5v14M5 12h14"/>',
    trash: '<path d="M4 7h16M10 11v6M14 11v6M6 7l1 13h10l1-13M9 7V4h6v3"/>',
    refresh: '<path d="M21 12a9 9 0 1 1-2.6-6.4L21 8"/><path d="M21 3v5h-5"/>',
    settings: '<circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.7 1.7 0 0 0 .3 1.8l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.7 1.7 0 0 0-1.8-.3 1.7 1.7 0 0 0-1 1.5V21a2 2 0 1 1-4 0v-.1a1.7 1.7 0 0 0-1.1-1.5 1.7 1.7 0 0 0-1.8.3l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.7 1.7 0 0 0 .3-1.8 1.7 1.7 0 0 0-1.5-1H3a2 2 0 1 1 0-4h.1a1.7 1.7 0 0 0 1.5-1.1 1.7 1.7 0 0 0-.3-1.8l-.1-.1a2 2 0 1 1 2.8-2.8l.1.1a1.7 1.7 0 0 0 1.8.3H9a1.7 1.7 0 0 0 1-1.5V3a2 2 0 1 1 4 0v.1a1.7 1.7 0 0 0 1 1.5 1.7 1.7 0 0 0 1.8-.3l.1-.1a2 2 0 1 1 2.8 2.8l-.1.1a1.7 1.7 0 0 0-.3 1.8V9a1.7 1.7 0 0 0 1.5 1H21a2 2 0 1 1 0 4h-.1a1.7 1.7 0 0 0-1.5 1z"/>',
    clock: '<circle cx="12" cy="12" r="9"/><path d="M12 7v5l3 2"/>',
    x: '<path d="M18 6 6 18M6 6l12 12"/>',
    more: '<circle cx="5" cy="12" r="1.6" fill="currentColor"/><circle cx="12" cy="12" r="1.6" fill="currentColor"/><circle cx="19" cy="12" r="1.6" fill="currentColor"/>',
  };
</script>

<script lang="ts">
  let {
    name,
    size = 20,
    filled = false,
    class: cls = "",
  }: { name: string; size?: number; filled?: boolean; class?: string } = $props();
</script>

<svg
  class={cls}
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill={filled ? "currentColor" : "none"}
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  {@html PATHS[name] ?? ""}
</svg>
