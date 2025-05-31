# AirPods CLI
 ![Rust](https://img.shields.io/badge/Built_with-Rust-934a2d?logo=rust)   ![License](https://img.shields.io/badge/License-MIT-4c1)   ![AirPods](https://img.shields.io/badge/AirPods-Connected-1e90ff?logo=bluetooth)  


A simple, fast terminal application to view your AirPods battery + disconnect your AirPods. Built with Rust 🦀

---

## ✅ Requirements

- A unix-based operating system
    
- [`blueutil`](https://github.com/toy/blueutil) installed (via `brew install blueutil`)
    
- Rust & Cargo (install via [https://rustup.rs](https://rustup.rs))
    

---

## 🚀 One-liner Install

```bash
bash <(curl -s https://raw.githubusercontent.com/myferr/airpods-cli/main/install.sh)
```

---

## 🧪 Usage

```bash
airpods                  # Show battery levels
airpods --disconnect     # Disconnect AirPods
airpods --no-unicode     # Disable emoji output
airpods --help           # Show usage
```

---

## 🧼 Uninstall

```bash
rm -rf ~/airpods-cli
# Then manually remove the alias from your shell config file (e.g., .zshrc or .bashrc)
```
