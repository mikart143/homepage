#!/usr/bin/env node
/**
 * Generates static/og-image.png — the 1200×630 Open Graph social card for
 * kruczek.dev (1.91:1, the aspect ratio social platforms crop to).
 *
 * The actual rendering is done by python3 + Pillow (PIL) because Pillow is
 * installed on the machine and ImageMagick is not; Node only orchestrates it.
 *
 * Reproducible: run `node scripts/generate-og.mjs` from the repo root (or
 * anywhere — paths are resolved against the project root, derived from this
 * file's location).
 *
 * Prerequisite: python3 with Pillow (`pip3 install Pillow`, >= 10.1 for the
 * load_default(size=...) fallback).
 */
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const projectRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const profilePath = path.join(projectRoot, 'static', 'assets', 'profile.webp');
const outPath = path.join(projectRoot, 'static', 'og-image.png');

if (!fs.existsSync(profilePath)) {
  console.error(`profile photo not found: ${profilePath}`);
  process.exit(1);
}

const PYTHON = `import os
import sys

try:
    from PIL import Image, ImageDraw, ImageFont
except ImportError:
    sys.stderr.write("error: Pillow is required (pip3 install Pillow)\\n")
    sys.exit(1)

W, H = 1200, 630
BASE = (15, 23, 42)  # #0f172a slate
STOPS = [(59, 130, 246), (139, 92, 246), (236, 72, 153)]  # blue, purple, pink
BLEND = 0.16  # how strongly the gradient shows over the slate base
TEXT_X = 440
TEXT_MAX_W = W - TEXT_X - 60
RING_ALPHA = 90


def lerp(a, b, t):
    return tuple(int(round(a[i] + (b[i] - a[i]) * t)) for i in range(3))


def row_color(y):
    t = y / (H - 1)
    if t < 0.5:
        return lerp(STOPS[0], STOPS[1], t * 2)
    return lerp(STOPS[1], STOPS[2], (t - 0.5) * 2)


def load_font(size, bold=False):
    candidates = [
        ('/System/Library/Fonts/Helvetica.ttc', 1 if bold else 0),
        ('/System/Library/Fonts/Supplemental/Arial Bold.ttf', 0) if bold else None,
        ('/System/Library/Fonts/Supplemental/Arial.ttf', 0) if not bold else None,
        ('/System/Library/Fonts/SFNS.ttf', 0),
    ]
    for candidate in candidates:
        if candidate is None:
            continue
        font_path, index = candidate
        try:
            return ImageFont.truetype(font_path, size, index=index)
        except OSError:
            continue
    try:
        return ImageFont.load_default(size=size)  # Pillow >= 10.1
    except TypeError:
        return ImageFont.load_default()


def fit_font(text, size, max_width, bold=False):
    font = load_font(size, bold=bold)
    while size > 18 and font.getbbox(text)[2] > max_width:
        size -= 1
        font = load_font(size, bold=bold)
    return font


def main():
    profile_path, out_path = sys.argv[1], sys.argv[2]

    # 1) Background: slate with a subtle vertical blue -> purple -> pink wash.
    base = Image.new('RGB', (W, H), BASE)
    grad = Image.new('RGB', (1, H))
    grad.putdata([row_color(y) for y in range(H)])
    canvas = Image.blend(grad.resize((W, H)), base, 1 - BLEND).convert('RGBA')

    # 2) Circular profile photo (left, vertically centered).
    size = 340
    photo = Image.open(profile_path).convert('RGB').resize((size, size), Image.LANCZOS)
    mask = Image.new('L', (size, size), 0)
    ImageDraw.Draw(mask).ellipse((0, 0, size - 1, size - 1), fill=255)
    head = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    head.paste(photo, (0, 0), mask)
    left, top = 80, H // 2 - size // 2
    canvas.alpha_composite(head, (left, top))

    # 3) Antialiased white ring around the photo (4x supersampled).
    ring = Image.new('L', (W * 4, H * 4), 0)
    d4 = ImageDraw.Draw(ring)
    cx, cy = left + size // 2, top + size // 2
    r1, r2 = size // 2, size // 2 - 6
    d4.ellipse(((cx - r1) * 4, (cy - r1) * 4, (cx + r1) * 4, (cy + r1) * 4), fill=255)
    d4.ellipse(((cx - r2) * 4, (cy - r2) * 4, (cx + r2) * 4, (cy + r2) * 4), fill=0)
    ring = ring.resize((W, H), Image.LANCZOS)
    ring_layer = Image.new('RGBA', (W, H), (255, 255, 255, 0))
    ring_layer.putalpha(ring.point(lambda v: v * RING_ALPHA // 255))
    canvas.alpha_composite(ring_layer)

    # 4) White text on the right.
    draw = ImageDraw.Draw(canvas)
    white = (255, 255, 255, 255)
    draw.text(
        (TEXT_X, 248),
        'Michał Kruczek',
        font=fit_font('Michał Kruczek', 64, TEXT_MAX_W, bold=True),
        fill=white,
        anchor='lm',
    )
    draw.text(
        (TEXT_X, 328),
        'Full-Stack Developer & AI Specialist',
        font=fit_font('Full-Stack Developer & AI Specialist', 36, TEXT_MAX_W),
        fill=white,
        anchor='lm',
    )
    draw.text(
        (TEXT_X, 545),
        'kruczek.dev',
        font=fit_font('kruczek.dev', 28, TEXT_MAX_W),
        fill=white,
        anchor='lm',
    )

    canvas.convert('RGB').save(out_path, 'PNG', optimize=True)
    kb = round(os.path.getsize(out_path) / 1024, 1)
    print('wrote %s  %dx%d  %.1f KB' % (out_path, W, H, kb))


if __name__ == '__main__':
    main()
`;

const run = spawnSync('python3', ['-', profilePath, outPath], {
  input: PYTHON,
  encoding: 'utf8'
});

if (run.error) {
  console.error(`failed to run python3: ${run.error.message}`);
  process.exit(1);
}
if (run.stdout) process.stdout.write(run.stdout);
if (run.stderr) process.stderr.write(run.stderr);
if (run.status !== 0) process.exit(run.status ?? 1);

if (!fs.existsSync(outPath)) {
  console.error(`python3 exited 0 but produced no ${outPath}`);
  process.exit(1);
}
console.log(`ok: ${outPath} (${(fs.statSync(outPath).size / 1024).toFixed(1)} KB)`);
