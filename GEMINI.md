# Clickease Project Rules

## Engineering Standards

- **Hook Adherence**: NEVER skip or bypass pre-commit hooks or CI/CD checks. All checks must pass before code is committed or merged.
- **No Inline Suppression**: NEVER use inline lint suppression comments (e.g., `eslint-disable`, `#[allow(...)]`, `// @ts-ignore`). All linting or type errors must be fixed at the root cause or addressed via project-wide configuration if the rule is truly inapplicable.
- **Emoji Prohibition**: NEVER use emojis in any capacity, including UI text, documentation (README, docs/), or code comments. Use text labels or standard symbols if icons are needed.
- **Root-Cause Fixes**: Always trace issues to their source. Avoid superficial patches or "just-in-case" logic.

## Architecture

- **Tech Stack**: Tauri v2 (Rust) + Vite (Vanilla TypeScript) + Tailwind CSS.
- **Package Manager**: pnpm.
- **OS Native Priority**: Prefer native OS APIs (SendInput, Quartz, uinput) over third-party abstractions when possible.
