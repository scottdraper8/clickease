# OS Publishing & Ecosystem Strategy

This document outlines the recommended strategies for releasing and distributing Clickease across macOS, Windows, and Linux, ensuring security compliance and a smooth user experience.

## 1. Executive Summary

| OS          | Recommended Format      | Distribution Strategy | Key Requirement                                |
| :---------- | :---------------------- | :-------------------- | :--------------------------------------------- |
| **macOS**   | `.dmg`                  | GitHub Releases       | Apple Notarization + Accessibility Permissions |
| **Windows** | `.msi` or `.exe` (NSIS) | GitHub + MS Store     | Code Signing + `uiAccess` (Secure Location)    |
| **Linux**   | `AppImage`              | GitHub Releases       | `libei` (Wayland) / X11 fallback               |

---

## 2. macOS Distribution: Gatekeeper & Notarization

Apple's **Gatekeeper** prevents the execution of software not signed by a recognized developer and notarized by Apple.

### 2.1 Release Format

- **Recommendation:** `.dmg` (Disk Image).
- **Why:** It is the standard distribution format for macOS, allowing for a branded installer window with a "Drag to Applications" shortcut.

### 2.2 Notarization Workflow (GitHub Actions)

To avoid the "App cannot be opened because it is from an unidentified developer" warning, Clickease must be notarized.

1.  **Certificate:** Requires a **Developer ID Application** certificate from the Apple Developer Program ($99/year).
2.  **Process:** The `tauri-apps/tauri-action` can automate this by providing:
    - `APPLE_CERTIFICATE`: Base64 encoded `.p12` file.
    - `APPLE_CERTIFICATE_PASSWORD`: Password for the `.p12`.
    - `APPLE_ID` & `APPLE_PASSWORD`: App-specific password for the Apple ID.
    - `APPLE_TEAM_ID`: Developer Team ID.

### 2.3 Critical Hurdle: Accessibility Permissions

Since Clickease simulates inputs, it **must** be granted Accessibility permissions by the user.

- **UX Strategy:** On first launch (or when a simulation is attempted), detect missing permissions and provide a deep link to: `x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility`.

---

## 3. Windows Distribution: UIPI & SmartScreen

Windows presents two major hurdles: reputation (SmartScreen) and privilege isolation (UIPI).

### 3.1 The `uiAccess="true"` Requirement

To simulate inputs into elevated windows (e.g., Task Manager) without running Clickease itself as Administrator, the app must use `uiAccess="true"` in its manifest.

- **Requirement 1: Secure Location.** The app **must** be installed in a directory writeable only by admins (e.g., `C:\Program Files`). This makes **raw binaries (.exe) non-viable** for full functionality; an **installer (MSI or NSIS)** is mandatory.
- **Requirement 2: Trusted Signature.** The binary must be signed with a certificate trusted by the machine.

### 3.2 SmartScreen Reputation Strategy

New certificates have zero reputation, causing "Windows protected your PC" warnings.

- **Tier 1 (Instant Trust):** Publish to the **Microsoft Store**. Store apps carry Microsoft's reputation.
- **Tier 2 (Modern Path):** Use **Microsoft Trusted Signing** (formerly Azure Code Signing). It is cheaper (~$10/mo) and integrates with CI/CD.
- **Tier 3 (Manual Boost):** For every release, manually submit the installer to the [Microsoft Security Intelligence Portal](https://www.microsoft.com/en-us/wdsi/filesubmission) to accelerate reputation building.

---

## 4. Linux Distribution: Wayland & Fragmentation

### 4.1 Release Format

- **Recommendation:** `AppImage`.
- **Why:** It is a single, self-contained binary that works across most distributions without installation. For native feel, `.deb` (Debian/Ubuntu) and `.rpm` (Fedora) should also be provided.

### 4.2 The Wayland Hurdle

Standard input simulation libraries (like `enigo` on X11) fail on Wayland due to security restrictions.

- **Strategy:**
  - **X11:** Use standard X11 extensions.
  - **Wayland:** Use **`libei` (Emulated Input)** via the `reis` or `eitype` Rust crates. This requires the **XDG RemoteDesktop portal**, which will prompt the user for permission.

---

## 5. Ecosystem & Malware Avoidance

Given that Clickease simulates inputs, it is at high risk of being flagged as "Suspicious" or "Malware" (HEUR/Ares or similar).

### 5.1 Best Practices

1.  **Code Signing:** Never release unsigned binaries. An unsigned "input simulator" is an immediate red flag for AV engines.
2.  **Timestamping:** Always use a RFC 3161 compliant timestamp server during signing to ensure the signature remains valid after certificate expiry.
3.  **Consistency:** Use the same certificate thumbprint for all releases to build long-term reputation.
4.  **Transparency:** Include a clear "About" section and links to source code in the UI to build user trust.

## 6. Implementation Checklist (GitHub Actions)

- [ ] Configure `tauri-action` for multi-platform builds.
- [ ] Set up GitHub Secrets for Apple Notarization.
- [ ] Set up GitHub Secrets for Windows Code Signing (via Trusted Signing or PFX).
- [ ] Configure `tauri.conf.json` to generate:
  - `dmg` for macOS.
  - `msi` for Windows.
  - `AppImage` and `deb` for Linux.
