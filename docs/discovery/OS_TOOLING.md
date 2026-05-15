# OS Tooling Research: Input Injection

This document outlines the low-level capabilities, limitations, and requirements for programmatically sending key and mouse events on Linux, macOS, and Windows.

---

## 1. Linux

### Primary Focus: `ydotool`

`ydotool` is a high-level command-line utility designed to work across different display protocols (X11, Wayland) and even in the TTY.

- **Capabilities:**
  - Simulates keyboard typing, key presses, mouse clicks, and mouse movements.
  - Protocol-agnostic: Works on Wayland (where `xdotool` fails) because it operates at the device level.
- **Limitations:**
  - **No Window Awareness:** Cannot target specific windows (e.g., "type in Chrome") because it operates below the window manager level.
  - **Daemon Dependency:** Requires `ydotoold` to be running. The daemon creates and maintains the virtual input device.
  - **Latency:** Slight overhead compared to direct `uinput` calls due to the client-daemon communication.
- **Requirements:**
  - **Permissions:** The daemon needs read/write access to `/dev/uinput`.
  - **Setup:** Usually requires a `udev` rule to allow a non-root user/group access to `/dev/uinput`, or running the daemon as root.

### Fallback: `uinput`

`uinput` is a Linux kernel module that allows userspace programs to create virtual input devices.

- **Capabilities:**
  - **Full Emulation:** Can create virtual keyboards, mice, joysticks, and touchscreens.
  - **System-Wide:** Events are processed by the kernel and passed to the active session (X11, Wayland, or Console).
- **Limitations:**
  - **Complexity:** Requires low-level programming (C `ioctl` calls or libraries like `libevdev`).
  - **No High-Level Logic:** You must manually handle key-up/key-down sequences and timing.
- **Security Warning (SEC-02):** Granting broad access to `/dev/uinput` (e.g., via the `input` group) allows any process to act as a hardware-level keylogger.
- **Mitigation:**
  1. Use a dedicated `udev` rule to restrict access to a specific `clickease` user/group.
  2. Prioritize `libei` on modern Wayland compositors for better permission scoping.
- **Requirements:**
  - **Administrative Privileges:** Access to `/dev/uinput` is typically restricted to root.
  - **Configuration:** Requires `CONFIG_INPUT_UINPUT` to be enabled in the kernel (standard on most distros).

---

## 2. macOS

### Native Method: Quartz Event Services

Quartz Event Services (part of the `Core Graphics` framework) is the primary way to interact with the macOS event stream.

- **Capabilities:**
  - **Injection:** `CGEventCreateKeyboardEvent` and `CGEventPost` allow injecting keystrokes globally or to specific PIDs.
  - **Observation/Filtering:** `CGEventTap` allows an app to intercept, modify, or suppress system-wide events.
- **Limitations:**
  - **Secure Input Mode:** If the user is focused on a password field, the system enables "Secure Event Input," which encrypts/hides keyboard events from all global taps and may block injection.
  - **Sandbox Restrictions:** Sandboxed apps are heavily restricted and generally cannot post events to other applications.
- **Requirements:**
  - **Accessibility Permissions (TCC):** The application must be granted "Accessibility" permissions in _System Settings > Privacy & Security_.
  - **Programmatic Check:** Use `AXIsProcessTrusted()` to verify if permissions are granted.

---

## 3. Windows

### Native Method: `SendInput`

`SendInput` is the modern Windows API for simulating input, replacing the deprecated `keybd_event`.

- **Capabilities:**
  - **Serialized Input:** Can send an array of `INPUT` structures (keyboard, mouse, or hardware) as a single atomic block, preventing interleaving with physical user input.
  - **Scan Codes:** Supports sending hardware scan codes (`KEYEVENTF_SCANCODE`), which is more reliable for games and low-level applications than virtual keys.
  - **Unicode Support:** Can send characters directly using `KEYEVENTF_UNICODE`.
- **Limitations:**
  - **Injected Flag:** Events are marked with `LLKHF_INJECTED`, which can be detected by low-level hooks (used by anti-cheat or security software).
  - **Physical State:** Does not automatically account for the state of modifier keys (Shift, Ctrl) unless explicitly handled in the input array.
- **Requirements:**
  - **User Interface Privilege Isolation (UIPI):** A process can only send input to windows of processes at an equal or lower integrity level. (e.g., a standard app cannot control an Admin terminal).
  - **Bypassing UIPI:** Requires the application to:
    1. Set `uiAccess="true"` in its manifest.
    2. Be digitally signed.
    3. Be installed in a secure location (e.g., `C:\Program Files`).

---

## Summary Comparison

| OS          | Recommended Tool/API  | Protocol      | Permission Level           | Key Limitation             |
| :---------- | :-------------------- | :------------ | :------------------------- | :------------------------- |
| **Linux**   | `ydotool` / `uinput`  | Kernel-level  | `/dev/uinput` (Group/Root) | No window context          |
| **macOS**   | Quartz Event Services | Core Graphics | Accessibility (TCC)        | Blocked by Secure Input    |
| **Windows** | `SendInput`           | User32        | UIPI (Integrity Level)     | Integrity level boundaries |
