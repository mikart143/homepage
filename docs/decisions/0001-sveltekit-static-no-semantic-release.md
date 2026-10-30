# 0001: SvelteKit static site, no semantic-release

Date: 2026-10-30. Status: accepted.

## Context

The homepage was a Leptos CSR app built with Trunk. The served `index.html` was an empty shell, so search engines saw no content.

## Decision

- Rebuild as a single prerendered SvelteKit (Svelte 5, TypeScript strict) page using `@sveltejs/adapter-static`, with `csr = false` so no client JS is shipped. Tailwind v4 via `@tailwindcss/vite`. Hosting stays on Cloudflare Pages (project `homepage`).
- Drop semantic-release, `CHANGELOG.md` generation and version bumping. Deploy on every push to `main` after the checks pass.

## Rejected options

- **Leptos SSR**: needs a server runtime or hydration for a page with no interactivity; Cloudflare Pages static hosting would no longer fit.
- **Astro**: also fits static output, but SvelteKit was chosen as the preferred stack.

## Consequences

- `wrangler pages deploy` has no `--dry-run` flag, so the workflow's `dry_run` input builds and skips the deploy.
- `CHANGELOG.md` is kept as history of the Rust/Leptos era only.
- SvelteKit 3 reads config from `vite.config.ts` and uses `#lib` instead of `$lib`.
