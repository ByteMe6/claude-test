import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export interface Track {
  id: string;
  path: string;
  title: string;
  artist: string;
  album: string;
  albumArtist: string;
  albumId: string;
  trackNo: number | null;
  discNo: number | null;
  year: number | null;
  genre: string | null;
  duration: number;
  cover: string | null;
  color: string | null;
  mtime: number;
  added: number;
}

export interface LibraryPayload {
  folders: string[];
  tracks: Track[];
  coversDir: string;
}

export interface Progress {
  position: number;
  playing: boolean;
}

export const api = {
  getLibrary: () => invoke<LibraryPayload>("get_library"),
  addFolder: (path: string) => invoke<LibraryPayload>("add_folder", { path }),
  removeFolder: (path: string) => invoke<LibraryPayload>("remove_folder", { path }),
  rescan: () => invoke<LibraryPayload>("rescan"),

  load: (path: string, autoplay: boolean, start = 0) =>
    invoke<void>("player_load", { path, autoplay, start }),
  play: () => invoke<void>("player_play"),
  pause: () => invoke<void>("player_pause"),
  seek: (position: number) => invoke<void>("player_seek", { position }),
  volume: (volume: number) => invoke<void>("player_volume", { volume }),
  stop: () => invoke<void>("player_stop"),

  onProgress: (cb: (p: Progress) => void): Promise<UnlistenFn> =>
    listen<Progress>("player:progress", (e) => cb(e.payload)),
  onEnded: (cb: () => void): Promise<UnlistenFn> => listen("player:ended", () => cb()),
  onError: (cb: (msg: string) => void): Promise<UnlistenFn> =>
    listen<string>("player:error", (e) => cb(e.payload)),
};

export const fileSrc = (path: string) => convertFileSrc(path);
