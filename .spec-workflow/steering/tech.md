# Technology Stack

## Project Type

CLIツール - ターミナルから実行するコマンドラインアプリケーション

## Core Technologies

### Primary Language(s)
- **Language**: Rust (stable)
- **Compiler**: rustc
- **Language-specific tools**: cargo（ビルド、パッケージ管理、テスト）

### Key Dependencies/Libraries
- **clap 4** (derive feature): CLI引数パース、サブコマンド定義
- **serde 1** (derive feature): シリアライズ/デシリアライズ
- **serde_json 1**: JSON設定ファイル読み取り（Obsidian設定）
- **toml 0.8**: TOML設定ファイル読み書き（thn設定）
- **chrono 0.4**: 日付時刻処理、日付フォーマット変換
- **dirs 5**: クロスプラットフォームディレクトリパス取得

### Application Architecture

シンプルなCLIアーキテクチャ:
```
main.rs (エントリポイント)
    ↓
cli.rs (引数パース)
    ↓
config.rs / obsidian.rs (設定読み取り)
    ↓
memo.rs (メモ追記ロジック)
```

- **単一バイナリ**: 外部依存なしで動作
- **モジュール分離**: 各機能を独立したモジュールに分離
- **設定レイヤー**: thn独自設定とObsidian設定の二層構造

### Data Storage
- **Primary storage**: ファイルシステム（Markdownファイル）
- **設定ファイル**:
  - `~/.config/thn/config.toml` (thn設定)
  - `.obsidian/daily-notes.json` (Obsidian設定、読み取り専用)
  - `.obsidian/plugins/obsidian-memos/data.json` (Thino設定、読み取り専用)
- **Data formats**: TOML（設定）、JSON（Obsidian設定）、Markdown（出力）

### External Integrations
- **APIs**: なし（ローカルファイル操作のみ）
- **Protocols**: なし
- **Authentication**: なし

## Development Environment

### Build & Development Tools
- **Build System**: cargo
- **Package Management**: cargo (Cargo.toml)
- **Development workflow**: `cargo run`、`cargo watch`（オプション）

### Code Quality Tools
- **Static Analysis**: `cargo clippy`
- **Formatting**: `cargo fmt`
- **Testing Framework**: Rust標準テスト（`cargo test`）
- **Documentation**: `cargo doc`

### Version Control & Collaboration
- **VCS**: Git
- **Branching Strategy**: GitHub Flow（main + feature branches）
- **Code Review Process**: Pull Request

## Deployment & Distribution
- **Target Platform(s)**: macOS, Linux, Windows
- **Distribution Method**:
  - Homebrew（macOS/Linux）: `brew install ignission/tap/thn`
  - ソースビルド: `cargo install --path .`
- **Installation Requirements**: Rust toolchain（ソースビルドの場合のみ）
- **Update Mechanism**: Homebrew upgrade / cargo install

## Technical Requirements & Constraints

### Performance Requirements
- 起動から完了まで100ms以内
- メモリ使用量: 最小限（数MB）
- 大きなMarkdownファイルでも遅延なく動作

### Compatibility Requirements
- **Platform Support**: macOS 10.15+, Linux (glibc), Windows 10+
- **Dependency Versions**: Rust 1.70+
- **Standards Compliance**: UTF-8テキストファイル、LF改行

### Security & Compliance
- **Security Requirements**: ローカルファイル操作のみ、ネットワーク通信なし
- **Compliance Standards**: N/A
- **Threat Model**: ファイルパス操作時のパストラバーサル防止

## Technical Decisions & Rationale

### Decision Log
1. **Rust選択**: 高速起動、単一バイナリ配布、型安全性を重視
2. **clap (derive)**: 宣言的なCLI定義、自動ヘルプ生成、メンテナンス性
3. **設定自動読み取り**: Obsidian設定を読み取ることで二重設定を回避

## Known Limitations

- **日付フォーマット制限**: `YYYY`, `MM`, `DD`の組み合わせのみサポート。曜日、月名、週番号は未対応
- **文字コード**: UTF-8のみ対応
- **改行コード**: LF（Unix形式）のみ対応
