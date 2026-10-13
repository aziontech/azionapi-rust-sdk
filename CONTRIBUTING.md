# Contributing to azionapi-rust-sdk

## Repository layout

```
<crate>/                   # one generated crate per Azion API product
  src/apis/                #   async API functions (generated)
  src/models/              #   request and response models (generated)
  docs/                    #   API reference (generated)
  Cargo.toml
tests/vitest/              # functional tests for all crates
  harness/                 #   harness crate depending on every crate
scripts/clippy.sh          # clippy baseline for the generated code
.github/workflows/         # CI: compliance, security, lint, tests, version tag
```

## Generated code

The crate directories are produced by OpenAPI Generator and arrive through
automated `generated-sdk` pull requests. Do not edit them by hand: a manual
change is lost on the next regeneration. Fix the OpenAPI document or the
generator settings instead. Hand-written changes are welcome in the
repository-level files (tests, scripts, workflows and documentation).

When a regeneration adds or removes a crate, update the path dependencies in
`tests/vitest/harness/Cargo.toml` and refresh its `Cargo.lock`; the tests
check that the harness depends on every crate.

## Local checks

```sh
# Functional tests (Node.js 22 and Rust 1.90)
cd tests/vitest && npm ci && npm test

# Lint
./scripts/clippy.sh
find . -name '*.sh' -not -path './.git/*' -not -path '*/node_modules/*' -print0 | xargs -0 shellcheck --severity=error
```

## Pull requests

1. Create a branch from `main`.
2. Keep the change focused and make sure the checks above pass.
3. Open a pull request to `main`; CODEOWNERS review is required.
4. Every merge to `main` creates the next SemVer tag automatically.

## Commit and PR titles

We follow [Conventional Commits](https://www.conventionalcommits.org/). PR
titles are checked in CI and may carry a Jira key prefix:

```
<type>[(scope)]: <description>
[ENG-123] fix(tests): cover the waf crate
[NO-ISSUE] docs: clarify installation
```

Types: `feat`, `fix`, `docs`, `chore`, `ci`, `test`, `refactor`, `perf`, `build`, `revert`.

The version tag created on merge is a minor bump by default. Add `#major`,
`#patch` or `#none` to a commit message to change it.

## Security

Do not open public issues for vulnerabilities. Follow [SECURITY.md](SECURITY.md).
