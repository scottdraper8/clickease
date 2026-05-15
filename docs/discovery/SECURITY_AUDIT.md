# Security Audit Report: Clickease Project

**Date:** 2025-05-22
**Auditor:** Senior Security Architect
**Status:** Initial Review
**Project Phase:** Discovery / Planning

---

## 1. Executive Summary

This audit evaluates the Clickease project across its architectural design, OS-specific tooling choices, and CI/CD configuration. While the project demonstrates strong security foundations (e.g., Tauri v2 ACLs, `harden-runner` in CI), several critical and high-risk issues were identified, particularly in the PR review process and dependency management strategy.

---

## 2. Findings Summary

| ID         | Finding                                        | Risk Level   | Category     |
| :--------- | :--------------------------------------------- | :----------- | :----------- |
| **SEC-01** | PR Age Check Bypass via Synchronize Events     | **Critical** | CI/CD        |
| **SEC-02** | Over-permissive Input Device Access (Linux)    | **High**     | OS Security  |
| **SEC-03** | Dependency Stagnation via Security-Only Policy | **High**     | Supply Chain |
| **SEC-04** | UIPI Bypass Abuse Potential (Windows)          | **Medium**   | OS Security  |
| **SEC-05** | Lack of Fine-Grained Tauri Capabilities        | **Medium**   | Architecture |
| **SEC-06** | Missing OIDC for CI/CD Credentials             | **Medium**   | CI/CD        |
| **SEC-07** | Absence of Automated Dependency Auditing       | **Low**      | Supply Chain |

---

## 3. Detailed Findings & Mitigations

### SEC-01: PR Age Check Bypass via Synchronize Events

- **Risk Level:** **Critical**
- **Description:** The `.github/workflows/pr-age-check.yml` verifies PR age based on `github.event.pull_request.created_at`. This logic does not reset or re-evaluate when new commits are pushed (`synchronize` event). A malicious actor can open a benign PR, wait 24 hours to satisfy the check, and then push a malicious payload. The check will still pass because the PR's _creation_ date is older than 24 hours.
- **Mitigation:** Modify the workflow to check the timestamp of the **latest commit** or use `github.event.pull_request.updated_at` to ensure a 24-hour cooldown after the _last change_.

### SEC-02: Over-permissive Input Device Access (Linux)

- **Risk Level:** **High**
- **Description:** Using `ydotool` or direct `uinput` access typically requires granting the user read/write permissions to `/dev/uinput`. If implemented via broad groups (e.g., `input`), it allows any process under that user to not only simulate input but also **log all system-wide keystrokes**, effectively acting as a hardware-level keylogger.
- **Mitigation:**
  1. Implement a dedicated `udev` rule that restricts `/dev/uinput` access to a specific `clickease` group.
  2. Provide a clear, secure setup script for Linux users rather than suggesting broad permission changes.
  3. Explore `libei` (Emulated Input) as the primary path for Wayland, which uses portal-based permissions.

### SEC-03: Dependency Stagnation via Security-Only Policy

- **Risk Level:** **High**
- **Description:** The `.github/dependabot.yml` ignores all non-security updates. This leads to "dependency rot," where the project misses critical bug fixes, performance improvements, and non-CVE security hardening. Over time, this makes upgrades more difficult and leaves the project vulnerable to logic bugs that haven't been assigned a CVE yet.
- **Mitigation:** Update Dependabot configuration to allow `patch` and `minor` version updates. Use automated CI suites and a "canary" or "dev" branch to validate updates before they reach `main`.

### SEC-04: UIPI Bypass Abuse Potential (Windows)

- **Risk Level:** **Medium**
- **Description:** Using `uiAccess="true"` in the application manifest allows Clickease to bypass User Interface Privilege Isolation (UIPI), enabling it to send inputs to elevated processes (e.g., Admin CMD, Task Manager). If Clickease itself is compromised via an IPC vulnerability, the attacker gains control over high-integrity processes.
- **Mitigation:**
  1. Ensure the Rust backend strictly validates all IPC payloads before passing them to `SendInput`.
  2. Document that the app should be installed in `C:\Program Files` as per Microsoft requirements to minimize the risk of binary hijacking.

### SEC-05: Lack of Fine-Grained Tauri Capabilities

- **Risk Level:** **Medium**
- **Description:** While `ARCHITECTURE.md` mentions Tauri v2 capabilities, the current plan lacks specific "scoped" definitions. A single `simulate_click` command that accepts any coordinate is a powerful primitive that should be restricted.
- **Mitigation:** Define granular capabilities in `src-tauri/capabilities/`. For example, create scopes that only allow clicks within specific app-defined bounds or restrict keyboard input to specific character sets unless explicitly unlocked.

### SEC-06: Missing OIDC for CI/CD Credentials

- **Risk Level:** **Medium**
- **Description:** The CI/CD strategy relies on long-lived GitHub Secrets (e.g., `APPLE_CERTIFICATE`, `APPLE_PASSWORD`). If these secrets are leaked, they remain valid until manually revoked.
- **Mitigation:** For Windows signing, prioritize **Microsoft Trusted Signing** with OIDC (via Azure/Identity) to eliminate long-lived PFX secrets. For macOS, ensure certificate passwords are rotated regularly and minimize the secret's scope.

### SEC-07: Absence of Automated Dependency Auditing

- **Risk Level:** **Low**
- **Description:** The project lacks automated tools to scan for known vulnerabilities in the dependency tree (crates and npm packages) beyond Dependabot's reactive PRs.
- **Mitigation:** Add `cargo-audit` for Rust and `pnpm audit` for the frontend to the `.pre-commit-config.yaml` or as a CI step to fail builds containing known vulnerabilities.

---

## 4. Conclusion

The Clickease project has a solid architectural start, but the current CI/CD and OS-integration plans contain gaps that could be exploited. Implementing the mitigations for **SEC-01** and **SEC-03** should be the immediate priority before the implementation phase begins.
