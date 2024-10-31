import { writable } from "svelte/store";

export const currentContextStore = writable<string>("");
