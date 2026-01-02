# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

**thn (Thino CLI)** - ObsidianプラグインThinoと互換性のあるCLIツール。ターミナルからデイリーノートにメモを追記できる。

## 開発コマンド

```bash
# ビルド
cargo build
cargo build --release

# 実行
cargo run -- "メモ内容"
cargo run -- init
cargo run -- init /path/to/vault
cargo run -- config

# テスト
cargo test
cargo test <test_name>

# リント・フォーマット
cargo fmt
cargo clippy
```

## アーキテクチャ

```
src/
├── main.rs      # エントリポイント、CLIディスパッチ
├── cli.rs       # clapによるCLI引数定義
├── config.rs    # 設定ファイル管理（~/.config/thn/config.toml）
├── obsidian.rs  # Obsidian設定読み取り（daily notes設定等）
└── memo.rs      # メモ追記ロジック（Thinoフォーマット）
```

### データフロー

1. `thn "メモ"` → cli.rsでパース
2. config.rs → `~/.config/thn/config.toml`から`vault_path`取得
3. obsidian.rs → Obsidian設定ファイル読み取り
4. memo.rs → InsertAfterヘッダー末尾にメモ追記

## CLIコマンド

| コマンド | 説明 |
|---------|------|
| `thn <メモ>` | デイリーノートにメモ追記 |
| `thn init [path]` | Vaultパス設定（省略時は対話形式） |
| `thn config` | 現在の設定を表示 |

## Thino互換性

### メモフォーマット
```markdown
- HH:MM メモ内容
```

### 追記位置
`InsertAfter`ヘッダーの末尾（次の同レベル以上ヘッダーの直前）に挿入。

### デイリーノートパス
```
{vault_path}/{folder}/{date}.md
```

## 設定ファイル

### thn設定
`~/.config/thn/config.toml`
```toml
vault_path = "/path/to/vault"
```

### Obsidian設定（自動読み取り）

| ファイル | フィールド | デフォルト |
|---------|-----------|-----------|
| `.obsidian/daily-notes.json` | `folder` | "" (Vaultルート) |
| `.obsidian/daily-notes.json` | `format` | "YYYY-MM-DD" |
| `.obsidian/plugins/obsidian-memos/data.json` | `InsertAfter` | "" (ファイル末尾) |

## 日付フォーマット

サポート: `YYYY`, `MM`, `DD` の組み合わせのみ
- `YYYY-MM-DD`, `YYYY/MM/DD`, `YYYYMMDD`, `DD-MM-YYYY` など

未サポート: 曜日(`ddd`)、月名(`MMM`)、週番号(`wo`)等

## エラー処理

- 形式: `error: {メッセージ}`
- 成功時: 出力なし、終了コード 0
- 失敗時: stderrに出力、終了コード 1

主なエラー:
- `error: memo content required`
- `error: not configured. run 'thn init' first`
- `error: vault not found: {path}`
- `error: not an obsidian vault: {path}`

## 依存クレート

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
chrono = "0.4"
dirs = "5"
```

## 設計原則

- **UNIX哲学**: 成功時は沈黙、失敗時のみstderrに出力
- **フォールバック**: Obsidian設定が見つからない場合はデフォルト値を使用
- **自動作成**: デイリーノートファイルが存在しなければ作成

## 制約事項

- 文字コード: UTF-8のみ
- 改行コード: LF（Unix形式）
