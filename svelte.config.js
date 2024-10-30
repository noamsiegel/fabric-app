// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      fallback: 'index.html'
    }),
    // prerender: {
    //   default: true,
    //   handleMissingId: 'ignore'  // Add this line
    // },
    // Add this to ensure proper static handling
    env: {
      dir: '.'
    }
    // Add these configurations for Tauri
    // prerender: {
    //   default: true
    // },
    // Prevent SvelteKit from handling static assets that Tauri will handle
    // inlineStyleThreshold: Infinity
  },
};

export default config;
