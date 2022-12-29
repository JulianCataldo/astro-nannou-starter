import { defineConfig } from 'astro/config';
import { ViteRsw } from 'vite-plugin-rsw';

// https://astro.build/config
export default defineConfig({
  site: 'https://juliancataldo.github.io',

  base: '/astro-nannou-starter',

  vite: {
    plugins: [ViteRsw()],
  },
});
