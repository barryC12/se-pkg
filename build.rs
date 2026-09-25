use std::env;
use std::fs;
use std::path::PathBuf;

fn install_script(src: &PathBuf, scripts_dir: &PathBuf, out_dir: &str) {
    if !src.exists() {
        return;
    }
    fs::create_dir_all(scripts_dir).ok();
    let filename = src.file_name().unwrap();
    let dest = scripts_dir.join(filename);
    if let Err(e) = fs::copy(src, &dest) {
        println!("cargo:warning=Could not install {} to {}: {}", filename.to_string_lossy(), dest.display(), e);
    } else {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&dest).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&dest, perms).ok();
        }
        println!("cargo:warning=Installed {} to {}", filename.to_string_lossy(), dest.display());
    }
}

fn main() {
    let out_dir = match env::var("OUT_DIR") {
        Ok(d) => d,
        Err(_) => return,
    };

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Install man page
    let man_src = PathBuf::from(&manifest_dir).join("man/se.1");
    if man_src.exists() {
        let system_man = PathBuf::from("/usr/local/share/man/man1");
        let user_man = dirs::home_dir()
            .map(|h| h.join(".local/share/man/man1"))
            .unwrap_or_else(|| PathBuf::from(&out_dir).join("man1"));

        let man_dest_dir = if system_man.exists() || fs::create_dir_all(&system_man).is_ok() {
            system_man
        } else {
            fs::create_dir_all(&user_man).ok();
            user_man.clone()
        };

        let man_dest = man_dest_dir.join("se.1");
        if let Err(e) = fs::copy(&man_src, &man_dest) {
            println!("cargo:warning=Could not install man page to {}: {}", man_dest.display(), e);
            if man_dest_dir != user_man {
                fs::create_dir_all(&user_man).ok();
                let fallback = user_man.join("se.1");
                if fs::copy(&man_src, &fallback).is_ok() {
                    println!("cargo:warning=Man page installed to {} instead", fallback.display());
                }
            }
        } else {
            println!("cargo:warning=Man page installed to {}", man_dest.display());
        }
    }

    // Install scripts to ~/.config/se/scripts/
    let scripts_dir = dirs::home_dir()
        .map(|h| h.join(".config/se/scripts"))
        .unwrap_or_else(|| PathBuf::from(&out_dir).join("se/scripts"));

    install_script(
        &PathBuf::from(&manifest_dir).join("scripts/se-pkg-paru-helper"),
        &scripts_dir,
        &out_dir,
    );

    install_script(
        &PathBuf::from(&manifest_dir).join("scripts/scalainst"),
        &scripts_dir,
        &out_dir,
    );
}
