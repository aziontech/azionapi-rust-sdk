import { existsSync } from 'node:fs'
import path from 'node:path'
import { describe, expect, it } from 'vitest'
import { cargo, documentedEndpoints, harnessDependencies, listCrates, moduleFile, publicFunctions } from '../src/sdk'

const crates = listCrates()

// `storage/Cargo.toml` declares `version = "1.0.0 (1)"`, which is not valid
// SemVer, so Cargo refuses to load the crate. The defect is in the generated
// manifest and must be fixed in the generator input. When a regeneration
// fixes it, the guard below fails: add the crate to the harness and remove
// it from this list.
const KNOWN_BROKEN = new Set(['storage'])

describe('generated Rust SDK crates', () => {
  it('discovers every generated crate', () => {
    expect(crates.length).toBeGreaterThanOrEqual(18)
  })

  it('compiles every loadable crate through the harness', () => {
    // globalSetup builds the harness, which depends on each of these crates.
    const expected = crates.map((c) => c.name).filter((n) => !KNOWN_BROKEN.has(n))
    expect(harnessDependencies().sort()).toEqual(expected)
  })

  for (const crate of crates) {
    describe(crate.name, () => {
      it('declares every endpoint documented in its README as a public async function', () => {
        const documented = documentedEndpoints(crate)
        expect(documented.length).toBeGreaterThan(0)
        for (const endpoint of documented) {
          const file = moduleFile(crate, endpoint.api)
          expect(existsSync(file), path.relative(crate.dir, file)).toBe(true)
          expect(publicFunctions(file), `${endpoint.api}::${endpoint.fn}`).toContain(endpoint.fn)
        }
      })
    })
  }

  for (const name of KNOWN_BROKEN) {
    it(`${name}: Cargo still rejects the generated manifest`, async () => {
      const result = await cargo(['metadata', '--no-deps', '--format-version', '1', '--manifest-path', `${name}/Cargo.toml`])
      expect(result.ok).toBe(false)
      expect(result.output).toMatch(/unexpected character|invalid|version/i)
    })
  }
})
