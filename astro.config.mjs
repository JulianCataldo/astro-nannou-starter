import { defineConfig } from "astro/config";
import { ViteRsw } from "vite-plugin-rsw";

// https://astro.build/config
export default defineConfig({
  vite: {
    plugins: [ViteRsw()],
  },
});
