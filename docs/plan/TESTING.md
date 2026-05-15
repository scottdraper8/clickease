# Clickease Testing Strategy

This document outlines the testing approach for Clickease to ensure reliability across all supported platforms.

## 1. Frontend Testing (Vanilla TS)

- **Framework:** [Vitest](https://vitest.dev/)
- **Scope:**
  - Unit tests for scheduling logic, input validation, and state management.
  - Component tests (using JSDOM) for Neumorphic UI elements and theme switching.
- **Commands:** `pnpm test`

## 2. Backend Testing (Rust)

- **Framework:** Standard Rust `test` module.
- **Scope:**
  - **Unit Tests:** Logic for timing, sequence parsing, and platform abstraction traits.
  - **Integration Tests:** OS-specific input simulation (where possible in CI) and IPC command handlers.
- **Commands:** `cargo test`

## 3. End-to-End (E2E) Testing

- **Framework:** [Playwright](https://playwright.dev/) with [Tauri-Action](https://github.com/tauri-apps/tauri-action) integration.
- **Scope:** Verifying the full flow from UI configuration to simulated backend execution.
- **Note:** Real input simulation may be limited in headless CI environments; focus on IPC contract verification.

## 4. CI Integration

- Every PR triggers the full suite of unit and integration tests across Windows, macOS, and Linux runners.
- Coverage reports are generated and uploaded as artifacts.
