use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
const VERSION: &str = "3.9.0";
fn home() -> String {
    dirs::home_dir()
        .unwrap()
        .to_string_lossy()
        .to_string()
}
fn config_dir() -> String {
    format!("{}/.config/se", home())
}
fn config_file() -> String {
    format!("{}/config.se", config_dir())
}
fn scripts_dir() -> String {
    format!("{}/scripts", config_dir())
}
fn run(cmd: &str) {
    let _ = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .status();
}
fn exists(cmd: &str) -> bool {
    Command::new("bash")
        .arg("-c")
        .arg(format!("command -v {} >/dev/null 2>&1", cmd))
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
fn setup_pm() -> String {
    fs::create_dir_all(config_dir()).ok();
    fs::create_dir_all(scripts_dir()).ok();
    if exists("xbps-install") {
        println!("\x1b[1;34mDetected Void Linux (xbps)\x1b[0m");
        print!("\x1b[1;33mInstall xtools? [Y/n]\x1b[0m ");
        io::stdout().flush().unwrap();
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        if ans.trim().is_empty() || ans.trim().eq_ignore_ascii_case("y") {
            run("sudo xbps-install -S xtools");
        }
        fs::write(config_file(), "PM=void").unwrap();
        return "void".into();
    }
    if exists("apt") {
        println!("\x1b[1;34mDetected apt-based system\x1b[0m");
        print!("\x1b[1;33mInstall nala? [Y/n]\x1b[0m ");
        io::stdout().flush().unwrap();
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        if ans.trim().is_empty() || ans.trim().eq_ignore_ascii_case("y") {
            run("sudo apt install nala -y");
            fs::write(config_file(), "PM=nala").unwrap();
            return "nala".into();
        } else {
            fs::write(config_file(), "PM=apt").unwrap();
            return "apt".into();
        }
    }
    if exists("pacman") {
        println!("\x1b[1;34mDetected pacman-based system\x1b[0m");
        print!("\x1b[1;33mUse paru (AUR helper)? [y/N]\x1b[0m ");
        io::stdout().flush().unwrap();
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        if ans.trim().eq_ignore_ascii_case("y") {
            if exists("paru") {
                fs::write(config_file(), "PM=paru").unwrap();
                return "paru".into();
            } else {
                println!("Install paru manually, then rerun se.");
                std::process::exit(1);
            }
        }
        fs::write(config_file(), "PM=pacman").unwrap();
        return "pacman".into();
    }
    if exists("dnf") {
        fs::write(config_file(), "PM=dnf").unwrap();
        return "dnf".into();
    }
    if exists("zypper") {
        fs::write(config_file(), "PM=zypper").unwrap();
        return "zypper".into();
    }
    if exists("emerge") {
        fs::write(config_file(), "PM=emerge").unwrap();
        return "emerge".into();
    }
    println!("No supported package manager found.");
    std::process::exit(1);
}
fn load_pm() -> String {
    if !Path::new(&config_file()).exists() {
        return setup_pm();
    }
    let data = fs::read_to_string(config_file()).unwrap_or_default();
    data.lines()
        .find(|l| l.starts_with("PM="))
        .map(|l| l.replace("PM=", ""))
        .unwrap_or_else(|| setup_pm())
}
fn help() {
    println!("\x1b[1mse\x1b[0m — universal package wrapper");
    println!("version {}\n", VERSION);
    println!("COMMANDS");
    println!("  -B,   --brew         Use Homebrew for the following command");
    println!("  -M,   --make         Use make for the following command");
    println!("  -C,   --cargo        Use Cargo for the following command");
    println!("  -GC,  --git-clone    Clone a git repository");
    println!("  -W,   --wget         Download a file with wget");
    println!("  -L,   --lang         Install a programming language runtime");
    println!("  -S,   --get          Install package");
    println!("  -Rm,  --remove       Remove package");
    println!("  -Cl,  --purge        Purge package");
    println!("  -I,   --index        Search packages");
    println!("  -Up,  --update       Update package lists");
    println!("  -Ut,  --upgrade      Upgrade system");
    println!("  -Rec, --reconfigure  Reconfigure package manager");
    println!("  -V,   --version      Show version");
    println!("  -H,   --help         Show help");
    println!("\nMAKE SUBCOMMANDS  (use after -M / --make)");
    println!("  -C,  --clean         Run make clean");
    println!("  -I,  --install       Run sudo make install");
    println!("  -U,  --uninstall     Run sudo make uninstall");
    println!("\nCARGO SUBCOMMANDS  (use after -C / --cargo)");
    println!("  -B,  --build         Run cargo build --release");
    println!("  -I,  --install       Run cargo install <crate>");
    println!("\nEXAMPLES");
    println!("  se -B -S wget            Install wget via Homebrew");
    println!("  se -B -I ripgrep         Search Homebrew for ripgrep");
    println!("  se -B -Up                Update Homebrew");
    println!("  se -M                    Run make");
    println!("  se -M -C                 Run make clean");
    println!("  se -M -I                 Run sudo make install");
    println!("  se -M -U                 Run sudo make uninstall");
    println!("  se -C -B                 Run cargo build --release");
    println!("  se -C -I ripgrep         Run cargo install ripgrep");
    println!("  se -GC https://...       Clone a git repository");
    println!("  se -W https://...        Download a file with wget");
    println!("  se -L scala              Install Scala");
    println!("  se -L java               Install OpenJDK 21");
}
fn custom_pm_exists(pm: &str) -> bool {
    let path = format!("{}/se-pkg-{}", scripts_dir(), pm);
    Path::new(&path).exists()
}
fn run_custom_pm(pm: &str, action: &str, args: &[String]) {
    let script = format!("{}/se-pkg-{}", scripts_dir(), pm);
    let mut cmd = Command::new(script);
    cmd.arg(action);
    for arg in args {
        cmd.arg(arg);
    }
    let _ = cmd.status();
}
fn dispatch_brew(cmd: &str, rest: &[String]) {
    if !exists("brew") {
        println!("\x1b[1;31mHomebrew (brew) is not installed or not in PATH.\x1b[0m");
        println!("Install it from https://brew.sh");
        std::process::exit(1);
    }
    match cmd {
        "-S" | "--get" => {
            let pkg = rest.join(" ");
            run(&format!("brew install {}", pkg));
        }
        "-Rm" | "--remove" => {
            let pkg = rest.join(" ");
            run(&format!("brew uninstall {}", pkg));
        }
        "-Cl" | "--purge" => {
            let pkg = rest.join(" ");
            run(&format!("brew uninstall --zap {}", pkg));
        }
        "-I" | "--index" => {
            let pkg = rest.join(" ");
            run(&format!("brew search {}", pkg));
        }
        "-Up" | "--update" => run("brew update"),
        "-Ut" | "--upgrade" => run("brew upgrade"),
        _ => {
            println!("\x1b[1;33mUnknown option for --brew:\x1b[0m {}", cmd);
            help();
        }
    }
}
fn dispatch_make(subcmd: Option<&str>) {
    if !exists("make") {
        println!("\x1b[1;31mmake is not installed or not in PATH.\x1b[0m");
        std::process::exit(1);
    }
    match subcmd {
        None => run("make"),
        Some("-C") | Some("--clean") => run("make clean"),
        Some("-I") | Some("--install") => run("sudo make install"),
        Some("-U") | Some("--uninstall") => run("sudo make uninstall"),
        Some(other) => {
            println!("\x1b[1;33mUnknown option for --make:\x1b[0m {}", other);
            help();
        }
    }
}
fn dispatch_cargo(subcmd: &str, rest: &[String]) {
    if !exists("cargo") {
        println!("\x1b[1;31mcargo is not installed or not in PATH.\x1b[0m");
        println!("Install Rust via https://rustup.rs");
        std::process::exit(1);
    }
    match subcmd {
        "-B" | "--build" => run("cargo build --release"),
        "-I" | "--install" => {
            if rest.is_empty() {
                println!("\x1b[1;33mcargo install requires a crate name, e.g. se -C -I ripgrep\x1b[0m");
                std::process::exit(1);
            }
            let krate = rest.join(" ");
            run(&format!("cargo install {}", krate));
        }
        _ => {
            println!("\x1b[1;33mUnknown option for --cargo:\x1b[0m {}", subcmd);
            help();
        }
    }
}
fn dispatch_git_clone(url: &str) {
    if !exists("git") {
        println!("\x1b[1;31mgit is not installed or not in PATH.\x1b[0m");
        std::process::exit(1);
    }
    run(&format!("git clone {}", url));
}
fn dispatch_wget(url: &str) {
    if !exists("wget") {
        println!("\x1b[1;31mwget is not installed or not in PATH.\x1b[0m");
        std::process::exit(1);
    }
    run(&format!("wget {}", url));
}
fn ensure_wget(pm: &str) {
    if !exists("wget") {
        println!("\x1b[1;33mwget not found — installing via {}...\x1b[0m", pm);
        let install_cmd = match pm {
            "nala"   => "sudo nala install wget -y",
            "apt"    => "sudo apt install wget -y",
            "pacman" => "sudo pacman -S --noconfirm wget",
            "paru"   => "paru -S --noconfirm wget",
            "dnf"    => "sudo dnf install wget -y",
            "zypper" => "sudo zypper install -y wget",
            "emerge" => "sudo emerge wget",
            "void"   => "sudo xbps-install -Sy wget",
            _        => { println!("\x1b[1;31mCannot auto-install wget for unknown PM\x1b[0m"); std::process::exit(1); }
        };
        run(install_cmd);
        if !exists("wget") {
            println!("\x1b[1;31mwget install failed. Please install it manually.\x1b[0m");
            std::process::exit(1);
        }
    }
}
fn install_java(pm: &str) {
    let cmd = match pm {
        "nala"   => "sudo nala install openjdk-21-jdk -y",
        "apt"    => "sudo apt install openjdk-21-jdk -y",
        "pacman" => "sudo pacman -S --noconfirm jdk21-openjdk",
        "paru"   => "paru -S --noconfirm jdk21-openjdk",
        "dnf"    => "sudo dnf install java-21-openjdk -y",
        "zypper" => "sudo zypper install -y java-21-openjdk",
        "emerge" => "sudo emerge dev-java/openjdk:21",
        "void"   => "sudo xbps-install -Sy openjdk21",
        _ => {
            println!("\x1b[1;31mCannot install OpenJDK 21: unsupported package manager '{}'\x1b[0m", pm);
            std::process::exit(1);
        }
    };
    println!("\x1b[1;34mInstalling OpenJDK 21 via {}...\x1b[0m", pm);
    run(cmd);
}
fn dispatch_lang(lang: &str, pm: &str) {
    match lang.to_lowercase().as_str() {
        "scala" => {
            ensure_wget(pm);
            run("~/.config/se/scripts/scalainst");
        }
        "java" => {
            install_java(pm);
        }
        _ => {
            println!("\x1b[1;33mUnknown language:\x1b[0m {}", lang);
            println!("Supported languages: scala, java");
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        return;
    }
    if args[1] == "-V" || args[1] == "--version" {
        println!("se version {}", VERSION);
        return;
    }
    if args[1] == "-H" || args[1] == "--help" {
        help();
        return;
    }
    if args[1] == "-Rec" || args[1] == "--reconfigure" {
        let _ = fs::remove_file(config_file());
        let pm = setup_pm();
        println!("Reconfigured to {}", pm);
        return;
    }
    // -B / --brew: next arg is the action, remainder are packages
    if args[1] == "-B" || args[1] == "--brew" {
        if args.len() < 3 {
            println!("\x1b[1;33m--brew requires a command, e.g. se -B -S <pkg>\x1b[0m");
            help();
            return;
        }
        dispatch_brew(&args[2], &args[3..].to_vec());
        return;
    }
    // -M / --make: optional subcommand (-C, -I, -U)
    if args[1] == "-M" || args[1] == "--make" {
        let subcmd = args.get(2).map(|s| s.as_str());
        dispatch_make(subcmd);
        return;
    }
    // -C / --cargo: requires subcommand (-B, -I)
    if args[1] == "-C" || args[1] == "--cargo" {
        if args.len() < 3 {
            println!("\x1b[1;33m--cargo requires a subcommand, e.g. se -C -B or se -C -I <crate>\x1b[0m");
            help();
            return;
        }
        dispatch_cargo(&args[2], &args[3..].to_vec());
        return;
    }
    // -GC / --git-clone: requires a URL
    if args[1] == "-GC" || args[1] == "--git-clone" {
        if args.len() < 3 {
            println!("\x1b[1;33m--git-clone requires a URL, e.g. se -GC https://github.com/user/repo\x1b[0m");
            return;
        }
        dispatch_git_clone(&args[2]);
        return;
    }
    // -W / --wget: requires a URL
    if args[1] == "-W" || args[1] == "--wget" {
        if args.len() < 3 {
            println!("\x1b[1;33m--wget requires a URL, e.g. se -W https://example.com/file.tar.gz\x1b[0m");
            return;
        }
        dispatch_wget(&args[2]);
        return;
    }
    // -L / --lang: install a programming language runtime
    if args[1] == "-L" || args[1] == "--lang" {
        if args.len() < 3 {
            println!("\x1b[1;33m--lang requires a language name, e.g. se -L scala\x1b[0m");
            println!("Supported languages: scala, java");
            return;
        }
        let pm = load_pm();
        dispatch_lang(&args[2], &pm);
        return;
    }
    let pm = load_pm();
    if custom_pm_exists(&pm) {
        match args[1].as_str() {
            "-S" | "--get" => run_custom_pm(&pm, "install", &args[2..]),
            "-Rm" | "--remove" => run_custom_pm(&pm, "remove", &args[2..]),
            "-Cl" | "--purge" => run_custom_pm(&pm, "purge", &args[2..]),
            "-I" | "--index" => run_custom_pm(&pm, "search", &args[2..]),
            "-Up" | "--update" => run_custom_pm(&pm, "update", &[]),
            "-Ut" | "--upgrade" => run_custom_pm(&pm, "upgrade", &[]),
            _ => {
                println!("\x1b[1;33mUnknown option:\x1b[0m {}", args[1]);
                help();
            }
        }
        return;
    }
    let (install, remove, purge, search, update, upgrade) = match pm.as_str() {
        "nala" => (
            "sudo nala install",
            "sudo nala remove",
            "sudo nala purge",
            "nala search",
            "sudo nala update",
            "sudo nala upgrade",
        ),
        "apt" => (
            "sudo apt install",
            "sudo apt remove",
            "sudo apt purge",
            "apt search",
            "sudo apt update",
            "sudo apt upgrade",
        ),
        "pacman" => (
            "sudo pacman -S",
            "sudo pacman -R",
            "sudo pacman -Rns",
            "pacman -Ss",
            "sudo pacman -Sy",
            "sudo pacman -Syu",
        ),
        "paru" => (
            "paru -S",
            "paru -R",
            "paru -Rns",
            "paru -Ss",
            "paru -Sy",
            "paru -Syu",
        ),
        "dnf" => (
            "sudo dnf install",
            "sudo dnf remove",
            "sudo dnf remove",
            "dnf search",
            "sudo dnf check-update",
            "sudo dnf upgrade",
        ),
        "zypper" => (
            "sudo zypper install",
            "sudo zypper remove",
            "sudo zypper remove",
            "zypper search",
            "sudo zypper refresh",
            "sudo zypper update",
        ),
        "emerge" => (
            "sudo emerge",
            "sudo emerge -C",
            "sudo emerge -C",
            "emerge -s",
            "sudo emerge --sync",
            "sudo emerge -avuDN @world",
        ),
        "void" => (
            "sudo xbps-install -S",
            "sudo xbps-remove",
            "sudo xbps-remove -R",
            "xbps-query -Rs",
            "sudo xbps-install -S",
            "sudo xbps-install -Su",
        ),
        _ => {
            println!("Unsupported PM");
            return;
        }
    };
    let cmd = &args[1];
    match cmd.as_str() {
        "-S" | "--get" => {
            let pkg = args[2..].join(" ");
            run(&format!("{} {}", install, pkg));
        }
        "-Rm" | "--remove" => {
            let pkg = args[2..].join(" ");
            run(&format!("{} {}", remove, pkg));
        }
        "-Cl" | "--purge" => {
            let pkg = args[2..].join(" ");
            run(&format!("{} {}", purge, pkg));
        }
        "-I" | "--index" => {
            let pkg = args[2..].join(" ");
            run(&format!("{} {}", search, pkg));
        }
        "-Up" | "--update" => run(update),
        "-Ut" | "--upgrade" => run(upgrade),
        _ => {
            println!("\x1b[1;33mUnknown option:\x1b[0m {}", cmd);
            help();
        }
    }
}
