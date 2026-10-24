import { api as tauriApi, fileSrc, type LibraryPayload, type Track } from "./api";
import { mockApi } from "./mock";

const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
/** Outside of Tauri (plain `pnpm dev` in a browser) the UI runs on demo data. */
export const api = isTauri ? tauriApi : mockApi;
export const demoMode = !isTauri;

export interface Album {
  id: string;
  title: string;
  artist: string;
  year: number | null;
  cover: string | null;
  color: string | null;
  tracks: Track[];
  duration: number;
  added: number;
}

export interface Artist {
  name: string;
  albums: Album[];
  tracks: Track[];
  cover: string | null;
  color: string | null;
}

const collator = new Intl.Collator("ru", { sensitivity: "base", numeric: true });
export const compare = (a: string, b: string) => collator.compare(a, b);

function load<T>(key: string, fallback: T): T {
  try {
    const raw = localStorage.getItem(`lyra:${key}`);
    return raw ? (JSON.parse(raw) as T) : fallback;
  } catch {
    return fallback;
  }
}
function save(key: string, value: unknown) {
  try {
    localStorage.setItem(`lyra:${key}`, JSON.stringify(value));
  } catch {
    /* storage unavailable — not critical */
  }
}

function buildAlbums(tracks: Track[]): Album[] {
  const map = new Map<string, Album>();
  for (const t of tracks) {
    let a = map.get(t.albumId);
    if (!a) {
      a = {
        id: t.albumId,
        title: t.album,
        artist: t.albumArtist,
        year: t.year,
        cover: t.cover,
        color: t.color,
        tracks: [],
        duration: 0,
        added: t.added,
      };
      map.set(t.albumId, a);
    }
    a.tracks.push(t);
    a.duration += t.duration;
    a.added = Math.max(a.added, t.added);
    a.cover ??= t.cover;
    a.color ??= t.color;
    a.year ??= t.year;
  }
  for (const a of map.values()) {
    a.tracks.sort(
      (x, y) =>
        (x.discNo ?? 1) - (y.discNo ?? 1) ||
        (x.trackNo ?? 1e9) - (y.trackNo ?? 1e9) ||
        compare(x.title, y.title),
    );
  }
  return [...map.values()].sort((x, y) => compare(x.title, y.title));
}

function buildArtists(albums: Album[]): Artist[] {
  const map = new Map<string, Artist>();
  for (const al of albums) {
    const key = al.artist.toLowerCase();
    let ar = map.get(key);
    if (!ar) {
      ar = { name: al.artist, albums: [], tracks: [], cover: null, color: null };
      map.set(key, ar);
    }
    ar.albums.push(al);
    ar.tracks.push(...al.tracks);
    ar.cover ??= al.cover;
    ar.color ??= al.color;
  }
  for (const ar of map.values()) ar.albums.sort((a, b) => (b.year ?? 0) - (a.year ?? 0));
  return [...map.values()].sort((a, b) => compare(a.name, b.name));
}

class LibraryStore {
  tracks = $state.raw<Track[]>([]);
  folders = $state.raw<string[]>([]);
  coversDir = $state("");
  scanning = $state(false);
  loaded = $state(false);

  byId = $derived(new Map(this.tracks.map((t) => [t.id, t])));
  albums = $derived(buildAlbums(this.tracks));
  albumById = $derived(new Map(this.albums.map((a) => [a.id, a])));
  artists = $derived(buildArtists(this.albums));
  totalDuration = $derived(this.tracks.reduce((s, t) => s + t.duration, 0));

  private apply(p: LibraryPayload) {
    this.tracks = p.tracks;
    this.folders = p.folders;
    this.coversDir = p.coversDir;
    this.loaded = true;
  }

  async init() {
    this.apply(await api.getLibrary());
    // Pick up files added since last launch. Unchanged files are skipped by mtime, so this is cheap.
    if (this.folders.length) this.rescan();
  }

  private async scan(job: () => Promise<LibraryPayload>) {
    this.scanning = true;
    try {
      this.apply(await job());
    } catch (e) {
      ui.toast(`Ошибка сканирования: ${e}`);
    } finally {
      this.scanning = false;
    }
  }

  addFolder = (path: string) => this.scan(() => api.addFolder(path));
  removeFolder = (path: string) => this.scan(() => api.removeFolder(path));
  rescan = () => this.scan(() => api.rescan());

  coverUrl(cover: string | null): string | null {
    if (!cover) return null;
    if (demoMode) return cover;
    return fileSrc(`${this.coversDir}/${cover}`);
  }

  artistByName(name: string) {
    return this.artists.find((a) => a.name.toLowerCase() === name.toLowerCase());
  }
}

export const library = new LibraryStore();

// ---------------------------------------------------------------------------

export type Repeat = "off" | "all" | "one";

class PlayerStore {
  /** Track ids in play order (already shuffled if shuffle is on). */
  queue = $state.raw<string[]>(load("queue", []));
  /** Original order — restored when shuffle is turned off. */
  private baseQueue: string[] = load("baseQueue", []);
  index = $state<number>(load("index", -1));
  playing = $state(false);
  position = $state<number>(load("position", 0));
  volume = $state<number>(load("volume", 0.8));
  muted = $state<boolean>(load("muted", false));
  shuffle = $state<boolean>(load("shuffle", false));
  repeat = $state<Repeat>(load("repeat", "off"));
  /** Name of what the queue was started from ("Альбом · X"), shown in the queue panel. */
  source = $state<string>(load("source", ""));

  current = $derived<Track | undefined>(library.byId.get(this.queue[this.index]));
  upNext = $derived(
    this.queue
      .slice(this.index + 1)
      .map((id) => library.byId.get(id))
      .filter((t): t is Track => !!t),
  );

  private loadedId: string | null = null;
  private seekLockUntil = 0;

  async init() {
    await api.onProgress((p) => {
      if (performance.now() > this.seekLockUntil) this.position = p.position;
      this.playing = p.playing;
    });
    await api.onEnded(() => this.handleEnded());
    await api.onError((msg) => {
      ui.toast(`Не удалось воспроизвести: ${msg}`);
      this.playing = false;
    });
    await api.volume(this.muted ? 0 : this.volume);

    // Restore the last session paused at the same spot.
    const t = this.current;
    if (t) {
      this.loadedId = t.id;
      await api.load(t.path, false, this.position);
    }

    setInterval(() => this.persist(), 5000);
    window.addEventListener("beforeunload", () => this.persist());
  }

  private persist() {
    save("queue", this.queue);
    save("baseQueue", this.baseQueue);
    save("index", this.index);
    save("position", this.position);
    save("volume", this.volume);
    save("muted", this.muted);
    save("shuffle", this.shuffle);
    save("repeat", this.repeat);
    save("source", this.source);
  }

  private async start(autoplay = true) {
    const t = this.current;
    if (!t) return;
    this.loadedId = t.id;
    this.position = 0;
    this.playing = autoplay;
    await api.load(t.path, autoplay, 0);
    this.persist();
  }

  /** Plays a list of tracks starting at `startIndex`. */
  playList(tracks: Track[], startIndex = 0, source = "") {
    if (!tracks.length) return;
    const ids = tracks.map((t) => t.id);
    this.baseQueue = ids;
    this.source = source;
    if (this.shuffle) {
      const first = ids[startIndex];
      this.queue = [first, ...shuffled(ids.filter((_, i) => i !== startIndex))];
      this.index = 0;
    } else {
      this.queue = ids;
      this.index = startIndex;
    }
    this.start();
  }

  playShuffled(tracks: Track[], source = "") {
    if (!tracks.length) return;
    this.shuffle = true;
    this.playList(tracks, Math.floor(Math.random() * tracks.length), source);
  }

  isCurrent(id: string) {
    return this.current?.id === id;
  }

  async toggle() {
    if (!this.current) {
      if (library.tracks.length) this.playList(library.tracks, 0, "Все треки");
      return;
    }
    if (this.loadedId !== this.current.id) return this.start();
    this.playing = !this.playing;
    await (this.playing ? api.play() : api.pause());
  }

  next(auto = false) {
    if (!this.queue.length) return;
    if (this.index < this.queue.length - 1) {
      this.index++;
      return this.start();
    }
    if (this.repeat === "all") {
      this.index = 0;
      return this.start();
    }
    if (auto) {
      // Queue finished: stay on the last track; pressing play starts it over.
      this.playing = false;
      this.position = 0;
      this.loadedId = null;
      return;
    }
    // Manual "next" on the last track wraps to the first one, paused.
    this.index = 0;
    this.start(false);
  }

  prev() {
    if (this.position > 3 || this.index <= 0) {
      this.seek(0);
      return;
    }
    this.index--;
    this.start();
  }

  private handleEnded() {
    if (this.repeat === "one") return this.start();
    this.next(true);
  }

  seek(seconds: number) {
    if (!this.current) return;
    const pos = Math.max(0, Math.min(seconds, this.current.duration));
    this.position = pos;
    // Ignore stale progress events until the engine has applied the seek.
    this.seekLockUntil = performance.now() + 400;
    api.seek(pos);
  }

  seekBy(delta: number) {
    this.seek(this.position + delta);
  }

  setVolume(v: number) {
    this.volume = Math.max(0, Math.min(1, v));
    this.muted = false;
    api.volume(this.volume);
  }

  toggleMute() {
    this.muted = !this.muted;
    api.volume(this.muted ? 0 : this.volume);
  }

  toggleShuffle() {
    this.shuffle = !this.shuffle;
    const currentId = this.queue[this.index];
    if (this.shuffle) {
      this.baseQueue = [...this.queue];
      const rest = this.queue.filter((_, i) => i !== this.index);
      this.queue = currentId ? [currentId, ...shuffled(rest)] : shuffled(rest);
      this.index = currentId ? 0 : -1;
    } else if (this.baseQueue.length) {
      // Keep songs added to the queue while shuffled.
      const extra = this.queue.filter((id) => !this.baseQueue.includes(id));
      this.queue = [...this.baseQueue, ...extra];
      this.index = Math.max(0, this.queue.indexOf(currentId));
    }
  }

  cycleRepeat() {
    this.repeat = this.repeat === "off" ? "all" : this.repeat === "all" ? "one" : "off";
  }

  playNext(t: Track) {
    if (!this.current) return this.playList([t]);
    const q = [...this.queue];
    q.splice(this.index + 1, 0, t.id);
    this.queue = q;
    ui.toast(`«${t.title}» будет следующим`);
  }

  addToQueue(t: Track) {
    if (!this.current) return this.playList([t]);
    this.queue = [...this.queue, t.id];
    ui.toast(`«${t.title}» добавлен в очередь`);
  }

  /** Jumps to a position in the "up next" list. */
  jumpTo(upNextIndex: number) {
    this.index = this.index + 1 + upNextIndex;
    this.start();
  }

  removeFromUpNext(upNextIndex: number) {
    const q = [...this.queue];
    q.splice(this.index + 1 + upNextIndex, 1);
    this.queue = q;
  }

  clearUpNext() {
    this.queue = this.queue.slice(0, this.index + 1);
  }
}

function shuffled<T>(arr: T[]): T[] {
  const a = [...arr];
  for (let i = a.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [a[i], a[j]] = [a[j], a[i]];
  }
  return a;
}

export const player = new PlayerStore();

// ---------------------------------------------------------------------------

class LikesStore {
  ids = $state.raw<Set<string>>(new Set(load<string[]>("liked", [])));
  /** Liked tracks, most recently liked first. */
  tracks = $derived(
    [...this.ids]
      .reverse()
      .map((id) => library.byId.get(id))
      .filter((t): t is Track => !!t),
  );

  has(id: string) {
    return this.ids.has(id);
  }

  toggle(id: string) {
    const next = new Set(this.ids);
    if (next.has(id)) next.delete(id);
    else next.add(id);
    this.ids = next;
    save("liked", [...next]);
  }
}

export const likes = new LikesStore();

// ---------------------------------------------------------------------------

export type Route =
  | { name: "home" }
  | { name: "songs" }
  | { name: "albums" }
  | { name: "artists" }
  | { name: "liked" }
  | { name: "search" }
  | { name: "settings" }
  | { name: "album"; id: string }
  | { name: "artist"; name_: string };

export interface MenuItem {
  label: string;
  icon?: string;
  action: () => void;
  danger?: boolean;
}

class UiStore {
  private stack = $state<Route[]>([{ name: "home" }]);
  private pos = $state(0);
  route = $derived(this.stack[this.pos]);
  canBack = $derived(this.pos > 0);
  canForward = $derived(this.pos < this.stack.length - 1);

  search = $state("");
  queueOpen = $state<boolean>(load("queueOpen", false));
  nowPlayingOpen = $state(false);
  toasts = $state<{ id: number; text: string }[]>([]);
  menu = $state<{ x: number; y: number; items: MenuItem[] } | null>(null);

  go(route: Route) {
    const cur = this.route;
    if (JSON.stringify(cur) === JSON.stringify(route)) return;
    this.stack = [...this.stack.slice(0, this.pos + 1), route];
    this.pos = this.stack.length - 1;
    this.nowPlayingOpen = false;
  }
  back() {
    if (this.canBack) this.pos--;
  }
  forward() {
    if (this.canForward) this.pos++;
  }

  toggleQueue() {
    this.queueOpen = !this.queueOpen;
    save("queueOpen", this.queueOpen);
  }

  toast(text: string) {
    const id = Math.random();
    this.toasts = [...this.toasts, { id, text }];
    setTimeout(() => (this.toasts = this.toasts.filter((t) => t.id !== id)), 3200);
  }

  openMenu(e: MouseEvent, items: MenuItem[]) {
    e.preventDefault();
    e.stopPropagation();
    this.menu = { x: e.clientX, y: e.clientY, items };
  }

  trackMenu(e: MouseEvent, t: Track) {
    this.openMenu(e, [
      { label: "Играть следующим", icon: "next", action: () => player.playNext(t) },
      { label: "Добавить в очередь", icon: "queue", action: () => player.addToQueue(t) },
      {
        label: likes.has(t.id) ? "Убрать из любимых" : "Добавить в любимые",
        icon: "heart",
        action: () => likes.toggle(t.id),
      },
      { label: "Перейти к альбому", icon: "album", action: () => this.go({ name: "album", id: t.albumId }) },
      {
        label: "Перейти к исполнителю",
        icon: "artist",
        action: () => this.go({ name: "artist", name_: t.albumArtist }),
      },
    ]);
  }
}

export const ui = new UiStore();
