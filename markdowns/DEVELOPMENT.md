## Development

This project builds for four targets from a single codebase: **Terminal** (native binary), **Web** (WASM static site), **macOS** (native GUI), and **Windows** (native GUI). Make sure you meet the [development prerequisites](DEVELOPMENT_PREREQUISITES.md) first.

**Run / develop** (from the repo root):

```bash
./run --terminal
./run --wasm
./run --macos
./run --windows
```

`--terminal` runs the native binary via Cargo. `--wasm` builds the site into `docs/` and serves it at http://localhost:4627. `--macos` / `--windows` build and run the native GUI apps.

Or with Cargo / scripts directly:

```bash
cargo run --bin rewordle                                            # terminal
APP_NAME=Rewordle cargo run --features macos-native --bin rewordle_macos  # macOS GUI
APP_NAME=Rewordle cargo run --features windows-native --bin rewordle_windows  # Windows GUI
scripts/build_web.sh --dev --serve                                  # web (dev + serve)
scripts/build_web.sh                                                # web (release build into docs/)
```

App code lives in `src/app/` — game UI in `src/app/ui/`, screens in `src/app/screens/`, shared game logic in `src/game.rs`. Entry points: `src/main.rs` (terminal), `src/macos.rs` (macOS GUI), `src/windows.rs` (Windows GUI), `src/lib.rs` (web).

**Publish with GitHub Actions:**

Two workflows in `.github/workflows/` build and distribute for you on GitHub's servers:

- **Create Downloadable Binaries** (`downloadable_binaries.yml`) — builds release binaries and attaches them to a GitHub Release.
- **Deploy to GitHub Pages** (`github_pages.yml`) — builds the WASM/web version and deploys it as a static site, automatically on every push to `main` (or manually).

> Note: the workflows require an `INCREDIBLE_ALPHA` secret (a GitHub token with read access to the private crate) under Settings > Secrets and variables > Actions.
