import { defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    globals: true,
    globalSetup: './globalSetup.ts',
    include: ['functional/**/*.test.ts'],
    testTimeout: 60_000,
    // The harness build compiles every generated crate.
    hookTimeout: 1_800_000,
  },
})
