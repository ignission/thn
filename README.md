<p align="center">
  <h1 align="center">thn</h1>
  <p align="center">
    <strong>Append memos to Obsidian daily notes from your terminal</strong>
  </p>
  <p align="center">
    <a href="https://github.com/ignission/thn/actions/workflows/ci.yml"><img src="https://github.com/ignission/thn/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
    <a href="https://github.com/ignission/thn/releases"><img src="https://img.shields.io/github/v/release/ignission/thn" alt="Release"></a>
    <a href="https://github.com/ignission/thn/blob/main/LICENSE"><img src="https://img.shields.io/github/license/ignission/thn" alt="License"></a>
  </p>
</p>

---

<p align="center">
  <img src="https://github.com/user-attachments/assets/488ba559-eed9-41cc-93be-5073c912897a" alt="thn demo" width="600">
</p>

**thn** is a CLI tool compatible with the [Thino](https://github.com/Quorafind/Obsidian-Thino) plugin. Quickly capture memos from your terminal without opening Obsidian.

```bash
$ thn "Summarize the new feature decisions from the meeting"
```

Automatically appended to your daily note:

```markdown
- 14:30 Summarize the new feature decisions from the meeting
```

<img width="602" height="269" alt="image" src="https://github.com/user-attachments/assets/8ddc566d-db3e-4752-a019-9b4354c4c243" />


## ✨ Features

- **⚡ Fast**: Capture memos without launching Obsidian
- **🔄 Thino-compatible**: Uses `- HH:MM memo` format for Thino UI integration
- **⚙️ Zero config**: Automatically reads Obsidian settings
- **🔇 UNIX philosophy**: Silent on success, output only on error

## 📦 Installation

### Homebrew (macOS / Linux)

```bash
brew install ignission/tap/thn
```

### Cargo (Rust)

```bash
cargo install --git https://github.com/ignission/thn
```

### Binary

Download from [Releases](https://github.com/ignission/thn/releases)

## 🚀 Quick Start

### 1. Configure your vault

```bash
# Interactive mode
$ thn init
Vault path: /Users/you/Documents/MyVault

# Or specify directly
$ thn init /path/to/vault
```

### 2. Append a memo

```bash
$ thn "Shopping list: milk, bread, eggs"
```

### 3. Check configuration

```bash
$ thn config
vault_path: /Users/you/Documents/MyVault
daily_folder: Daily
daily_format: YYYY-MM-DD
```

## 📝 Usage Examples

```bash
# Simple memo
thn "Idea: think of a new project name"

# Works without quotes (be careful with shell special characters)
thn Check tomorrow schedule

# Use quotes for multi-line memos
thn "TODO:
- Task 1
- Task 2"
```

## ⚙️ Configuration

### thn config file

`~/.config/thn/config.toml`

```toml
vault_path = "/path/to/vault"
```

### Obsidian settings (auto-detected)

| Setting | Source | Default |
|---------|--------|---------|
| Daily notes folder | `.obsidian/daily-notes.json` | Vault root |
| Date format | `.obsidian/daily-notes.json` | `YYYY-MM-DD` |

## 📋 Requirements

| Required | Recommended |
|----------|-------------|
| Obsidian | Thino plugin |
| Daily Notes plugin | |

> **Note**: Memos are appended even without Thino. Install Thino to view memos in the Thino UI.

## ⚠️ Limitations

### Templates

thn creates files directly, so **Obsidian templates (including Templater) are not applied**.

**Workarounds:**
1. Open the daily note in Obsidian first (recommended)
2. Enable "Open daily note on startup"
3. Use [obsidian-cli](https://github.com/Yakitrak/obsidian-cli) alongside thn

### Supported modes

Only Thino's DAILY mode is supported. FILE/MULTI/CANVAS modes are not supported.

## 🤝 Contributing

Issues and PRs are welcome.

## 📄 License

[MIT](LICENSE)

---

<p align="center">
  For Obsidian and Terminal Lovers ❤️
</p>
