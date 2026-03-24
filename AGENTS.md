# Agent Instructions

This repository uses `mise.toml` to manage toolchain versions (e.g., `rust` and `cargo:trunk`).
It also includes GitHub Actions workflows `.github/workflows/deploy.yml` and `.github/workflows/pr.yml` that build and test the game.

**Important Rule for Version Bumping:**

Local environments (managed via `mise.toml`) and CI/CD pipelines (managed via GitHub workflows) MUST always be kept in sync. This synchronization works in both directions:

1. **Manual Updates:** Whenever you manually update the version of `rust` or `cargo:trunk` in `mise.toml`, you must also update the corresponding version strings in the GitHub workflows.
2. **Automated Updates (e.g., Dependabot):** If an automated bot updates the version strings in `.github/workflows/deploy.yml` or `.github/workflows/pr.yml`, you must update `mise.toml` to match.

**Mapping between files:**
- In `.github/workflows/deploy.yml` and `.github/workflows/pr.yml`:
  - `dtolnay/rust-toolchain@master` `toolchain` -> `rust` in `mise.toml`.
  - `jetli/trunk-action` `version` -> `cargo:trunk` in `mise.toml`.
