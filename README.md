<div align="center">

<kbd><img width="100%" src="assets/banner.png" alt="Clickease Banner"></kbd>

[![Clickease Version](https://img.shields.io/badge/Clickease-0.0.2-ff79c6?logo=github&logoColor=white&labelColor=6272a4)](https://github.com/scottdraper8/clickease/releases)
[![CI](https://github.com/scottdraper8/clickease/actions/workflows/ci.yml/badge.svg)](https://github.com/scottdraper8/clickease/actions/workflows/ci.yml)
[![Rust 1.80+](https://img.shields.io/badge/Rust-1.80+-ffb86c?logo=rust&logoColor=white&labelColor=6272a4)](https://www.rust-lang.org/)
[![pnpm](https://img.shields.io/badge/pnpm-10.0+-f1fa8c?logo=pnpm&logoColor=282a36&labelColor=6272a4)](https://pnpm.io/)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-8be9fd?logo=opensourceinitiative&logoColor=white&labelColor=6272a4)](LICENSE)

<hr>

Clickease is a cross-platform desktop application designed to automate keyboard and mouse inputs.

<hr>

</div>

## Overview

Clickease abstracts complex operating system security models into a unified automation dashboard. It utilizes native system APIs to ensure high-fidelity input simulation that bypasses standard application-level restrictions.

```mermaid
graph LR
    subgraph Frontend [User Interface]
        UI[Vanilla TS Dashboard]
        Theme[Theming Engine]
    end

    subgraph Backend [Rust Core]
        IPC[Tauri IPC Layer]
        Sched[Async Scheduler]
        State[Managed App State]
    end

    subgraph Platform [OS Implementation]
        Win[SendInput API]
        Mac[Quartz Events]
        Lin[uinput Kernel]
    end

    UI --> IPC
    IPC --> Sched
    Sched --> State
    State --> Win
    State --> Mac
    State --> Lin

    %% Dracula Theme Styling
    style Frontend fill:#282a36,stroke:#6272a4,color:#f8f8f2
    style Backend fill:#44475a,stroke:#bd93f9,color:#f8f8f2
    style Platform fill:#282a36,stroke:#ff79c6,color:#f8f8f2
    style UI fill:#6272a4,color:#f8f8f2
    style Theme fill:#6272a4,color:#f8f8f2
    style IPC fill:#bd93f9,color:#282a36
    style Sched fill:#bd93f9,color:#282a36
    style State fill:#bd93f9,color:#282a36
    style Win fill:#ff79c6,color:#282a36
    style Mac fill:#ff79c6,color:#282a36
    style Lin fill:#ff79c6,color:#282a36
```

<!-- Prettier keeps messing with this admonition -->

> [!IMPORTANT]
>
> **Privileged Access Required**: To ensure reliable input injection across all windows (including elevated ones), Clickease requires **Administrator** privileges on Windows and **Accessibility** permissions on macOS.

## Development

This project is built using **Tauri v2**. To begin development, ensure you have the Rust toolchain and Node.js environment configured.

> [!TIP]
>
> For developers on immutable Linux distributions (Bazzite, Fedora Silverblue), it is recommended to use a **Distrobox** container with the necessary system headers (`webkit2gtk`, `dbus-devel`) installed.

1. **Install Dependencies**:
   ```bash
   pnpm install
   ```
2. **Launch in Dev Mode**:
   ```bash
   pnpm tauri dev
   ```
