# Homepage

Personal single-page portfolio for Michał Kruczek ([kruczek.dev](https://kruczek.dev/)), built with SvelteKit (Svelte 5, TypeScript) and Tailwind CSS v4. It is prerendered to static HTML with `@sveltejs/adapter-static` and ships no client-side JavaScript (`csr = false`); the contact form is a plain HTML `POST`.

## Stack

- SvelteKit 3 + Svelte 5, TypeScript (strict)
- `@sveltejs/adapter-static`, single prerendered page
- Tailwind CSS v4 via `@tailwindcss/vite`, plus custom background animations
- ESLint, Prettier, `svelte-check`
- Hosted on Cloudflare Pages (project `homepage`)

## Project Layout

- Layout (global CSS, skip link, `<main>`): [src/routes/+layout.svelte](src/routes/+layout.svelte)
- Page options (`prerender`, `csr`): [src/routes/+layout.ts](src/routes/+layout.ts)
- Page, SEO meta and JSON-LD: [src/routes/+page.svelte](src/routes/+page.svelte)
- Components: [src/lib/components](src/lib/components) (Hero, ImportantLinks, Services, Contact, GlassSection, PlasmaBackground)
- Styles: [src/app.css](src/app.css) (Tailwind import, background animations, reduced-motion rule)
- Static files (assets, icons, `robots.txt`, `sitemap.xml`): [static](static)
- Build config (adapter, Tailwind plugin): [vite.config.ts](vite.config.ts) (SvelteKit 3 no longer reads `svelte.config.js`)

## Prerequisites

- Node.js, see [.nvmrc](.nvmrc) (SvelteKit 3 needs >= 22.17)

## Quickstart

1. Install dependencies: `npm ci`
2. Dev server: `npm run dev`
3. Production build to `build/`: `npm run build`
4. Preview the build: `npm run preview`

## Quality

- Type check: `npm run check`
- Lint: `npm run lint`
- Format: `npm run format` (check only: `npm run format:check`)
- Dependency audit: `npm audit --omit=dev`

## Deployment

Pushing to `main` runs [deploy.yaml](.github/workflows/deploy.yaml): `npm ci`, `npm run build`, then `wrangler pages deploy build --project-name=homepage`. It needs the `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID` repository secrets. Run it manually with the `dry_run` input to pass `--dry-run`.

There is no versioning or automated release; [CHANGELOG.md](CHANGELOG.md) is kept as history of the Rust/Leptos era only.

## Customization

- Update SEO/meta and JSON-LD in [src/routes/+page.svelte](src/routes/+page.svelte).
- Edit link and service content in the data arrays at the top of [ImportantLinks.svelte](src/lib/components/ImportantLinks.svelte) and [Services.svelte](src/lib/components/Services.svelte).
- Replace assets (profile image, resume) under [static/assets](static/assets).

## License

MIT © Michał Kruczek
