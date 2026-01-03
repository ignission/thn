# Tasks Document: Core Features

- [ ] 1. CLI引数定義の拡張
  - File: `src/cli.rs`
  - initサブコマンドにオプション引数`path`を追加
  - メモ引数のバリデーション追加
  - Purpose: 対話形式initと引数指定initの両対応
  - _Leverage: 既存のCli構造体_
  - _Requirements: 2, 3_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer specializing in clap CLI framework | Task: Extend existing Cli struct in src/cli.rs to support optional path argument for init subcommand, following requirements 2 and 3 | Restrictions: Do not break existing CLI interface, maintain backward compatibility, use clap derive macros | _Leverage: existing Cli struct in src/cli.rs | _Requirements: Requirements 2 (init with arg), 3 (interactive init) | Success: `thn init` works without args, `thn init /path` works with path arg, help text is correct | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_

- [ ] 2. 設定ファイル保存機能
  - File: `src/config.rs`
  - Config::save()メソッド実装
  - ディレクトリ自動作成（~/.config/thn/）
  - Purpose: init時に設定を永続化
  - _Leverage: 既存のConfig構造体、tomlクレート_
  - _Requirements: 2_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer with expertise in file I/O and serialization | Task: Implement Config::save() method in src/config.rs to serialize config to TOML and write to ~/.config/thn/config.toml, creating parent directories if needed | Restrictions: Must handle IO errors properly, use dirs crate for config path, follow existing error handling patterns | _Leverage: existing Config struct, toml and dirs crates | _Requirements: Requirement 2 (init saves config) | Success: Config is saved to correct path, directories are created, errors are handled gracefully | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_

- [ ] 3. Vaultパスバリデーション
  - File: `src/config.rs`
  - validate_vault_path()関数実装
  - パス存在確認、.obsidianディレクトリ確認
  - Purpose: 不正なVaultパスを早期に検出
  - _Leverage: std::fs, std::path_
  - _Requirements: 2_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer with expertise in filesystem operations | Task: Implement validate_vault_path() function in src/config.rs to verify path exists and contains .obsidian directory | Restrictions: Must return appropriate ConfigError variants, do not follow symlinks for security, handle edge cases | _Leverage: std::fs, std::path modules | _Requirements: Requirement 2 (vault validation) | Success: Valid vaults pass, non-existent paths return VaultNotFound, paths without .obsidian return NotObsidianVault | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_

- [ ] 4. 対話形式Vaultパス入力
  - File: `src/config.rs`
  - prompt_vault_path()関数実装
  - stdin からパス読み取り、トリム処理
  - Purpose: 引数なしinit時のユーザー入力
  - _Leverage: std::io_
  - _Requirements: 3_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer with expertise in CLI user interaction | Task: Implement prompt_vault_path() function in src/config.rs to display "Vault path: " prompt and read user input from stdin | Restrictions: Must trim whitespace, handle empty input, flush stdout before reading | _Leverage: std::io module | _Requirements: Requirement 3 (interactive init) | Success: Prompt displays correctly, input is trimmed, empty input is rejected | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_

- [ ] 5. Obsidian Daily Notes設定読み取り
  - File: `src/obsidian.rs`
  - DailyNotesSettings構造体とload()実装
  - デフォルト値フォールバック
  - Purpose: デイリーノートのfolder/format取得
  - _Leverage: serde, serde_json_
  - _Requirements: 5_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer with expertise in JSON parsing and serde | Task: Implement DailyNotesSettings struct with load() method in src/obsidian.rs to read .obsidian/daily-notes.json with defaults | Restrictions: Must use serde defaults, handle missing file gracefully, do not panic on parse errors | _Leverage: serde, serde_json crates | _Requirements: Requirement 5 (Obsidian settings) | Success: Settings load from JSON, missing file returns defaults, invalid JSON returns defaults | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_

- [ ] 6. Thino設定読み取り
  - File: `src/obsidian.rs`
  - ThinoSettings構造体とload()実装
  - InsertAfterフィールド読み取り
  - Purpose: メモ挿入位置の取得
  - _Leverage: serde, serde_json_
  - _Requirements: 5_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer with expertise in JSON parsing | Task: Implement ThinoSettings struct with load() method in src/obsidian.rs to read .obsidian/plugins/obsidian-memos/data.json | Restrictions: Must handle InsertAfter field rename with serde, handle missing file/plugin gracefully | _Leverage: serde, serde_json crates | _Requirements: Requirement 5 (Thino settings) | Success: InsertAfter is read correctly, missing plugin returns empty string default | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_

- [ ] 7. 日付フォーマット変換
  - File: `src/obsidian.rs`
  - format_date()関数実装
  - YYYY/MM/DD パターン置換
  - Purpose: Obsidianフォーマットからファイル名生成
  - _Leverage: chrono_
  - _Requirements: 8_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer with expertise in date/time handling | Task: Implement format_date() function in src/obsidian.rs to convert Obsidian format (YYYY-MM-DD) to actual date string using chrono | Restrictions: Support YYYY, MM, DD patterns only, fallback to default for unsupported patterns, do not use regex | _Leverage: chrono crate | _Requirements: Requirement 8 (date format) | Success: YYYY-MM-DD produces 2026-01-03, unsupported patterns fallback to default | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_

- [ ] 8. デイリーノートパス生成
  - File: `src/memo.rs`
  - daily_note_path()関数実装
  - vault_path + folder + date.md の結合
  - Purpose: 追記先ファイルパスの決定
  - _Leverage: std::path, obsidian.rs_
  - _Requirements: 6_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer | Task: Implement daily_note_path() function in src/memo.rs to construct path from vault_path, folder, and formatted date | Restrictions: Handle empty folder correctly (use vault root), ensure cross-platform path handling | _Leverage: std::path, obsidian.rs format_date | _Requirements: Requirement 6 (daily note path) | Success: Path is correctly constructed for various folder settings | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_

- [ ] 9. デイリーノート自動作成
  - File: `src/memo.rs`
  - ensure_daily_note()関数実装
  - ディレクトリ作成、InsertAfterヘッダー追記
  - Purpose: ファイルが存在しない場合の自動生成
  - _Leverage: std::fs_
  - _Requirements: 6_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer with filesystem expertise | Task: Implement ensure_daily_note() function in src/memo.rs to create daily note file with optional InsertAfter header if not exists | Restrictions: Create parent directories recursively, only write header if InsertAfter is set, handle IO errors | _Leverage: std::fs module | _Requirements: Requirement 6 (auto-create daily note) | Success: Missing files are created with correct header, existing files are not modified | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_

- [ ] 10. InsertAfter位置検索
  - File: `src/memo.rs`
  - find_insert_position()関数実装
  - ヘッダーレベル判定、セクション末尾検出
  - Purpose: メモ挿入位置の特定
  - _Leverage: なし（新規実装）_
  - _Requirements: 7_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer with text processing expertise | Task: Implement find_insert_position() function in src/memo.rs to find insertion point after InsertAfter header, before next same-level-or-higher header | Restrictions: Handle header levels 1-6, return file end if header not found, handle empty InsertAfter | _Leverage: None (new implementation) | _Requirements: Requirement 7 (InsertAfter search) | Success: Correct position found for various header configurations | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_

- [ ] 11. メモ行生成
  - File: `src/memo.rs`
  - format_memo_line()関数実装
  - `- HH:MM メモ内容` 形式
  - Purpose: Thino互換フォーマットでメモ生成
  - _Leverage: chrono_
  - _Requirements: 1_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer | Task: Implement format_memo_line() function in src/memo.rs to generate "- HH:MM content" format using current time | Restrictions: Use 24-hour format, pad with zeros, single space after time | _Leverage: chrono crate for current time | _Requirements: Requirement 1 (memo format) | Success: Output matches "- 14:30 memo content" format exactly | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_

- [ ] 12. メモ追記処理
  - File: `src/memo.rs`
  - append_memo()関数実装
  - ファイル読み込み、挿入、書き込み
  - Purpose: メモをデイリーノートに追記
  - _Leverage: std::fs, 上記の関数群_
  - _Requirements: 1_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer | Task: Implement append_memo() function in src/memo.rs to orchestrate reading file, finding position, inserting memo, and writing back | Restrictions: Handle all error cases, ensure newline handling is correct, do not corrupt existing content | _Leverage: std::fs, ensure_daily_note, find_insert_position, format_memo_line | _Requirements: Requirement 1 (memo append) | Success: Memo is inserted at correct position with proper formatting | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_

- [ ] 13. config表示コマンド実装
  - File: `src/main.rs`
  - configサブコマンドハンドラ実装
  - thn設定 + Obsidian設定の統合表示
  - Purpose: 現在の設定を確認可能に
  - _Leverage: config.rs, obsidian.rs_
  - _Requirements: 4_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer | Task: Implement config subcommand handler in src/main.rs to display vault_path, daily_folder, daily_format, insert_after | Restrictions: Handle not-configured state, format output exactly as specified | _Leverage: config.rs Config::load, obsidian.rs settings | _Requirements: Requirement 4 (config display) | Success: Output matches specified format, errors are handled | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_

- [ ] 14. initコマンド実装
  - File: `src/main.rs`
  - initサブコマンドハンドラ実装
  - 引数有無で分岐、バリデーション、保存
  - Purpose: Vault設定の初期化
  - _Leverage: config.rs_
  - _Requirements: 2, 3_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer | Task: Implement init subcommand handler in src/main.rs to handle both path argument and interactive mode | Restrictions: Validate before saving, display appropriate errors, silent on success | _Leverage: config.rs validate_vault_path, prompt_vault_path, save | _Requirements: Requirements 2, 3 (init command) | Success: Both modes work correctly, errors are displayed, success is silent | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_

- [ ] 15. メモコマンド実装
  - File: `src/main.rs`
  - メモ引数ハンドラ実装
  - 設定読み込み、Obsidian設定読み込み、追記
  - Purpose: メインのメモ追記機能
  - _Leverage: config.rs, obsidian.rs, memo.rs_
  - _Requirements: 1_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer | Task: Implement memo argument handler in src/main.rs to orchestrate config loading, obsidian settings, and memo append | Restrictions: Handle empty memo, not-configured state, all error cases with proper messages | _Leverage: config.rs, obsidian.rs, memo.rs append_memo | _Requirements: Requirement 1 (memo command) | Success: Memo is appended successfully, all errors handled with correct messages | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_

- [ ] 16. エラー型定義
  - File: `src/config.rs`, `src/memo.rs`
  - ConfigError, MemoError enum定義
  - Display trait実装
  - Purpose: 統一されたエラーハンドリング
  - _Leverage: thiserror（オプション）_
  - _Requirements: 1, 2, 3, 4_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer with error handling expertise | Task: Define ConfigError and MemoError enums in respective files with Display trait for user-friendly messages | Restrictions: Follow "error: message" format, include relevant context in errors | _Leverage: std::fmt::Display, optionally thiserror crate | _Requirements: Requirements 1-4 (error handling) | Success: All error variants defined, Display produces correct messages | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_

- [ ] 17. 統合テスト
  - File: `tests/integration_test.rs`
  - init → memo → config フローテスト
  - エラーケーステスト
  - Purpose: E2E動作確認
  - _Leverage: tempfile, assert_cmd_
  - _Requirements: All_
  - _Prompt: Implement the task for spec 003-core-features, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer with testing expertise | Task: Create integration tests in tests/integration_test.rs covering init, memo, config flow and error cases | Restrictions: Use temp directories, clean up after tests, test both success and failure paths | _Leverage: tempfile for temp dirs, assert_cmd for CLI testing | _Requirements: All requirements | Success: All flows tested, error cases covered, tests pass reliably | Instructions: Mark task as in-progress in tasks.md before starting, use log-implementation tool after completion, then mark as complete_
