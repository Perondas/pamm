import { defineConfig } from "astro/config";

// TODO: set `site` to the URL the page will be deployed to.
export default defineConfig({
  site: "https://example.com/",
  vite: {
    css: {
      preprocessorOptions: {
        scss: {
          // Pico's SCSS triggers Sass deprecation warnings we can't fix here.
          quietDeps: true,
          silenceDeprecations: ["if-function"],
        },
      },
    },
  },
});
