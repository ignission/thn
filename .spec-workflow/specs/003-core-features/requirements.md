# Requirements Document

## Introduction

thn CLIのコア機能実装。メモ追記、初期設定（対話形式含む）、設定表示、Obsidian設定読み取り、デイリーノート自動作成、InsertAfterヘッダー検索・追記を実装する。

## Alignment with Product Vision

product.mdで定義した「ワンコマンドメモ追記」を実現する。UNIX哲学（成功時は沈黙、失敗時のみエラー出力）に基づき、Thinoプラグインと完全互換のフォーマットでメモを追記する。

## Requirements

### Requirement 1: メモ追記

**User Story:** ターミナルユーザーとして、`thn "メモ内容"` でデイリーノートにメモを追記したい。Obsidianを開かずに素早く記録するため。

#### Acceptance Criteria

1. WHEN `thn "メモ内容"` を実行 THEN システム SHALL デイリーノートに `- HH:MM メモ内容` 形式で追記する
2. WHEN メモ追記成功 THEN システム SHALL 出力なしで終了コード 0 を返す
3. WHEN メモ内容が空 THEN システム SHALL `error: memo content required` を stderr に出力し終了コード 1 を返す
4. WHEN 設定ファイルが存在しない THEN システム SHALL `error: not configured. run 'thn init' first` を stderr に出力する

### Requirement 2: 初期設定（引数指定）

**User Story:** ユーザーとして、`thn init /path/to/vault` でVaultパスを設定したい。ワンコマンドで設定を完了するため。

#### Acceptance Criteria

1. WHEN `thn init /path/to/vault` を実行 THEN システム SHALL 指定パスを `~/.config/thn/config.toml` に保存する
2. WHEN 指定パスが存在しない THEN システム SHALL `error: vault not found: {path}` を stderr に出力する
3. WHEN 指定パスに `.obsidian` ディレクトリがない THEN システム SHALL `error: not an obsidian vault: {path}` を stderr に出力する
4. WHEN 設定保存成功 THEN システム SHALL 出力なしで終了コード 0 を返す

### Requirement 3: 初期設定（対話形式）

**User Story:** ユーザーとして、`thn init` で対話形式でVaultパスを入力したい。パスを覚えていなくてもタブ補完で入力するため。

#### Acceptance Criteria

1. WHEN `thn init` を引数なしで実行 THEN システム SHALL `Vault path: ` プロンプトを表示する
2. WHEN ユーザーがパスを入力 THEN システム SHALL Requirement 2 と同様のバリデーションを行う
3. WHEN 入力が空 THEN システム SHALL エラーメッセージを表示して再度プロンプトを表示する

### Requirement 4: 設定表示

**User Story:** ユーザーとして、`thn config` で現在の設定を確認したい。設定が正しいか確認するため。

#### Acceptance Criteria

1. WHEN `thn config` を実行 THEN システム SHALL `vault_path`, `daily_folder`, `daily_format`, `insert_after` を表示する
2. WHEN Obsidian設定ファイルが存在しない THEN システム SHALL デフォルト値を表示する
3. IF 設定ファイルが存在しない THEN システム SHALL `error: not configured. run 'thn init' first` を表示する

### Requirement 5: Obsidian設定読み取り

**User Story:** ユーザーとして、Obsidianの設定を自動で読み取ってほしい。二重設定を避けるため。

#### Acceptance Criteria

1. WHEN `.obsidian/daily-notes.json` が存在 THEN システム SHALL `folder` と `format` を読み取る
2. WHEN `.obsidian/plugins/obsidian-memos/data.json` が存在 THEN システム SHALL `InsertAfter` を読み取る
3. IF 設定ファイルが存在しない THEN システム SHALL デフォルト値を使用する（folder: "", format: "YYYY-MM-DD", InsertAfter: ""）

### Requirement 6: デイリーノート自動作成

**User Story:** ユーザーとして、デイリーノートが存在しない場合は自動作成してほしい。手動でファイルを作成する手間を省くため。

#### Acceptance Criteria

1. WHEN デイリーノートファイルが存在しない THEN システム SHALL 親ディレクトリを再帰的に作成する
2. WHEN デイリーノートファイルが存在しない AND InsertAfterが設定されている THEN システム SHALL InsertAfterヘッダー行を含むファイルを作成する
3. WHEN デイリーノートファイルが存在しない AND InsertAfterが未設定 THEN システム SHALL 空のファイルを作成する

### Requirement 7: InsertAfterヘッダー検索

**User Story:** ユーザーとして、Thinoと同じ位置にメモを挿入してほしい。Obsidianで見た時に整理された状態にするため。

#### Acceptance Criteria

1. WHEN InsertAfterが設定されている AND ヘッダーが存在 THEN システム SHALL そのセクション末尾に追記する
2. WHEN InsertAfterが設定されている AND ヘッダーが存在しない THEN システム SHALL ファイル末尾に追記する
3. WHEN InsertAfterが未設定 THEN システム SHALL ファイル末尾に追記する
4. WHEN セクション末尾を特定 THEN システム SHALL 次の同レベル以上ヘッダーの直前を挿入位置とする

### Requirement 8: 日付フォーマット変換

**User Story:** ユーザーとして、Obsidianの日付フォーマット設定に従ってファイル名を決定してほしい。既存のデイリーノートと整合性を保つため。

#### Acceptance Criteria

1. WHEN format が `YYYY-MM-DD` THEN システム SHALL `2026-01-03` 形式のファイル名を生成する
2. WHEN format が `YYYY/MM/DD` THEN システム SHALL `2026/01/03` 形式のファイル名を生成する
3. WHEN format に未サポートパターン（ddd, MMM等）が含まれる THEN システム SHALL デフォルト `YYYY-MM-DD` にフォールバックする

## Non-Functional Requirements

### Code Architecture and Modularity
- **Single Responsibility Principle**: cli.rs（引数）、config.rs（thn設定）、obsidian.rs（Obsidian設定）、memo.rs（追記ロジック）を分離
- **Modular Design**: 各モジュールは独立してテスト可能
- **Clear Interfaces**: モジュール間はResult型で明確なエラー伝播

### Performance
- 起動から完了まで100ms以内
- 大きなMarkdownファイル（10MB以上）でも遅延なく動作

### Security
- パストラバーサル攻撃を防止（Vault外への書き込み禁止）
- ファイル権限は既存ファイルを継承

### Reliability
- 書き込み中断時のデータ損失を防止（アトミック書き込み推奨）
- すべてのエラーケースで適切なエラーメッセージを表示

### Usability
- エラーメッセージは `error: {内容}` の統一形式
- 成功時は沈黙（UNIX哲学）
- 終了コード: 成功=0, エラー=1
