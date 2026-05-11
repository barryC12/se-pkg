use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

const VERSION: &str = "3.8.5";

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
    println!("  -S,  --get         Install package");
    println!("  -Rm, --remove      Remove package");
    println!("  -Cl, --purge       Purge package");
    println!("  -I,  --index       Search packages");
    println!("  -Up, --update      Update package lists");
    println!("  -Ut, --upgrade     Upgrade system");
    println!("  -Rec,--reconfigure Reconfigure package manager");
    println!("  -V,  --version     Show version");
    println!("  -H,  --help        Show help");
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
