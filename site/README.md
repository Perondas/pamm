# pamm site

Landing page for pamm, built with [Astro](https://astro.build/) from the
[astro-smallworld](https://github.com/anaxite/astro-smallworld) template and styled with
[Pico CSS](https://picocss.com/).

Requires Node 22 or newer (see `mise.toml`).

```sh
pnpm install
pnpm dev      # local dev server
pnpm build    # type-check and build to dist/
```

- Page content: `src/pages/index.astro`
- Site title, description, repo link: `src/settings.ts`
- Deploy URL: `site` in `astro.config.mjs`
