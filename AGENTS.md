# AGENTS.md

## Project Overview

PEEK is a Tauri v2 desktop overlay assistant for instant command lookup. Dual build system: Vite (frontend) + Cargo (backend).

## Build Commands

```bash
# Frontend only (TypeScript + Vite)
npm run build

# Full Tauri dev (starts both frontend + backend)
npm run tauri dev

# Production build
npm run tauri build

# Frontend dev server only (for UI iteration)
npm run dev  # Runs on port 1420

# Import knowledge base (JSON to SQLite)
npm run import
```

**Important:** `npm run tauri dev` runs `npm run dev` automatically via `beforeDevCommand` in `tauri.conf.json`. Don't run both.

## Project Structure

```
src/                    # React frontend (TypeScript)
src-tauri/              # Rust backend
  src/
    main.rs             # Entry point (calls peek_lib::run())
    lib.rs              # Tauri setup, plugin registration, command handlers
    db/                 # SQLite operations (rusqlite)
    commands/           # Tauri IPC command handlers
    ai/                 # OpenRouter integration
data/                   # Knowledge base JSON files
scripts/                # Import tools
```

## Key Architecture Notes

- **Window config:** 500x400px, no decorations, transparent, always-on-top, hidden by default (`tauri.conf.json:13-25`)
- **Global shortcut:** `Ctrl+Space` toggles overlay (configured in Tauri)
- **Database:** SQLite stored in app data dir (`peek.db`), auto-seeded on first run
- **Search:** Custom scoring in `src-tauri/src/commands/search.rs` with AI fallback
- **AI:** Optional OpenRouter integration, settings stored in localStorage

## Code Conventions

- TypeScript strict mode, no `any`
- Tailwind utility classes for styling
- Path alias: `@/` maps to `./src/`
- Rust: standard `rustfmt` style

## Known Issues

- No test scripts defined
- `fuse.js` in dependencies but search uses custom scoring in Rust
- Rust build requires system dependencies (pkg-config, glib-2.0, etc.)

## Verification

No automated tests configured. Manual verification:
1. `npm run build` - Verify TypeScript compiles
2. `cargo check` (in `src-tauri/`) - Verify Rust compiles (requires system deps)
3. `npm run tauri dev` - Verify app launches
