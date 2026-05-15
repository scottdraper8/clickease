# Clickease CI/CD Strategy

This document outlines the Continuous Integration and Continuous Deployment strategy for Clickease, ensuring high-quality builds across Windows, macOS, and Linux.

## 1. Security Mandate: Step Security Harden-Runner

...

## 2. Fork Security (Anti-Worm/Shai Hulud)

To prevent "worm" style attacks where malicious forks trigger actions to exfiltrate secrets or poison the cache (e.g., the Shai Hulud incident):

- **Read-Only Defaults:** All workflows MUST explicitly set `permissions: read-all` at the top level. Permissions should only be escalated on a per-job basis where strictly necessary.
- **`pull_request_target` Prohibition:** Never use `pull_request_target` for tasks that check out untrusted code or run build scripts (e.g., `pnpm install`, `cargo build`). This event runs with write-access and access to secrets; use `pull_request` instead for all CI tasks.
- **Approval Policy:** The repository MUST be configured to require manual approval for all outside contributors before any GitHub Actions workflows are triggered.
- **Cache Isolation:** CI caches should be keyed with `github.event.pull_request.head.sha` to prevent cross-PR cache poisoning.

## 3. Dependency Management & Cooldown

...

To ensure the integrity of our build pipeline and prevent supply chain attacks, **all GitHub Action workflows MUST include Step Security's `harden-runner` as the first step in every job.**

### Why Harden-Runner?

- **Network Egress Filtering:** Restricts outbound connections to only those necessary for the build (e.g., pnpm registries, crates.io, Apple/Windows signing servers).
- **File Integrity Monitoring:** Detects and alerts on unauthorized file modifications during the build process.

### Implementation Snippet

```yaml
jobs:
  build:
    runs-on: ubuntu-latest # or windows/macos
    steps:
      - name: Harden Runner
        uses: step-security/harden-runner@v2
        with:
          egress-policy: audit # Start with audit, then transition to block
```

## 2. CI Pipeline (GitHub Actions)

... (rest of the content)

The CI pipeline will be triggered on every Pull Request and push to the `main` branch.

### 1.1 Web/Frontend Validation

- **Linting:** Run `pnpm lint` (ESLint + Prettier).
- **Type Checking:** Run `pnpm type-check` (tsc).
- **Testing:** Run frontend unit/component tests (Vitest).

### 1.2 Rust/Backend Validation

- **Formatting:** Run `cargo fmt --check`.
- **Linting:** Run `cargo clippy -- -D warnings`.
- **Testing:** Run `cargo test` (unit and integration tests).

### 1.3 Cross-Platform Build Check

- Compile the application for Windows, macOS, and Linux on every PR to ensure no platform-specific regressions are introduced.

## 2. CD Pipeline (Release)

The CD pipeline will be triggered by creating a new Git tag (e.g., `v1.0.0`).

### 2.1 Multi-Platform Artifact Generation

- **macOS:** Build and notarize `.dmg` and `.app` bundles using Apple's Notary Service.
- **Windows:** Build and sign `.msi` and `.exe` installers using Trusted Signing.
- **Linux:** Build `.AppImage` and `.deb` packages.

### 2.2 Automated Releases

- Automatically create a GitHub Release with the generated artifacts.
- Generate a changelog based on conventional commit messages.

## 3. Tooling Requirements

- **Package Manager:** `pnpm`
- **Rust Toolchain:** `stable` (standardized across environments)
- **Tauri CLI:** Used for bundling and cross-compilation orchestration.
- **Pre-commit:** Ensures local code meets CI standards before pushing.
