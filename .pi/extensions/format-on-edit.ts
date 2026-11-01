/**
 * format-on-edit — project pi extension hook.
 *
 * Enforces at edit/commit time what CI's `npm run format:check` checks after push,
 * so pi sessions cannot produce unformatted commits:
 *
 *   (A) tool_result on Edit|Write  -> run `prettier --write` on the edited file
 *   (B) tool_call on Bash `git commit` -> run `prettier --write` on staged files and re-add them
 *
 * Uses the repo-local prettier binary (./node_modules/.bin/prettier, present after
 * `npm ci`, pinned by package-lock) — never touches the network, mirrors CI exactly.
 * Failures are logged, never thrown, so the hook can't break an edit or a commit.
 */
import type { ExtensionAPI } from '@earendil-works/pi-coding-agent';

const PRETTIER_BIN = './node_modules/.bin/prettier';
const PRETTIER_SCOPE = /\.(?:svelte|ts|js|mjs|json|css|md|ya?ml)$/;
const GIT_COMMIT_RE = /(?:^|[;&]\s*)git\s+commit\b/;

function toolNameOf(event: unknown): string | undefined {
  const name = (event as { toolName?: unknown } | null | undefined)?.toolName;
  return typeof name === 'string' ? name.toLowerCase() : undefined;
}

function isTool(event: unknown, ...names: string[]): boolean {
  const name = toolNameOf(event);
  return name !== undefined && names.includes(name);
}

function inputOf(event: unknown): Record<string, unknown> | undefined {
  const input = (event as { input?: unknown } | null | undefined)?.input;
  return input && typeof input === 'object' ? (input as Record<string, unknown>) : undefined;
}

function filePathOf(input: Record<string, unknown> | undefined): string | undefined {
  if (!input) return undefined;
  for (const key of ['path', 'filePath', 'file']) {
    const value = input[key];
    if (typeof value === 'string' && value.trim()) return value;
  }
  return undefined;
}

function commandOf(input: Record<string, unknown> | undefined): string {
  const command = input?.command;
  return typeof command === 'string' ? command : '';
}

async function formatWithPrettier(pi: ExtensionAPI, files: string[]): Promise<void> {
  if (files.length === 0) return;
  const result = await pi.exec(PRETTIER_BIN, ['--write', ...files]);
  if (result.code !== 0) {
    console.warn('[format-on-edit] prettier exited', result.code, result.stderr ?? '');
  }
}

export default function (pi: ExtensionAPI): void {
  console.log('[format-on-edit] active');

  // (A) Auto-format right after every file edit/write.
  pi.on('tool_result', async (event: unknown) => {
    if (!isTool(event, 'edit', 'write')) return;
    const file = filePathOf(inputOf(event));
    if (!file || !PRETTIER_SCOPE.test(file)) return;
    try {
      await formatWithPrettier(pi, [file]);
    } catch (error) {
      console.warn('[format-on-edit]', error);
    }
  });

  // (B) Format gate right before `git commit`: format staged files, then re-stage them.
  pi.on('tool_call', async (event: unknown) => {
    if (!isTool(event, 'bash')) return;
    const command = commandOf(inputOf(event));
    if (!GIT_COMMIT_RE.test(command)) return;
    try {
      const { stdout, code } = await pi.exec('git', [
        'diff',
        '--cached',
        '--name-only',
        '--diff-filter=ACM'
      ]);
      if (code !== 0) return;
      const files = stdout
        .split('\n')
        .map((line) => line.trim())
        .filter(Boolean)
        .filter((file) => PRETTIER_SCOPE.test(file));
      if (files.length === 0) return;
      await formatWithPrettier(pi, files);
      await pi.exec('git', ['add', ...files]);
    } catch (error) {
      console.warn('[format-on-edit]', error);
    }
  });
}
