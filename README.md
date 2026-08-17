# 🗑️ omarchy-debloat

Fast interactive & automated Omarchy / DHH preinstalled packages and launchers purge utility written in **Rust** 🦀.

---

### ⚡ Quick One-Liner Installation

```bash
TAG=$(curl -fsSL https://api.github.com/repos/Praveensenpai/omarchy-debloat/releases/latest | grep '"tag_name"' | sed 's/.*"tag_name": *"\(.*\)".*/\1/') && curl -fsSL "https://raw.githubusercontent.com/Praveensenpai/omarchy-debloat/${TAG}/install.sh" | bash
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

- **Preinstalled Packages**: `1password-beta`, `spotify`, `libreoffice-fresh`, `signal-desktop`, `obsidian`, `cups`, `kdenlive`, `localsend`, `gnome-calculator`, `moonlight-qt`, `omacalc`, `omacut`, `omawrite`, etc.
- **Orphaned Desktop Files**: Cleans up leftover `.desktop` launchers in `~/.local/share/applications/`.
- **Hidden System Launchers**: Injects `NoDisplay=true` to hide cluttering system application menus.
- **NPX Wrapper Stubs**: Cleans up default NPX stubs (`codex`, `copilot`, `opencode`, `pi`, `playwright-cli`).
- **Launcher Cleanups**: Executes `omarchy-webapp-remove-all`, `omarchy-tui-remove-all`, `update-desktop-database`, and `omarchy-restart-walker`.

---

### 📜 License

Distributed under the [MIT License](LICENSE).
