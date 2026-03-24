# Agent Instructions

This repository uses `mise.toml` to manage toolchain versions (e.g., `rust` and `cargo:trunk`).
It also includes GitHub Actions workflows `.github/workflows/deploy.yml` and `.github/workflows/pr.yml` that build and test the game.

**Important Rule for Version Bumping:**

Whenever you update the version of `rust` or `cargo:trunk` in `mise.toml`, you MUST also update the corresponding version strings in the GitHub workflows to keep local environments and CI/CD pipelines aligned.

- In `.github/workflows/deploy.yml` and `.github/workflows/pr.yml`:
  - Look for `dtolnay/rust-toolchain@master` and update `toolchain` to match the Rust version in `mise.toml`.
  - Look for `jetli/trunk-action` and update `version` to match the Trunk version (e.g., `v0.21.14`) in `mise.toml`.
