# thn

![CI](https://github.com/ignission/thn/actions/workflows/ci.yml/badge.svg)

Obsidian [Thino](https://github.com/Quorafind/Obsidian-Memos)プラグインと互換性のあるCLIツール。ターミナルからデイリーノートにメモを追記できます。

## 特徴

- Thinoプラグインと完全互換のフォーマット
- Obsidian設定を自動読み取り（二重設定不要）
- UNIX哲学に基づく設計（成功時は沈黙）

## インストール

```bash
# Homebrew（準備中）
brew install ignission/tap/thn

# ソースから
cargo install --path .
```

## 使い方

### 初期設定

```bash
# 対話形式
thn init

# パス指定
thn init /path/to/vault
```

### メモを追記

```bash
thn "今日のミーティングで決定した内容"
```

デイリーノートに以下の形式で追記されます：

```markdown
- 14:30 今日のミーティングで決定した内容
```

### 設定確認

```bash
thn config
```

## 設定

### thn設定

`~/.config/thn/config.toml`

```toml
vault_path = "/path/to/vault"
```

### Obsidian設定（自動読み取り）

以下の設定ファイルから自動的に読み取ります：

| 設定 | ファイル | デフォルト |
|-----|---------|-----------|
| デイリーノートフォルダ | `.obsidian/daily-notes.json` | Vaultルート |
| 日付フォーマット | `.obsidian/daily-notes.json` | `YYYY-MM-DD` |
| 挿入位置 | `.obsidian/plugins/obsidian-memos/data.json` | ファイル末尾 |

## 要件

- Obsidian + Daily Notesプラグイン
- Thinoプラグイン

## ライセンス

MIT
