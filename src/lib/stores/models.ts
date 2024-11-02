import { writable } from "svelte/store";

export const defaultModelStore = writable<string>("");
export const defaultVendorStore = writable<string>("");
