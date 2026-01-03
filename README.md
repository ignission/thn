# thn

![CI](https://github.com/ignission/thn/actions/workflows/ci.yml/badge.svg)

Obsidian [Thino](https://github.com/Quorafind/Obsidian-Memos)プラグインと互換性のあるCLIツール。ターミナルからデイリーノートにメモを追記できます。

## 特徴

- Thinoプラグインと完全互換のフォーマット
- Obsidian設定を自動読み取り（二重設定不要）
- UNIX哲学に基づく設計（成功時は沈黙）

## インストール

```bash
# Homebrew
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
- Thinoプラグイン（推奨：メモをThino UIで閲覧する場合）

## 制限事項

### デイリーノートテンプレート

thnはデイリーノートファイルを直接作成するため、**Obsidianのテンプレート（Templater含む）は適用されません**。

テンプレートを使用している場合は、以下のいずれかの方法で対応してください：

1. **推奨**: Obsidianで先にデイリーノートを開く（テンプレートが適用される）
2. Daily Notes設定で「Open daily note on startup」を有効化
3. [obsidian-cli](https://github.com/Yakitrak/obsidian-cli)の`daily`コマンドを併用

### 対応モード

ThinoのDAILYモード（デイリーノート連携）のみ対応。FILE/MULTI/CANVASモードは非対応です。

## ライセンス

MIT
