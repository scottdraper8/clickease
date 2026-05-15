# Clickease Repository Structure

This document defines the organized folder structure for the Clickease project, ensuring a clean separation between the Rust backend (Tauri v2) and the Vanilla TypeScript frontend (Vite + pnpm).

## Folder Hierarchy

```text
clickease/
├── .github/                # CI/CD Workflows (GitHub Actions)
│   └── workflows/          # Build, Lint, and Release pipelines
├── .vscode/                # Shared VS Code settings and extensions
├── docs/                   # Documentation (Design, Plans, Discovery)
│   ├── discovery/          # Research findings
│   └── plan/               # Implementation and architectural plans
├── public/                 # Static assets for the frontend (icons, fonts)
├── src/                    # Frontend source (Vanilla TypeScript)
│   ├── assets/             # CSS, images, and other source assets
│   ├── components/         # Framework-less UI components
│   ├── services/           # TS wrappers for Tauri commands
│   ├── styles/             # Global CSS and Tailwind configuration
│   ├── main.ts             # Frontend entry point
│   └── index.html          # Main HTML entry
├── src-tauri/              # Rust backend (Tauri v2)
│   ├── capabilities/       # Permission definitions (ACL)
│   ├── icons/              # Application bundle icons
│   ├── src/                # Rust source code
│   │   ├── commands/       # Tauri command implementations
│   │   ├── engine/         # Core simulation logic (traits and OS implementations)
│   │   ├── main.rs         # Application entry point and setup
│   │   └── lib.rs          # Shared logic and module declarations
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
├── .gitignore              # Standard ignore patterns
├── .pre-commit-config.yaml # Pre-commit hooks configuration
├── package.json            # Frontend dependencies and scripts (pnpm)
├── pnpm-lock.yaml          # pnpm lockfile
├── tsconfig.json           # TypeScript configuration
├── vite.config.ts          # Vite configuration
└── README.md               # Project overview
```

## Rationale

1.  **Tauri Convention:** Using `src-tauri` is the standard and expected structure for Tauri projects, facilitating tool compatibility and developer onboarding.
2.  **Frontend at Root/src:** Placing the frontend source in `src/` at the root keeps the top-level clean and follows common Vite project structures.
3.  **No-Framework Clarity:** The `src/components/` folder will contain pure TypeScript modules that manage DOM elements directly, reinforcing the "No React" mandate.
4.  **Backend Modularization:** Separating `engine` (low-level logic) from `commands` (IPC layer) ensures the simulation logic is testable in isolation.
5.  **pnpm:** Chosen for its efficiency and speed in managing node modules.
