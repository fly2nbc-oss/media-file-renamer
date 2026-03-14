# Versioning Policy

This project uses [Semantic Versioning (SemVer)](https://semver.org/): `MAJOR.MINOR.PATCH`.

## Rules

- `MAJOR`: breaking changes (incompatible behavior, migration required).
- `MINOR`: new backward-compatible features.
- `PATCH`: backward-compatible bug fixes, hardening, and small improvements.

## Practical Mapping

- Feature PR merged to `main` -> bump `MINOR` and reset `PATCH` to `0`.
- Bugfix/chore/security/perf PR merged to `main` -> bump `PATCH`.
- Breaking feature PR merged to `main` -> bump `MAJOR`, reset `MINOR` and `PATCH` to `0`.

## Build vs Release

- Local developer builds should not automatically change project version numbers.
- Version bumps should happen on release-relevant merges (or release automation), not on every local build.
- CI build/run numbers can be tracked separately and must not replace SemVer.

## Source of Truth and Sync Targets

To avoid version drift, keep version values aligned in:

- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`
- `package-lock.json` (when updated by npm tooling)

## Automation

Use the npm scripts below to keep all sync targets aligned automatically:

- `npm run version:major`
- `npm run version:minor`
- `npm run version:patch`
- `npm run version:auto` (derives bump from commit history since last tag)

Preview without writing changes:

- `node scripts/version-bump.mjs auto --dry-run`

`version:auto` follows Conventional Commit style:

- breaking markers (`!` or `BREAKING CHANGE:`) -> `MAJOR`
- `feat:` -> `MINOR`
- otherwise -> `PATCH`

## Release Hygiene

- Create a release tag for each published version (for example: `v1.3.0`).
- Keep a human-readable changelog (`CHANGELOG.md`) and group entries by:
  - Added (`MINOR`)
  - Fixed (`PATCH`)
  - Changed/Removed with breaking notes (`MAJOR`)

## Optional Commit Convention

Using Conventional Commits can simplify automated bumps:

- `feat:` -> `MINOR`
- `fix:`, `perf:`, `refactor:`, `chore:` -> `PATCH`
- `!` or `BREAKING CHANGE:` -> `MAJOR`
