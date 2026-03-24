# Agent Instructions

This repository uses `mise.toml` to manage toolchain versions (e.g., `rust` and `cargo:trunk`).
It also includes a GitHub Actions workflow `.github/workflows/deploy.yml` that builds and deploys the game to GitHub Pages.

**Important Rule for Version Bumping:**

Whenever you update the version of `rust` or `cargo:trunk` in `mise.toml`, you MUST also update the corresponding version strings in `.github/workflows/deploy.yml` to keep local environments and CI/CD pipelines aligned.

- In `.github/workflows/deploy.yml`:
  - Look for `dtolnay/rust-toolchain@master` and update `toolchain` to match the Rust version in `mise.toml`.
  - Look for `jetli/trunk-action` and update `version` to match the Trunk version (e.g., `v0.21.14`) in `mise.toml`.
