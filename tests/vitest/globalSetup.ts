import { execFileSync } from 'node:child_process'
import path from 'node:path'
import type { TestProject } from 'vitest/node'
import { HARNESS_DIR } from './src/sdk'

// Builds the harness crate, which depends on every generated crate: a crate
// that does not compile fails the whole suite here.
export async function setup(project: TestProject): Promise<void> {
  const cargo = process.env.CARGO ?? 'cargo'
  execFileSync(cargo, ['build', '--locked', '--bins', '--manifest-path', path.join(HARNESS_DIR, 'Cargo.toml')], {
    stdio: 'inherit',
  })
  const targetDir = process.env.CARGO_TARGET_DIR ?? path.join(HARNESS_DIR, 'target')
  project.provide('harness', path.join(targetDir, 'debug', 'sdk-harness'))
}

declare module 'vitest' {
  export interface ProvidedContext {
    harness: string
  }
}
