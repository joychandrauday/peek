# PEEK

> Lightweight desktop overlay assistant for instant answers without context switching.

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)
![Tauri](https://img.shields.io/badge/Tauri-v2-orange)
![React](https://img.shields.io/badge/React-18-blue)
![TypeScript](https://img.shields.io/badge/TypeScript-5-blue)

## What is PEEK?

PEEK is a lightweight desktop overlay that helps developers, students, and knowledge workers get instant answers without leaving their workflow.

**Core Experience:**
1. Press `Ctrl+Space` from anywhere
2. Type your query (e.g., "docker remove unused images")
3. Get instant answer (e.g., `docker image prune -a`)
4. Copy and continue working

**Not a chatbot** — an instant knowledge retrieval layer.

![PEEK Demo](https://github.com/joychandrauday/peek/blob/main/assets/demo.gif?raw=true)

## Features

| Feature | Description |
|---------|-------------|
| **Global Shortcut** | `Ctrl+Space` opens overlay from any application |
| **Fuzzy Search** | Find answers even with partial or imprecise queries |
| **Instant Copy** | Click or press Enter to copy answers |
| **Local Database** | Works offline with built-in knowledge base |
| **AI Fallback** | Optional OpenRouter integration for unknown queries |
| **Lightweight** | Minimal resource usage, fast startup |

## Download

### Pre-built Binaries

Download the latest release from [GitHub Releases](https://github.com/joychandrauday/peek/releases):

| Platform | File | Size |
|----------|------|------|
| **Windows** | `PEEK_0.1.0_x64-setup.msi` | ~5 MB |
| **macOS (Intel)** | `PEEK_0.1.0_x64.dmg` | ~6 MB |
| **macOS (Apple Silicon)** | `PEEK_0.1.0_aarch64.dmg` | ~5 MB |
| **Linux (Debian/Ubuntu)** | `peek_0.1.0_amd64.deb` | ~5 MB |
| **Linux (AppImage)** | `peek_0.1.0_amd64.AppImage` | ~8 MB |

### Install Instructions

#### Windows

1. Download the `.msi` installer
2. Double-click to run installer
3. Follow the installation wizard
4. PEEK will start in the system tray
5. Press `Ctrl+Space` to open

#### macOS

1. Download the `.dmg` file
2. Open the DMG and drag PEEK to Applications
3. **First launch:** Right-click PEEK → Open (to bypass Gatekeeper)
4. Grant Accessibility permissions when prompted (for global shortcut)
5. Press `Ctrl+Space` to open

#### Linux

**Debian/Ubuntu:**
```bash
sudo dpkg -i peek_0.1.0_amd64.deb
```

**AppImage:**
```bash
chmod +x peek_0.1.0_amd64.AppImage
./peek_0.1.0_amd64.AppImage
```

## Usage

### Basic Search

1. Press `Ctrl+Space` to open PEEK
2. Type your query in the search box
3. Browse results with arrow keys
4. Press `Enter` or click to copy answer
5. Press `Escape` to close

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Space` | Toggle PEEK overlay |
| `↑` `↓` | Navigate results |
| `Enter` | Copy selected answer |
| `Escape` | Close overlay |

### AI Integration (Optional)

PEEK works without AI, but you can enable OpenRouter for additional queries:

1. Click settings icon (bottom-right)
2. Enable AI Fallback
3. Enter your [OpenRouter API key](https://openrouter.ai/keys)
4. Click "Test Connection"

**Free tier available:** Uses `openrouter/free` model by default.

## Built-in Knowledge Base

PEEK ships with **200+ commands** across 6 categories:

| Category | Commands | Examples |
|----------|----------|----------|
| **Git** | 35 | Branch, commit, merge, stash, rebase |
| **Docker** | 35 | Containers, images, compose, networks |
| **Linux** | 50 | Files, processes, networking, systemd |
| **JavaScript** | 45 | Array methods, async, ES6+, proxies |
| **React** | 40 | Hooks, components, patterns, router |
| **SQL** | 45 | Queries, joins, indexes, transactions |

### Adding Custom Commands

Create a JSON file in `data/` directory:

```json
[
  {
    "title": "Your command title",
    "answer": "the command or answer",
    "category": "category",
    "tags": "comma,separated,tags"
  }
]
```

Then import:

```bash
npm run import
```

## Build from Source

### Prerequisites

| Requirement | Version | Install |
|-------------|---------|---------|
| **Node.js** | v18+ | [nodejs.org](https://nodejs.org/) |
| **Rust** | Latest | [rustup.rs](https://rustup.rs/) |

**Platform-specific dependencies:**

#### Windows

```powershell
# Install Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
# Select "Desktop development with C++" workload
```

#### macOS

```bash
xcode-select --install
```

#### Linux (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install -y \
  pkg-config \
  libgtk-3-dev \
  libwebkit2gtk-4.1-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

### Development

```bash
# Clone repository
git clone https://github.com/joychandrauday/peek.git
cd peek

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Frontend only (faster, no Rust compilation)
npm run dev
```

### Production Build

```bash
# Build for current platform
npm run tauri build

# Output locations:
# Windows: src-tauri/target/release/bundle/msi/
# macOS:   src-tauri/target/release/bundle/dmg/
# Linux:   src-tauri/target/release/bundle/deb/
```

## Tech Stack

| Component | Technology | Purpose |
|-----------|------------|---------|
| Desktop | [Tauri v2](https://tauri.app/) | Native desktop shell |
| Frontend | [React](https://react.dev/) + [TypeScript](https://www.typescriptlang.org/) | UI framework |
| Styling | [Tailwind CSS](https://tailwindcss.com/) | Utility-first CSS |
| Database | SQLite (via [rusqlite](https://github.com/rusqlite/rusqlite)) | Local data storage |
| Search | Custom scoring algorithm | Fuzzy matching |
| AI | [OpenRouter](https://openrouter.ai/) | Optional AI fallback |
| Build | [Vite](https://vitejs.dev/) | Frontend bundler |

## Project Structure

```
peek/
├── src/                        # React frontend
│   ├── components/
│   │   ├── Overlay/            # Main overlay container
│   │   ├── SearchBox/          # Search input with keyboard nav
│   │   ├── AnswerCard/         # Result display with copy button
│   │   └── Settings/           # AI settings panel
│   ├── services/
│   │   ├── search.ts           # Search API client
│   │   └── ai.ts               # AI API client
│   ├── hooks/
│   │   ├── useSearch.ts        # Search state management
│   │   └── useTauri.ts         # Tauri IPC hook
│   ├── App.tsx                 # Root component
│   └── main.tsx                # Entry point
├── src-tauri/                  # Rust backend
│   ├── src/
│   │   ├── main.rs             # Entry point
│   │   ├── lib.rs              # Tauri setup & plugin registration
│   │   ├── db/
│   │   │   ├── mod.rs          # Database operations
│   │   │   └── schema.rs       # Schema & seed data
│   │   ├── commands/
│   │   │   ├── search.rs       # Search IPC handler
│   │   │   ├── database.rs     # CRUD operations
│   │   │   └── ai.rs           # AI IPC handler
│   │   └── ai/
│   │       ├── mod.rs          # AI module
│   │       ├── types.rs        # AI types
│   │       └── openrouter.rs   # OpenRouter client
│   └── tauri.conf.json         # Tauri configuration
├── data/                       # Knowledge base (JSON)
│   ├── git.json
│   ├── docker.json
│   ├── linux.json
│   ├── javascript.json
│   ├── react.json
│   └── sql.json
├── scripts/
│   └── import.ts               # JSON → SQLite importer
├── package.json
├── tsconfig.json
├── vite.config.ts
└── tailwind.config.js
```

## Architecture

### Local-First Design

```
User Query → SQLite Search → Score ≥ 50%? → Return Result
                                ↓ No
                         AI Config Set? → Call OpenRouter → Return AI Answer
                                ↓ No
                         Return "No answer found"
```

### Search Algorithm

PEEK uses a custom scoring algorithm (not Fuse.js):

| Match Type | Score |
|------------|-------|
| Exact title match | 100 |
| Title starts with query | 90 |
| Title contains query | 80 |
| Answer contains query | 70 |
| Tag match | 60 |
| Word-by-word match | Up to 50 |

### Performance Goals

| Metric | Target |
|--------|--------|
| Startup time | < 2 seconds |
| Search latency | < 100ms |
| Memory usage | < 150MB |
| Bundle size | < 10MB |

## Development Commands

```bash
# Frontend
npm run dev          # Vite dev server (port 1420)
npm run build        # TypeScript + Vite build
npm run preview      # Preview production build

# Tauri
npm run tauri dev    # Full dev mode (frontend + backend)
npm run tauri build  # Production build for current platform

# Utilities
npm run import       # Import JSON knowledge base to SQLite
```

## Troubleshooting

### Common Issues

| Issue | Solution |
|-------|----------|
| `Ctrl+Space` doesn't work | Grant accessibility permissions (macOS) or run as admin (Windows) |
| Window doesn't appear | Check system tray for PEEK icon, click to toggle |
| "Unidentified developer" (macOS) | Right-click → Open, or: `xattr -cr /Applications/PEEK.app` |
| Search returns no results | Ensure database is seeded (check `peek.db` in app data dir) |
| AI not working | Verify API key in Settings, check internet connection |

### System Tray

PEEK runs in the system tray. Right-click the icon for options:
- **Show/Hide:** Toggle the overlay
- **Settings:** Open AI settings
- **Quit:** Exit PEEK

## Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing`)
5. Open Pull Request

### Code Style

- **TypeScript:** Strict mode, no `any`
- **Rust:** Follow `rustfmt` conventions
- **CSS:** Tailwind utility classes
- **Commits:** Conventional commits format

## Roadmap

- [ ] Global shortcut customization
- [ ] More knowledge base categories
- [ ] Custom knowledge base import UI
- [ ] Windows/Linux shortcuts (`Ctrl+Space`)
- [ ] macOS shortcut (`Cmd+Space`)
- [ ] Auto-update support
- [ ] Cloud sync (optional)

## License

MIT License - see [LICENSE](LICENSE) for details.

## Support

- [GitHub Issues](https://github.com/joychandrauday/peek/issues)
- [Discussions](https://github.com/joychandrauday/peek/discussions)

---

Built with Tauri + React
