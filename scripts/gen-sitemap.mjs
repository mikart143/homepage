import { fileURLToPath } from 'node:url';
import { writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

const staticDir = join(dirname(fileURLToPath(import.meta.url)), '..', 'static');
const lastmod = new Date().toISOString().slice(0, 10);

const sitemap = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://kruczek.dev/</loc>
    <lastmod>${lastmod}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
</urlset>
`;

writeFileSync(join(staticDir, 'sitemap.xml'), sitemap);
console.log(`Generated static/sitemap.xml (lastmod: ${lastmod})`);
