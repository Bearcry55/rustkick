# 🚀 rustkick

**A minimal, interactive Rust boilerplate exporter for GitHub and AUR.**

`rustkick` helps developers instantly generate deployment-ready folders for Rust projects — whether for publishing to GitHub or preparing AUR packages. It includes a simple guided CLI to set up everything you need.

---

## ✨ Features

* 🔧 Copies your existing Rust project's source and `Cargo.toml`
* 📁 Supports custom extra files (like `config.json`, `.env`, etc.)
* 📜 Optional MIT License
* 🧠 Friendly CLI prompts for customization
* 🎯 `--mode git` for GitHub setup
* 🌹 `--mode aur` for AUR packaging

---

## 📦 Usage

```bash
cargo run -- --mode git
```

Or for AUR-ready packaging:

```bash
cargo run -- --mode aur
```

You'll be prompted for:

* Deployment folder name
* Whether to include a license
* Extra files to include (e.g. `hi.txt`, `.env`)
* Optional setup tips (Git/AUR commands)

---

## 🛠️ Example

```bash
cargo run -- --mode git
👌 📦 Enter deployment folder name · Enter deployment folder name \xb7 rustkick
👌 📜 Add MIT LICENSE? · Add MIT LICENSE? \xb7 yes
👌 📁 Include additional files? · Include additional files? \xb7 yes
👌 📜 Enter files · Enter files \xb7 .env, config.json
👌 💡 Tips? · Tips? \xb7 yes
```

This creates a `rustkick/` folder with:

```
rustkick/
├── Cargo.toml
├── LICENSE
├── README.md
├── .env
├── config.json
└── src/
```
📥 Installation

You can install rustkick from source using cargo:
```
cargo install --git https://github.com/Bearcry55/rustkick
```
Or clone it manually and run:
```
git clone https://github.com/Bearcry55/rustkick.git
cd rustkick
cargo build --release
./target/release/rustkick --mode git
```

---

## 🧠 Tips

### For GitHub

```bash
git init
git remote add origin https://github.com/<your-name>/rustkick.git
git push -u origin master
```

### For AUR

```bash
git init
git remote add origin ssh://aur@aur.archlinux.org/<your-package>.git
git push -u origin master
```

---

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## 🙌 Contributing

Contributions welcome! Feel free to open issues or PRs if you have ideas or improvements.

---

## 🔗 Author

Made with ❤️ by [Deep Narayan Banerjee](https://github.com/deep5050)

it is a tool which will make rust coder deployment a little easy it is a initiative for all rustician  
