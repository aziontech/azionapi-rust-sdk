import { execFile } from 'node:child_process'
import { existsSync, readdirSync, readFileSync } from 'node:fs'
import path from 'node:path'
import { promisify } from 'node:util'

const run = promisify(execFile)

export const REPO_ROOT = path.resolve(__dirname, '../../..')
export const HARNESS_DIR = path.join(REPO_ROOT, 'tests/vitest/harness')

export interface Crate {
  name: string
  dir: string
}

/** Every generated crate: a top-level directory with a Cargo.toml. */
export function listCrates(): Crate[] {
  return readdirSync(REPO_ROOT, { withFileTypes: true })
    .filter((d) => d.isDirectory() && !d.name.startsWith('.') && d.name !== 'tests')
    .map((d) => ({ name: d.name, dir: path.join(REPO_ROOT, d.name) }))
    .filter((c) => existsSync(path.join(c.dir, 'Cargo.toml')))
    .sort((a, b) => a.name.localeCompare(b.name))
}

/** Endpoints documented in the crate README ("Documentation for API Endpoints" table). */
export function documentedEndpoints(crate: Crate): { api: string; fn: string }[] {
  const readme = readFileSync(path.join(crate.dir, 'README.md'), 'utf8')
  const row = /^\*(\w+)\* \| \[\*\*(\w+)\*\*\]\([^)]*\) \| \*\*\w+\*\* /gm
  return [...readme.matchAll(row)].map((m) => ({ api: m[1], fn: m[2] }))
}

/** `PersonalTokenApi` -> `personal_token_api` (generator module naming). */
export function moduleFile(crate: Crate, api: string): string {
  const snake = api.replace(/([a-z0-9])([A-Z])/g, '$1_$2').toLowerCase()
  return path.join(crate.dir, 'src', 'apis', `${snake}.rs`)
}

/** Public async functions declared in a generated API module. */
export function publicFunctions(file: string): string[] {
  const source = readFileSync(file, 'utf8')
  return [...source.matchAll(/^pub async fn (\w+)\(/gm)].map((m) => m[1])
}

/** Path dependencies declared by the harness crate. */
export function harnessDependencies(): string[] {
  const manifest = readFileSync(path.join(HARNESS_DIR, 'Cargo.toml'), 'utf8')
  return [...manifest.matchAll(/^(\w+) = \{ path = "\.\.\/\.\.\/\.\.\/(\w+)" \}/gm)].map((m) => m[2])
}

export async function harness(bin: string, args: string[]): Promise<any> {
  const { stdout } = await run(bin, args, { cwd: REPO_ROOT })
  return JSON.parse(stdout)
}

export async function cargo(args: string[]): Promise<{ ok: boolean; output: string }> {
  try {
    const { stdout, stderr } = await run(process.env.CARGO ?? 'cargo', args, { cwd: REPO_ROOT })
    return { ok: true, output: stdout + stderr }
  } catch (error: any) {
    return { ok: false, output: `${error.stdout ?? ''}${error.stderr ?? ''}` }
  }
}
