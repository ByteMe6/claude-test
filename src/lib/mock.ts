// Demo backend used when the UI runs in a plain browser (no Tauri).
// Lets you work on the design with `pnpm dev` without a native build.
import type { LibraryPayload, Progress, Track } from "./api";

const ALBUMS: [string, string, number, string, string, string[]][] = [
  ["Neon Tides", "Aurora Lane", 2024, "#7c3aed", "#ec4899", ["Glass Horizon", "Midnight Ferry", "Saltwater Radio", "Low Orbit", "Chromatic", "Undertow", "Harbor Lights", "Signal Fade"]],
  ["Paper Planets", "Mira Kode", 2023, "#0ea5e9", "#22d3ee", ["Weightless", "Origami Sky", "Static Bloom", "Sunday Engines", "Cartography", "Small Hours", "Kite"]],
  ["Velvet Static", "The Lumens", 2022, "#f97316", "#facc15", ["Amber", "Copper Wire", "Slow Burn", "Overdrive", "Golden Hour", "Afterglow", "Tape Hiss", "Fever", "Dust"]],
  ["Deep Field", "Oskar Veil", 2021, "#10b981", "#064e3b", ["Hubble", "Redshift", "Event Horizon", "Quiet Matter", "Parallax", "Lagrange"]],
  ["Soft Machines", "Nadia Rhee", 2024, "#f43f5e", "#881337", ["Heartware", "Analog Love", "Reboot", "Circuit Garden", "Patchbay", "Firmware Kiss", "Loopback"]],
  ["Northern Lines", "Aurora Lane", 2021, "#6366f1", "#1e1b4b", ["Polar Night", "Snowblind", "Tundra", "Fjord", "Aurora Borealis", "Driftwood"]],
  ["Concrete Garden", "Kairo", 2023, "#84cc16", "#365314", ["Ivy", "Rooftop", "Overgrown", "Brutalist", "Sprout", "Moss", "Rainwater", "City Bloom"]],
  ["Lucid", "Mira Kode", 2025, "#d946ef", "#4c1d95", ["Dreamstate", "Half Awake", "Lucid", "REM", "Hypnagogia", "Morning Light"]],
];

function cover(a: string, b: string, seed: number): string {
  const cx = 30 + ((seed * 37) % 40);
  const cy = 30 + ((seed * 53) % 40);
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><defs><linearGradient id="g" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="${a}"/><stop offset="1" stop-color="${b}"/></linearGradient><radialGradient id="r"><stop offset="0" stop-color="#fff" stop-opacity=".55"/><stop offset="1" stop-color="#fff" stop-opacity="0"/></radialGradient></defs><rect width="100" height="100" fill="url(#g)"/><circle cx="${cx}" cy="${cy}" r="${22 + (seed % 3) * 8}" fill="url(#r)"/><circle cx="${100 - cx}" cy="${100 - cy}" r="14" fill="none" stroke="#fff" stroke-opacity=".35" stroke-width="1.2"/></svg>`;
  return `data:image/svg+xml;utf8,${encodeURIComponent(svg)}`;
}

function hexToRgb(hex: string) {
  const n = parseInt(hex.slice(1), 16);
  return `${(n >> 16) & 255},${(n >> 8) & 255},${n & 255}`;
}

const now = Math.floor(Date.now() / 1000);
const tracks: Track[] = ALBUMS.flatMap(([album, artist, year, c1, c2, titles], ai) =>
  titles.map((title, ti) => ({
    id: `t${ai}-${ti}`,
    path: `/music/${artist}/${album}/${ti + 1}. ${title}.flac`,
    title,
    artist: ti === 3 && ai % 2 === 0 ? `${artist} feat. Kairo` : artist,
    album,
    albumArtist: artist,
    albumId: `a${ai}`,
    trackNo: ti + 1,
    discNo: 1,
    year,
    genre: "Electronic",
    duration: 150 + ((ai * 97 + ti * 61) % 160),
    cover: cover(c1, c2, ai),
    color: hexToRgb(c1),
    mtime: 0,
    added: now - ai * 86400 * 3 - ti * 60,
  })),
);

let payload: LibraryPayload = { folders: ["~/Music"], tracks, coversDir: "" };

let position = 0;
let duration = 0;
let playing = false;
let progressCb: ((p: Progress) => void) | null = null;
let endedCb: (() => void) | null = null;

setInterval(() => {
  if (!playing) return;
  position += 0.25;
  if (position >= duration) {
    playing = false;
    endedCb?.();
    return;
  }
  progressCb?.({ position, playing });
}, 250);

const ok = async () => {};

export const mockApi = {
  getLibrary: async () => payload,
  addFolder: async (path: string) => (payload = { ...payload, folders: [...payload.folders, path] }),
  removeFolder: async (path: string) =>
    (payload = { ...payload, folders: payload.folders.filter((f) => f !== path) }),
  rescan: async () => payload,
  load: async (path: string, autoplay: boolean, start = 0) => {
    duration = tracks.find((t) => t.path === path)?.duration ?? 200;
    position = start;
    playing = autoplay;
    progressCb?.({ position, playing });
  },
  play: async () => {
    playing = true;
    progressCb?.({ position, playing });
  },
  pause: async () => {
    playing = false;
    progressCb?.({ position, playing });
  },
  seek: async (p: number) => {
    position = p;
  },
  volume: ok,
  stop: async () => {
    playing = false;
  },
  onProgress: async (cb: (p: Progress) => void) => {
    progressCb = cb;
    return () => {};
  },
  onEnded: async (cb: () => void) => {
    endedCb = cb;
    return () => {};
  },
  onError: async (_cb: (msg: string) => void) => () => {},
};
