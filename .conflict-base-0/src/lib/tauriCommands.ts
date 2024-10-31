import { invoke } from "@tauri-apps/api/core";

export async function clipboard_contents_and_run_pattern(): Promise<string> {
  return await invoke<string>("clipboard_contents_and_run_pattern");
}
