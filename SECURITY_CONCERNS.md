# ClickEase Security Concerns

**Date:** July 3, 2026
**Status:** Temporary working document
**Scope:** CI/CD, dependency management, Tauri configuration, release pipeline

## Context

These findings were identified during a cross-project security review. The
existing `docs/discovery/SECURITY_AUDIT.md` covers architecture and OS-level
concerns (SEC-01 through SEC-07). This document covers infrastructure and
supply-chain gaps that audit did not address, informed by controls recently
implemented in the yaha repository.

## Severity Criteria

| Level | Meaning |
|-------|---------|
| High | Exploitable without unusual preconditions, or failure directly compromises repository integrity, credentials, or build artifacts. |
| Medium | Requires a specific precondition or the impact is limited by an external control. |
| Low | Defense-in-depth improvement with no direct exploit path under current conditions. |

## Findings

### CE-01: No CI Pipeline for Pull Requests

**Severity:** High

There is no workflow that runs on `pull_request`. The only required status
check for merging is the 24-hour cooldown. Code can reach `main` without
automated linting, testing, formatting checks, or security audits.

Pre-commit hooks (`clippy`, `rustfmt`, `eslint`, `prettier`) exist locally but
are not enforced in CI. A contributor who skips `pre-commit install` bypasses
all of them.

**Recommendation:**

Add a CI workflow triggered on `pull_request` and `push` to `main` that runs:

```text
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
cargo audit
pnpm install --frozen-lockfile
pnpm audit
pnpm exec eslint .
pnpm exec prettier --check .
```

Make this workflow a required status check for `main`.

### CE-02: Cargo.lock Not Committed

**Severity:** High

`Cargo.lock` is in `.gitignore`. For an application (as opposed to a library),
this means Rust builds are not reproducible: every `cargo build` can silently
resolve to different dependency versions. A compromised or yanked crate version
could enter the build without any reviewer seeing the change.

**Recommendation:**

Remove `Cargo.lock` from `.gitignore` and commit it. Use `cargo install
--locked` semantics in CI. This is the Cargo equivalent of `uv sync --locked`
and follows Rust's own guidance for applications.

### CE-03: GitHub Actions Not SHA-Pinned

**Severity:** High

All workflow actions use mutable tag references:

- `step-security/harden-runner@v2`
- `actions/checkout@v4`
- `actions/setup-node@v4`
- `pnpm/action-setup@v4`
- `dtolnay/rust-toolchain@stable`
- `tauri-apps/tauri-action@v0`

Tags can be force-pushed. A compromised upstream action could execute arbitrary
code in the release workflow, which has `contents: write` permission and access
to `GITHUB_TOKEN`.

`tauri-apps/tauri-action@v0` is particularly broad: `v0` spans the entire 0.x
major version range.

**Recommendation:**

Pin every action to a full commit SHA with a version comment:

```yaml
- uses: actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0 # v7.0.0
```

Enable repository enforcement of full-length SHA pins after converting.

### CE-04: Dependabot Not Configured

**Severity:** High

There is no `.github/dependabot.yml`. Dependabot alerts and security updates
are both disabled at the repository level. There is no automated notification
when a locked dependency becomes vulnerable.

The earlier audit (SEC-03) discussed a security-only Dependabot policy, but
Dependabot was never actually enabled.

**Recommendation:**

Enable the dependency graph, Dependabot alerts, and Dependabot security updates
in repository settings. Add `.github/dependabot.yml`:

```yaml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/src-tauri"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 0

  - package-ecosystem: "npm"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 0

  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 0
```

The zero PR limit suppresses routine version-update PRs while allowing
Dependabot security-update PRs.

### CE-05: Content Security Policy Disabled

**Severity:** High

`tauri.conf.json` sets `"csp": null`, which disables the Content Security
Policy entirely. The webview has no restrictions on script sources, inline
scripts, or network requests.

For a desktop application that simulates keyboard and mouse input at the OS
level, an XSS vulnerability in the webview could escalate to full input
simulation on the host.

**Recommendation:**

Set a restrictive CSP. At minimum:

```json
"csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'"
```

Tighten further once the frontend's actual resource needs are known.

### CE-06: Broad Tauri Capability Grants

**Severity:** Medium

`src-tauri/capabilities/default.json` grants `shell:default` to the main
window. Combined with the disabled CSP, this gives the webview broad shell
execution capabilities.

Other permissions (`opener:default`, `notification:default`, `os:default`,
`global-shortcut:default`) also use unscoped defaults rather than fine-grained
scoped permissions.

The earlier audit (SEC-05) flagged this. No remediation has been applied.

**Recommendation:**

Replace `:default` scopes with the minimum permissions the application actually
uses. Remove `shell:default` if shell execution is not needed from the
frontend, or scope it to specific allowed commands.

### CE-07: Release Builds Are Unsigned

**Severity:** Medium

The release workflow produces binaries for macOS, Linux, and Windows without
code signing. Users downloading ClickEase receive unsigned, unverified
executables for an application that requests elevated input-simulation
privileges.

Unsigned binaries trigger OS security warnings (Gatekeeper on macOS, SmartScreen
on Windows) and cannot be distinguished from tampered copies.

**Recommendation:**

- macOS: Configure Apple notarization via `tauri-apps/tauri-action` with
  `APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`, `APPLE_ID`,
  `APPLE_PASSWORD`, and `APPLE_TEAM_ID` secrets.
- Windows: Use Microsoft Trusted Signing or a code-signing certificate.
- Rotate signing secrets and consider OIDC-based credential access (SEC-06).

### CE-08: DLL Validation Disabled in CI

**Severity:** Medium

The release workflow sets `TAURI_SKIP_DLL_CHECK: 1`, which disables DLL
validation during the build. This suppresses checks that would catch missing or
unexpected DLL dependencies in the built artifact.

**Recommendation:**

Remove `TAURI_SKIP_DLL_CHECK: 1` and resolve any DLL issues it was masking.
If specific DLLs must be excluded, use `TAURI_LINUX_DEPLOY_SKIP_LIBS` (which
is already configured) rather than disabling all validation.

### CE-09: Missing Repository Hygiene Files

**Severity:** Low

- No `LICENSE` file despite the README badge claiming AGPL-3.0.
- No `SECURITY.md` with responsible disclosure instructions.
- No private vulnerability reporting enabled.
- Community health score is 28%.

**Recommendation:**

- Add an `LICENSE` file with the AGPL-3.0 text.
- Add a `SECURITY.md` describing supported versions and how to report
  vulnerabilities.
- Enable GitHub private vulnerability reporting.

### CE-10: Harden Runner in Audit-Only Mode

**Severity:** Low

`step-security/harden-runner` is configured with `egress-policy: audit`. This
logs outbound network connections but does not block unauthorized egress. A
compromised build step could exfiltrate secrets or source code.

**Recommendation:**

Review audit logs to build an egress allowlist, then switch to
`egress-policy: block` with explicit allowed endpoints.

## Recommended Implementation Order

1. Commit `Cargo.lock` and add a required CI workflow.
2. SHA-pin all GitHub Actions.
3. Enable Dependabot alerts and security updates; add `dependabot.yml`.
4. Set a restrictive CSP in `tauri.conf.json`.
5. Scope Tauri capabilities to minimum required permissions.
6. Add `LICENSE` and `SECURITY.md`.
7. Configure code signing for release builds.
8. Remove `TAURI_SKIP_DLL_CHECK` and transition Harden Runner to block mode.
