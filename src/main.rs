use clap::Parser;
use colored::*;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const DEFAULT_PACKAGES: &[&str] = &[
    "1password-beta",
    "1password-cli",
    "aether",
    "cliamp",
    "typora",
    "spotify",
    "libreoffice-fresh",
    "xournalpp",
    "signal-desktop",
    "pinta",
    "obsidian",
    "obs-studio",
    "kdenlive",
    "lazydocker",
    "claude-code",
    "chromium",
    "localsend",
    "localsend-bin",
    "gnome-calculator",
    "system-config-printer",
    "cups",
    "cups-filters",
    "cups-browsed",
    "cups-pdf",
];

const DESKTOP_ENTRIES_TO_REMOVE: &[&str] = &[
    "typora.desktop",
    "localsend.desktop",
    "org.freedesktop.IBus.Setup.desktop",
    "org.gnome.Evince.desktop",
];

const DESKTOP_ENTRIES_TO_HIDE: &[&str] = &[
    "org.freedesktop.IBus.Setup.desktop",
    "gnome-color-panel.desktop",
    "org.gnome.ColorProfileViewer.desktop",
    "org.gnome.Evince.desktop",
];

const NPX_STUBS_TO_REMOVE: &[&str] = &[
    "codex",
    "copilot",
    "opencode",
    "playwright-cli",
    "pi",
];

#[derive(Parser, Debug)]
#[command(
    name = "omarchy-debloat",
    author = "Praveen Senpai <pvnt20@gmail.com>",
    version = "0.1.0",
    about = "Fast interactive & automated Omarchy/DHH preinstalled packages and launchers purge utility"
)]
struct Args {
    /// Non-interactive mode: purge all detected default packages and launchers without prompts
    #[arg(short, long)]
    all: bool,

    /// Automatic confirmation for interactive prompts
    #[arg(short, long)]
    yes: bool,

    /// List currently installed default Omarchy/DHH packages without removing
    #[arg(short, long)]
    list: bool,
}

fn is_package_installed(pkg: &str) -> bool {
    let output = Command::new("pacman")
        .args(["-Qq", pkg])
        .output();
    match output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}

fn remove_package(pkg: &str) -> bool {
    // Try sudo pacman -Rns --noconfirm <pkg>
    let status = Command::new("sudo")
        .args(["pacman", "-Rns", "--noconfirm", pkg])
        .status();

    if let Ok(s) = status {
        if s.success() {
            return true;
        }
    }

    // Fallback: sudo pacman -R --noconfirm <pkg>
    let fallback = Command::new("sudo")
        .args(["pacman", "-R", "--noconfirm", pkg])
        .status();

    matches!(fallback, Ok(s) if s.success())
}

fn print_banner() {
    println!("{}", "=========================================================".magenta());
    println!("{}", "   🗑️  Omarchy/DHH Debloat & Purge Utility v0.1.0 (Rust)   ".bold().cyan());
    println!("{}", "=========================================================".magenta());
    println!();
}

fn get_home_dir() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string())
}

fn clean_desktop_entries() {
    let home = get_home_dir();
    let local_apps = format!("{}/.local/share/applications", home);
    let _ = fs::create_dir_all(&local_apps);

    // Remove orphaned desktop entries
    for entry in DESKTOP_ENTRIES_TO_REMOVE {
        let file_path = format!("{}/{}", local_apps, entry);
        if Path::new(&file_path).exists() {
            let _ = fs::remove_file(&file_path);
        }
    }

    // Hide desktop entries (NoDisplay=true)
    for entry in DESKTOP_ENTRIES_TO_HIDE {
        let target_local = format!("{}/{}", local_apps, entry);
        let sys_file = format!("/usr/share/applications/{}", entry);

        if Path::new(&sys_file).exists() {
            if let Ok(content) = fs::read_to_string(&sys_file) {
                let mut new_content = String::new();
                let mut inserted = false;

                for line in content.lines() {
                    new_content.push_str(line);
                    new_content.push('\n');
                    if !inserted && line.trim() == "[Desktop Entry]" {
                        new_content.push_str("NoDisplay=true\n");
                        inserted = true;
                    }
                }

                if !inserted {
                    new_content.push_str("NoDisplay=true\n");
                }

                let _ = fs::write(&target_local, new_content);
            }
        }
    }
}

fn clean_npx_stubs() {
    let home = get_home_dir();
    let local_bin = format!("{}/.local/bin", home);

    for stub in NPX_STUBS_TO_REMOVE {
        let file_path = format!("{}/{}", local_bin, stub);
        if Path::new(&file_path).exists() {
            let _ = fs::remove_file(&file_path);
        }
    }
}

fn run_omarchy_helper_commands() {
    // omarchy-webapp-remove-all
    let _ = Command::new("omarchy-webapp-remove-all").status();

    // omarchy-tui-remove-all
    let _ = Command::new("omarchy-tui-remove-all").status();

    // update-desktop-database
    let home = get_home_dir();
    let local_apps = format!("{}/.local/share/applications", home);
    let _ = Command::new("update-desktop-database")
        .arg(&local_apps)
        .status();

    // omarchy-restart-walker
    let _ = Command::new("omarchy-restart-walker").status();
}

fn main() {
    let args = Args::parse();
    print_banner();

    // Scan installed default packages
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message("Scanning system for preinstalled Omarchy/DHH packages...");

    let mut installed_pkgs: Vec<&str> = Vec::new();
    for pkg in DEFAULT_PACKAGES {
        if is_package_installed(pkg) {
            installed_pkgs.push(pkg);
        }
    }
    spinner.finish_and_clear();

    if args.list {
        println!("{}", "📋 Detected Preinstalled Packages:".bold().yellow());
        if installed_pkgs.is_empty() {
            println!("{}", "  ✔ None detected (system is clean!)".green());
        } else {
            for pkg in &installed_pkgs {
                println!("  • {}", pkg.red());
            }
        }
        return;
    }

    let pkgs_to_purge: Vec<&str> = if args.all || args.yes {
        installed_pkgs.clone()
    } else if installed_pkgs.is_empty() {
        Vec::new()
    } else {
        println!("{}", "🔍 Select packages to purge:".bold().yellow());
        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Use [Space] to select/deselect, [Enter] to confirm")
            .items(&installed_pkgs)
            .defaults(&vec![true; installed_pkgs.len()])
            .interact();

        match selections {
            Ok(indices) => indices.into_iter().map(|i| installed_pkgs[i]).collect(),
            Err(_) => {
                println!("{}", "❌ Selection cancelled.".red());
                return;
            }
        }
    };

    if !pkgs_to_purge.is_empty() {
        println!(
            "{}",
            format!("🗑️ Purging {} package(s)...", pkgs_to_purge.len())
                .bold()
                .purple()
        );
        for pkg in &pkgs_to_purge {
            print!("  • Removing {}... ", pkg.yellow());
            let _ = std::io::stdout().flush();
            if remove_package(pkg) {
                println!("{}", "✔ Done".green());
            } else {
                println!("{}", "✘ Skipped / Error".red());
            }
        }
    } else {
        println!("{}", "✔ No packages selected for removal.".green());
    }

    println!("{}", "🧹 Cleaning orphaned desktop entries and NPX stubs...".blue());
    clean_desktop_entries();
    clean_npx_stubs();

    println!("{}", "🌐 Executing Omarchy launcher cleanups...".blue());
    run_omarchy_helper_commands();

    println!();
    println!("{}", "🎉 Omarchy debloat & purge complete!".bold().green());
}
