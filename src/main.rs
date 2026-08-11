use clap::Parser;
use colored::*;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

const DEFAULT_PACKAGES: &[(&str, &str)] = &[
    ("1password-beta", "1Password Beta Password Manager"),
    ("1password-cli", "1Password Command Line Interface"),
    ("aether", "Aether Peer-to-Peer Newsreader"),
    ("cliamp", "Cliamp Music Player"),
    ("typora", "Typora Markdown Editor"),
    ("spotify", "Spotify Desktop Music Client"),
    ("libreoffice-fresh", "LibreOffice Suite (Fresh)"),
    ("xournalpp", "Xournal++ Handwriting & PDF Editor"),
    ("signal-desktop", "Signal Private Messenger"),
    ("pinta", "Pinta Painting & Image Editing Program"),
    ("obsidian", "Obsidian Knowledge Base"),
    ("obs-studio", "OBS Studio Screen Recorder / Streaming"),
    ("kdenlive", "Kdenlive Video Editor"),
    ("lazydocker", "LazyDocker TUI Terminal Interface"),
    ("claude-code", "Claude Code Assistant"),
    ("chromium", "Chromium Web Browser"),
    ("localsend", "LocalSend File Sharing Client"),
    ("localsend-bin", "LocalSend Binary Package"),
    ("gnome-calculator", "GNOME Desktop Calculator"),
    ("system-config-printer", "Printer Configuration Tool"),
    ("cups", "Common Unix Printing System"),
    ("cups-filters", "CUPS Printer Filters"),
    ("cups-browsed", "CUPS Network Printer Browser"),
    ("cups-pdf", "CUPS PDF Printer Driver"),
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
    about = "Ultra-fast interactive & automated Omarchy/DHH preinstalled packages and launchers purge utility"
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

fn print_banner() {
    println!();
    println!("{}", "╭───────────────────────────────────────────────────────────╮".bright_cyan());
    println!(
        "{}",
        "│  🗑️   O M A R C H Y   D E B L O A T   U T I L I T Y       │".bold().magenta()
    );
    println!(
        "{}",
        "│  ✨ Purge preinstalled bloat & launcher clutter v0.1.0    │".bright_blue()
    );
    println!("{}", "╰───────────────────────────────────────────────────────────╯".bright_cyan());
    println!();
}

fn is_package_installed(pkg: &str) -> bool {
    let output = Command::new("pacman").args(["-Qq", pkg]).output();
    match output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}

fn remove_package(pkg: &str) -> bool {
    let status = Command::new("sudo")
        .args(["pacman", "-Rns", "--noconfirm", pkg])
        .output();

    if let Ok(out) = status {
        if out.status.success() {
            return true;
        }
    }

    let fallback = Command::new("sudo")
        .args(["pacman", "-R", "--noconfirm", pkg])
        .output();

    matches!(fallback, Ok(out) if out.status.success())
}

fn get_home_dir() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string())
}

fn clean_desktop_entries() -> usize {
    let home = get_home_dir();
    let local_apps = format!("{}/.local/share/applications", home);
    let _ = fs::create_dir_all(&local_apps);
    let mut count = 0;

    for entry in DESKTOP_ENTRIES_TO_REMOVE {
        let file_path = format!("{}/{}", local_apps, entry);
        if Path::new(&file_path).exists() {
            if fs::remove_file(&file_path).is_ok() {
                count += 1;
            }
        }
    }

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

                if fs::write(&target_local, new_content).is_ok() {
                    count += 1;
                }
            }
        }
    }
    count
}

fn clean_npx_stubs() -> usize {
    let home = get_home_dir();
    let local_bin = format!("{}/.local/bin", home);
    let mut count = 0;

    for stub in NPX_STUBS_TO_REMOVE {
        let file_path = format!("{}/{}", local_bin, stub);
        if Path::new(&file_path).exists() {
            if fs::remove_file(&file_path).is_ok() {
                count += 1;
            }
        }
    }
    count
}

fn run_omarchy_helper_commands() {
    let _ = Command::new("omarchy-webapp-remove-all").output();
    let _ = Command::new("omarchy-tui-remove-all").output();

    let home = get_home_dir();
    let local_apps = format!("{}/.local/share/applications", home);
    let _ = Command::new("update-desktop-database")
        .arg(&local_apps)
        .output();

    let _ = Command::new("omarchy-restart-walker").output();
}

fn main() {
    let start_time = Instant::now();
    let args = Args::parse();
    print_banner();

    // Section 1: System Inspection
    println!("{}", "╭─ 🔍 System Inspection ──────────────────────────────────────╮".bright_cyan());
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("│  {spinner:.magenta} {msg}")
            .unwrap(),
    );
    pb.set_message("Scanning pacman database for preinstalled Omarchy bloat...");
    pb.enable_steady_tick(std::time::Duration::from_millis(80));

    let mut installed_pkgs: Vec<(&str, &str)> = Vec::new();
    for &(pkg, desc) in DEFAULT_PACKAGES {
        if is_package_installed(pkg) {
            installed_pkgs.push((pkg, desc));
        }
    }
    pb.finish_and_clear();

    if installed_pkgs.is_empty() {
        println!(
            "│  {}",
            "✔ Zero preinstalled bloat packages detected! System is clean.".bright_green().bold()
        );
    } else {
        println!(
            "│  {}",
            format!(
                "⚠️ Found {} preinstalled package(s) on your system.",
                installed_pkgs.len()
            )
            .bright_yellow()
            .bold()
        );
    }
    println!("{}", "╰─────────────────────────────────────────────────────────────╯".bright_cyan());
    println!();

    if args.list {
        if !installed_pkgs.is_empty() {
            println!("{}", "📋 Currently Installed Target Packages:".bold().yellow());
            for &(pkg, desc) in &installed_pkgs {
                println!("  • {:<24} {}", pkg.bright_red().bold(), desc.dimmed());
            }
        }
        return;
    }

    // Section 2: Package Selection & Removal
    let pkgs_to_purge: Vec<(&str, &str)> = if args.all || args.yes {
        installed_pkgs.clone()
    } else if installed_pkgs.is_empty() {
        Vec::new()
    } else {
        let display_items: Vec<String> = installed_pkgs
            .iter()
            .map(|(pkg, desc)| format!("{:<22}  {}", pkg.bold(), desc.dimmed()))
            .collect();

        println!("{}", "╭─ 🎯 Package Purge Selection ────────────────────────────────╮".bright_purple());
        println!("│  Use {} to select/deselect items, {} to confirm:", "[Space]".bold().yellow(), "[Enter]".bold().green());
        println!("{}", "╰─────────────────────────────────────────────────────────────╯".bright_purple());

        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .items(&display_items)
            .defaults(&vec![true; installed_pkgs.len()])
            .interact();

        match selections {
            Ok(indices) => indices.into_iter().map(|i| installed_pkgs[i]).collect(),
            Err(_) => {
                println!("{}", "❌ Selection cancelled by user.".bright_red());
                return;
            }
        }
    };

    let mut removed_count = 0;
    if !pkgs_to_purge.is_empty() {
        println!();
        println!("{}", format!("╭─ 🗑️ Removing {} Selected Packages ─────────────────────────╮", pkgs_to_purge.len()).bright_red());
        for &(pkg, _) in &pkgs_to_purge {
            print!("│  • Removing {:<26} ", pkg.bright_yellow());
            let _ = std::io::stdout().flush();
            if remove_package(pkg) {
                println!("{}", "✔ Removed".bright_green().bold());
                removed_count += 1;
            } else {
                println!("{}", "✘ Failed / Skipped".bright_black());
            }
        }
        println!("{}", "╰─────────────────────────────────────────────────────────────╯".bright_red());
    }

    // Section 3: Launchers & Stubs Cleanup
    println!();
    println!("{}", "╭─ 🧹 Desktop Entry & Launcher Cleanup ───────────────────────╮".bright_blue());
    
    print!("│  • Cleaning orphaned .desktop entries & hiding launchers... ");
    let _ = std::io::stdout().flush();
    let desktop_cleaned = clean_desktop_entries();
    println!("{}", format!("✔ {} entries cleaned", desktop_cleaned).bright_green());

    print!("│  • Removing NPX wrapper stubs from ~/.local/bin...         ");
    let _ = std::io::stdout().flush();
    let stubs_cleaned = clean_npx_stubs();
    println!("{}", format!("✔ {} stubs removed", stubs_cleaned).bright_green());

    print!("│  • Executing Omarchy launcher & walker database reset...    ");
    let _ = std::io::stdout().flush();
    run_omarchy_helper_commands();
    println!("{}", "✔ Complete".bright_green());

    println!("{}", "╰─────────────────────────────────────────────────────────────╯".bright_blue());

    // Section 4: Final Summary Card
    let elapsed = start_time.elapsed().as_secs_f64();
    println!();
    println!("{}", "╭─ ✨ Debloat Summary Card ───────────────────────────────────╮".bright_green().bold());
    println!("│  📦 Packages Purged:          {:<30} │", format!("{} package(s)", removed_count).bright_green().bold());
    println!("│  🧹 Desktop Launchers:        {:<30} │", format!("{} entry(s) cleaned", desktop_cleaned).bright_cyan());
    println!("│  ⚡ Stubs & Wrappers:         {:<30} │", format!("{} stub(s) removed", stubs_cleaned).bright_yellow());
    println!("│  ⏱️ Total Time Elapsed:       {:<30} │", format!("{:.2} seconds", elapsed).bright_white().bold());
    println!("{}", "╰─────────────────────────────────────────────────────────────╯".bright_green().bold());
    println!();
    println!("{}", "🎉 Omarchy debloat completed successfully!".bold().bright_magenta());
    println!();
}
