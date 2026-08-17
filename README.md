# 🗑️ omarchy-debloat

Fast interactive & automated Omarchy / DHH preinstalled packages and launchers purge utility written in **Rust** 🦀.

---

### ⚡ Quick One-Liner Installation

```bash
curl -fsSL -H 'Cache-Control: no-cache' "https://raw.githubusercontent.com/Praveensenpai/omarchy-debloat/main/install.sh?v=$(date +%s)" | bash
```

---

### 🚀 Usage

#### 1. Interactive Mode (Default)
Select/unselect which preinstalled packages to purge using an interactive terminal checkbox menu:

```bash
omarchy-debloat
```

#### 2. Non-Interactive Purge All
Purge all detected default Omarchy/DHH packages and launchers non-interactively (ideal for automated dotfiles setup):

```bash
omarchy-debloat --all
```

#### 3. List Detected Packages
Scan and list installed preinstalled packages without removing anything:

```bash
omarchy-debloat --list
```

---

### 📦 Purged Target Applications & Components

- **Preinstalled Packages**: `1password-beta`, `spotify`, `libreoffice-fresh`, `signal-desktop`, `obsidian`, `cups`, `kdenlive`, `localsend`, `gnome-calculator`, etc.
- **Orphaned Desktop Files**: Cleans up leftover `.desktop` launchers in `~/.local/share/applications/`.
- **Hidden System Launchers**: Injects `NoDisplay=true` to hide cluttering system application menus.
- **NPX Wrapper Stubs**: Cleans up default NPX stubs (`codex`, `copilot`, `opencode`, `pi`, `playwright-cli`).
- **Launcher Cleanups**: Executes `omarchy-webapp-remove-all`, `omarchy-tui-remove-all`, `update-desktop-database`, and `omarchy-restart-walker`.

---

### 📜 License

Distributed under the [MIT License](LICENSE).
