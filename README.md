<div align="center">

<br>

```
 ███████╗███████╗
 ██╔════╝██╔════╝
 ███████╗█████╗  
 ╚════██║██╔══╝  
 ███████║███████╗
 ╚══════╝╚══════╝
```

### **se-pkg**

*One command. Every distro.*

A unified package manager wrapper — install, update, remove, and manage packages  
with the same commands whether you're on Debian, Arch, Fedora, openSUSE, Void, or Gentoo.

<br>

[![Rust](https://img.shields.io/badge/built%20with-Rust-ce422b?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/se-pkg?style=flat-square&color=f5a623)](https://crates.io/crates/se-pkg)
[![License](https://img.shields.io/github/license/barryC12/se-pkg?style=flat-square&color=4a9eff)](LICENSE)
[![Linux](https://img.shields.io/badge/platform-Linux-27ae60?style=flat-square&logo=linux&logoColor=white)](https://github.com/barryC12/se-pkg)

<br>

</div>

---

## Supported distros

| Distro / family | Package manager |
|---|---|
| Debian, Ubuntu, Mint, Pop!\_OS, and derivatives | `apt` |
| Arch Linux, Manjaro, EndeavourOS, and derivatives | `pacman` |
| Fedora, RHEL, CentOS Stream, and derivatives | `dnf` |
| openSUSE Leap / Tumbleweed | `zypper` |
| Void Linux | `xbps` |
| Gentoo | `emerge` |

---

## Installation

### Step 1 — Install Rust

If you don't have Rust installed, get it via `rustup`:

```sh
curl https://sh.rustup.rs -sSf | sh
```

Then activate Rust in your current shell session (no need to restart):

```sh
. "$HOME/.cargo/env"            # sh / bash / zsh / ash / dash / pdksh
source "$HOME/.cargo/env.fish"  # fish
```

Then add `~/.cargo/bin` to your `PATH` permanently:

<details>
<summary><b>fish</b></summary>

```fish
fish_add_path ~/.cargo/bin
```

> Run this once — fish persists it automatically.

</details>

<details>
<summary><b>bash</b></summary>

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

</details>

<details>
<summary><b>zsh</b></summary>

```zsh
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

</details>

<br>

### Step 2 — Install `se`

**Option A — via Cargo (recommended)**

```sh
cargo install se-pkg
```

> **Arch Linux + paru:** If you're on Arch and want to use paru as your package manager, run the setup helper after installing:
> ```sh
> ~/.config/se/se-pkg-paru-helper
> ```

**Option B — build from source**

```sh
git clone https://github.com/barryC12/se-pkg.git
cd se-pkg
cargo build --release
```

The compiled binary lands at `target/release/se`. Move it onto your `PATH` if needed:

```sh
sudo cp target/release/se /usr/local/bin/
```

> **First run:** execute `se` with the flag -Rec to to detect and configure your package manager.

---

## Usage

```sh
se -S  / --get <package>       # Install a package
se -Rm / --remove <package>    # Remove a package
se -Cl / --purge <package>     # Purge a package and its config
se -Up / --update              # Update package lists
se -Ut / --upgrade             # Upgrade all installed packages
se -I  / --index               # Search for a package
se -Rec / --reconfigure        # Reconfigure the package manager
se -H  / --help                # Show command reference
se -V  / --version             # Show the installed version
man se                         # Full in-depth documentation — try this after --help!
```

---

## License

See [LICENSE](LICENSE) for details.
