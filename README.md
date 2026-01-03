<p align="center">
  <h1 align="center">thn</h1>
  <p align="center">
    <strong>ターミナルからObsidianデイリーノートにメモを追記</strong>
  </p>
  <p align="center">
    <a href="https://github.com/ignission/thn/actions/workflows/ci.yml"><img src="https://github.com/ignission/thn/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
    <a href="https://github.com/ignission/thn/releases"><img src="https://img.shields.io/github/v/release/ignission/thn" alt="Release"></a>
    <a href="https://github.com/ignission/thn/blob/main/LICENSE"><img src="https://img.shields.io/github/license/ignission/thn" alt="License"></a>
  </p>
</p>

---

**thn**は[Thino](https://github.com/Quorafind/Obsidian-Thino)プラグイン互換のCLIツール。Obsidianを開かずに、ターミナルから素早くメモを記録できます。

```bash
$ thn "会議で決まった新機能の方針をまとめる"
```

デイリーノートに自動追記：

```markdown
- 14:30 会議で決まった新機能の方針をまとめる
```

## ✨ 特徴

- **⚡ 高速**: Obsidianを起動せずにメモを記録
- **🔄 Thino互換**: `- HH:MM メモ` フォーマットでThino UIと連携
- **⚙️ ゼロ設定**: Obsidian設定を自動読み取り
- **🔇 UNIX哲学**: 成功時は沈黙、エラー時のみ出力

## 📦 インストール

### Homebrew (macOS / Linux)

```bash
brew install ignission/tap/thn
```

### Cargo (Rust)

```bash
cargo install --git https://github.com/ignission/thn
```

### バイナリ

[Releases](https://github.com/ignission/thn/releases)からダウンロード

## 🚀 クイックスタート

### 1. Vaultを設定

```bash
# 対話形式
$ thn init
Vault path: /Users/you/Documents/MyVault

# または直接指定
$ thn init /path/to/vault
```

### 2. メモを追記

```bash
$ thn "買い物リスト：牛乳、パン、卵"
```

### 3. 設定を確認

```bash
$ thn config
vault_path: /Users/you/Documents/MyVault
daily_folder: Daily
daily_format: YYYY-MM-DD
insert_after: # Journal
```

## 📝 使用例

```bash
# シンプルなメモ
thn "アイデア：新しいプロジェクト名を考える"

# 引用符なしでも動作（シェルの特殊文字に注意）
thn 明日の予定を確認する

# 複数行は引用符で囲む
thn "TODO:
- タスク1
- タスク2"
```

## ⚙️ 設定

### thn設定ファイル

`~/.config/thn/config.toml`

```toml
vault_path = "/path/to/vault"
```

### Obsidian設定（自動読み取り）

| 設定 | 読み取り元 | デフォルト |
|-----|-----------|-----------|
| デイリーノートフォルダ | `.obsidian/daily-notes.json` | Vaultルート |
| 日付フォーマット | `.obsidian/daily-notes.json` | `YYYY-MM-DD` |
| 挿入位置（InsertAfter） | `.obsidian/plugins/obsidian-memos/data.json` | ファイル末尾 |

## 📋 要件

| 必須 | 推奨 |
|-----|------|
| Obsidian | Thinoプラグイン |
| Daily Notesプラグイン | |

> **Note**: Thinoがなくてもメモは追記されます。ThinoをインストールするとThino UIでメモを閲覧できます。

## ⚠️ 制限事項

### テンプレート

thnはファイルを直接作成するため、**Obsidianテンプレート（Templater含む）は適用されません**。

**回避策：**
1. Obsidianで先にデイリーノートを開く（推奨）
2. 「Open daily note on startup」を有効化
3. [obsidian-cli](https://github.com/Yakitrak/obsidian-cli)を併用

### 対応モード

ThinoのDAILYモードのみ対応。FILE/MULTI/CANVASモードは非対応。

## 🤝 コントリビュート

Issue・PRを歓迎します。

## 📄 ライセンス

[MIT](LICENSE)

---

<p align="center">
  Made with ❤️ for Obsidian users
</p>
