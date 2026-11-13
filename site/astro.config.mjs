import { defineConfig } from "astro/config";

export default defineConfig({
  site: "https://pamm.perondas.dev",
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
