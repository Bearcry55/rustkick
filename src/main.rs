use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use dialoguer::{theme::ColorfulTheme, Input, Confirm};
use std::process::Command;

fn main() {
    let theme = ColorfulTheme::default();

    // Mode selection
    let args: Vec<String> = std::env::args().collect();
    let mode = if args.len() > 2 && args[1] == "--mode" {
        args[2].clone()
    } else {
        println!("❌ Usage: cargo run -- --mode <git|aur>");
        return;
    };

    // Folder name
    let folder_name: String = Input::with_theme(&theme)
    .with_prompt("📦 Enter deployment folder name")
    .interact_text()
    .unwrap();

    // Add MIT License?
    let add_license = Confirm::with_theme(&theme)
    .with_prompt("📝 Add MIT LICENSE?")
    .default(true)
    .interact()
    .unwrap();

    // Include extra files?
    let include_extra = Confirm::with_theme(&theme)
    .with_prompt("📁 Do you want to include additional files or folders (e.g., config.json, .env)?")
    .default(false)
    .interact()
    .unwrap();

    let mut extras: Vec<String> = Vec::new();
    if include_extra {
        let input: String = Input::with_theme(&theme)
        .with_prompt("📝 Enter file/folder names separated by commas")
        .interact_text()
        .unwrap();
        extras = input.split(',').map(|s| s.trim().to_string()).collect();
    }

    let dest = Path::new(&folder_name);
    fs::create_dir_all(dest.join("src")).unwrap();

    // Copy Rust files
    fs::copy("Cargo.toml", dest.join("Cargo.toml")).unwrap();
    if Path::new("Cargo.lock").exists() {
        fs::copy("Cargo.lock", dest.join("Cargo.lock")).unwrap();
    }
    for entry in fs::read_dir("src").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            fs::copy(&path, dest.join("src").join(path.file_name().unwrap())).unwrap();
        }
    }

    // LICENSE
    if add_license {
        let mit = include_str!("../LICENSE_MIT");
        let mut file = File::create(dest.join("LICENSE")).unwrap();
        file.write_all(mit.as_bytes()).unwrap();
    }

    // README
    let mut readme = File::create(dest.join("README.md")).unwrap();
    writeln!(readme, "# {}\n", folder_name).unwrap();

    if mode == "git" {
        // .gitignore
        let mut gitignore = File::create(dest.join(".gitignore")).unwrap();
        writeln!(gitignore, "/target\nCargo.lock\n**/*.rs.bk").unwrap();

        writeln!(readme, "## 🧭 Quick Start\n\n```bash\ncargo run\ncargo build --release\n\ngit init\ngit remote add origin <your-repo-url>\ngit push -u origin master\n```\n").unwrap();
    } else if mode == "aur" {
        // PKGBUILD
        let mut pkgbuild = File::create(dest.join("PKGBUILD")).unwrap();
        writeln!(pkgbuild, "{}",
                 r#"pkgname=example-rust-cli
                 desc="A CLI tool written in Rust"
                 pkgver=0.1.0
                 pkgrel=1
                 arch=('x86_64')
        url="https://github.com/yourname/example"
        license=('MIT')
        depends=('gcc-libs')
        makedepends=('cargo')
        source=($pkgname::git+https://github.com/yourname/example.git)
        sha256sums=('SKIP')

        build() {
        cd "$srcdir/$pkgname"
        cargo build --release
    }

    package() {
    cd "$srcdir/$pkgname"
    install -Dm755 target/release/example "$pkgdir/usr/bin/example"
    }"#
        ).unwrap();

        writeln!(readme, "## 📦 Arch Linux (AUR) Packaging Guide\n\n1. Edit PKGBUILD to match your repo.\n2. Run:\n\n```bash\nmakepkg -si\n```\n\n3. To publish:\n\n```bash\ngit init\ngit remote add origin ssh://aur@aur.archlinux.org/example.git\ngit push -u origin master\n```\n").unwrap();
    }

    // Include extra files
    for extra in extras {
        if Path::new(&extra).exists() {
            let metadata = fs::metadata(&extra).unwrap();
            if metadata.is_file() {
                fs::copy(&extra, dest.join(&extra)).unwrap();
            } else if metadata.is_dir() {
                let _ = Command::new("cp")
                .arg("-r")
                .arg(&extra)
                .arg(dest.join(&extra))
                .status();
            }
            println!("✅ Included: {}", extra);
        } else {
            println!("⚠️  Skipped: '{}' not found", extra);
        }
    }

    // Optional tip
    let show_tip = Confirm::with_theme(&theme)
    .with_prompt("💡 Would you like some tips for using this boilerplate?")
    .default(true)
    .interact()
    .unwrap();

    if show_tip {
        if mode == "git" {
            println!("\n👉 To publish on GitHub:\n  git init\n  git remote add origin <your-repo-url>\n  git push -u origin master");
        } else if mode == "aur" {
            println!("\n👉 To publish to AUR:\n  git init\n  git remote add origin ssh://aur@aur.archlinux.org/<your-pkg>.git\n  git push -u origin master");
        }
    }

    println!("✅ Boilerplate created at '{}'", folder_name);
}
