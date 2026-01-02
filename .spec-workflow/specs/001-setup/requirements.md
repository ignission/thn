# Requirements Document

## Introduction

thn CLIプロジェクトの環境構築。Rustプロジェクトの初期化、依存クレートの設定、基本的なモジュール構造の作成を行う。

## Alignment with Product Vision

product.mdで定義した「ワンコマンドメモ追記」を実現するための基盤を構築する。UNIX哲学に基づく設計原則を反映したプロジェクト構造を作成する。

## Requirements

### Requirement 1: Rustプロジェクト初期化

**User Story:** 開発者として、Rustプロジェクトを初期化したい。ビルドと実行ができる状態にするため。

#### Acceptance Criteria

1. WHEN `cargo build` を実行 THEN システム SHALL コンパイルに成功する
2. WHEN `cargo run` を実行 THEN システム SHALL バイナリを実行できる
3. WHEN `cargo test` を実行 THEN システム SHALL テストが実行される

### Requirement 2: 依存クレート設定

**User Story:** 開発者として、必要な依存クレートを設定したい。CLI引数パース、設定ファイル読み書き、日付処理ができるようにするため。

#### Acceptance Criteria

1. WHEN Cargo.toml を確認 THEN clap 4 (derive feature) SHALL 含まれている
2. WHEN Cargo.toml を確認 THEN serde 1 (derive feature) SHALL 含まれている
3. WHEN Cargo.toml を確認 THEN serde_json 1 SHALL 含まれている
4. WHEN Cargo.toml を確認 THEN toml 0.8 SHALL 含まれている
5. WHEN Cargo.toml を確認 THEN chrono 0.4 SHALL 含まれている
6. WHEN Cargo.toml を確認 THEN dirs 5 SHALL 含まれている

### Requirement 3: モジュール構造作成

**User Story:** 開発者として、基本的なモジュール構造を作成したい。機能ごとに分離された設計で開発を進めるため。

#### Acceptance Criteria

1. WHEN src/main.rs を確認 THEN システム SHALL エントリポイントとして存在する
2. WHEN src/cli.rs を確認 THEN システム SHALL CLI引数定義モジュールとして存在する
3. WHEN src/config.rs を確認 THEN システム SHALL 設定管理モジュールとして存在する
4. WHEN src/obsidian.rs を確認 THEN システム SHALL Obsidian設定読み取りモジュールとして存在する
5. WHEN src/memo.rs を確認 THEN システム SHALL メモ追記モジュールとして存在する

### Requirement 4: CLIスケルトン

**User Story:** ユーザーとして、`thn --help` でヘルプを表示したい。使い方を確認するため。

#### Acceptance Criteria

1. WHEN `thn --help` を実行 THEN システム SHALL ヘルプメッセージを表示する
2. WHEN `thn --version` を実行 THEN システム SHALL バージョンを表示する

## Non-Functional Requirements

### Code Architecture and Modularity
- **Single Responsibility Principle**: 各モジュールは単一の責務を持つ
- **Modular Design**: cli, config, obsidian, memo を独立したモジュールに分離
- **Clear Interfaces**: モジュール間は明確なインターフェースで連携

### Performance
- コンパイル時間: リリースビルドで1分以内
- バイナリサイズ: 5MB以下（リリースビルド）

### Reliability
- すべてのモジュールがコンパイルエラーなく動作
- clippy警告がゼロ

### Usability
- `cargo build` だけでビルド可能
- 追加の環境設定不要
