# Homepage

Personal single-page portfolio for Michał Kruczek built with Leptos (CSR) and Tailwind, bundled via Trunk and targeting WebAssembly.

## Stack

- Rust 2024, Leptos (CSR) with `leptos_meta` for SEO and structured data
- Tailwind CSS (Trunk tailwind pipeline) plus a few custom animations
- Trunk for WASM bundling and static asset pipeline, served from `index.html`

## Project Layout

- Core entry point: [src/main.rs](src/main.rs) → mounts [src/app.rs](src/app.rs)
- UI composition and metadata: [src/app.rs](src/app.rs)
- Reusable components: [src/components](src/components)
  - Hero: [src/components/hero.rs](src/components/hero.rs)
  - Links: [src/components/links.rs](src/components/links.rs)
  - Services: [src/components/services.rs](src/components/services.rs)
  - Contact form (SubmitForm backend): [src/components/contact.rs](src/components/contact.rs)
  - Glass effects: [src/components/common.rs](src/components/common.rs)
  - Animated background: [src/components/background.rs](src/components/background.rs)
- Styles: Tailwind entry [src/styles/tailwind.css](src/styles/tailwind.css), custom animations [src/styles/style.css](src/styles/style.css)
- Static assets copied via Trunk: [public](public)

## Prerequisites

- Rust toolchain (see [rust-toolchain.toml](rust-toolchain.toml))
- WebAssembly target: `rustup target add wasm32-unknown-unknown`
- Trunk and tooling: `cargo binstall -y trunk` (or `make install-tools` for full toolchain)

## Quickstart

1) Install deps and tools: `make setup`
2) Run dev server with live reload: `make run` (opens on a local port)
3) Build optimized static bundle: `make build-release` (outputs to `dist/`)

### Direct Trunk commands

- Dev: `trunk serve --open`
- Release build: `trunk build --release`

## Testing and Quality

- Tests with coverage: `make test` (uses `cargo llvm-cov` and `cargo nextest`)
- Lint and format: `make lint` and `make format`
- Security and license audit: `make audit` (cargo-deny)

## Deployment

- `dist/` contains the static bundle; serve with any static host (e.g., Nginx, GitHub Pages, Netlify).
- Ensure public assets (icons, resume, sitemap, robots) remain under [public](public) so Trunk copies them.

## Customization

- Update SEO/meta and JSON-LD in [src/app.rs](src/app.rs).
- Adjust palette/background animations in [src/styles/style.css](src/styles/style.css) and Tailwind utilities in [src/styles/tailwind.css](src/styles/tailwind.css).
- Replace assets (profile image, resume) under [public/assets](public/assets).

## License

MIT © Michał Kruczek
