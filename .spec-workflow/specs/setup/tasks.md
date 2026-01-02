# Tasks Document

- [x] 1. Rustプロジェクト初期化
  - File: Cargo.toml, src/main.rs
  - `cargo init` でプロジェクト作成
  - Cargo.toml にパッケージ情報を設定
  - Purpose: ビルド可能なRustプロジェクトを作成
  - _Requirements: 1_
  - _Prompt: Implement the task for spec setup, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer | Task: cargo init でRustプロジェクトを初期化し、Cargo.tomlにname="thn", version="0.1.0", edition="2021"を設定 | Restrictions: 既存ファイルを上書きしない | Success: cargo build が成功する | Instructions: タスク開始時にtasks.mdの[ ]を[-]に変更、完了後にlog-implementationツールで実装記録、その後[x]に変更_

- [x] 2. 依存クレート追加
  - File: Cargo.toml
  - 必要な依存クレートをCargo.tomlに追加
  - Purpose: CLI、設定、日付処理の依存関係を設定
  - _Requirements: 2_
  - _Prompt: Implement the task for spec setup, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer | Task: Cargo.tomlに依存クレートを追加: clap = { version = "4", features = ["derive"] }, serde = { version = "1", features = ["derive"] }, serde_json = "1", toml = "0.8", chrono = "0.4", dirs = "5" | Restrictions: バージョンは指定通りに設定 | Success: cargo build が依存解決に成功 | Instructions: タスク開始時にtasks.mdの[ ]を[-]に変更、完了後にlog-implementationツールで実装記録、その後[x]に変更_

- [x] 3. CLI引数定義モジュール作成
  - File: src/cli.rs
  - clapを使ったCLI引数構造体を定義
  - サブコマンド（init, config）を定義
  - Purpose: コマンドライン引数のパース機能を提供
  - _Leverage: clap derive macros_
  - _Requirements: 3, 4_
  - _Prompt: Implement the task for spec setup, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer specializing in CLI tools | Task: src/cli.rsを作成し、clap deriveでCli構造体とCommands enumを定義。thn <memo>, thn init [path], thn config の3つのコマンドをサポート | Restrictions: clap 4のderive機能を使用、#[command]属性でname, version, aboutを設定 | Success: cargo build が成功、--helpでヘルプ表示 | Instructions: タスク開始時にtasks.mdの[ ]を[-]に変更、完了後にlog-implementationツールで実装記録、その後[x]に変更_

- [x] 4. 設定管理モジュール作成
  - File: src/config.rs
  - Config構造体とload/save関数を定義
  - ~/.config/thn/config.toml のパス管理
  - Purpose: thn設定ファイルの読み書き機能を提供
  - _Leverage: serde, toml, dirs_
  - _Requirements: 3_
  - _Prompt: Implement the task for spec setup, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer | Task: src/config.rsを作成し、Config構造体（vault_path: PathBuf）、load() -> Result<Config>, save(&self) -> Result<()>, config_path() -> PathBufを実装 | Restrictions: dirsクレートでconfig_dir取得、エラーはResult型で返す | Success: 設定ファイルの読み書きが動作 | Instructions: タスク開始時にtasks.mdの[ ]を[-]に変更、完了後にlog-implementationツールで実装記録、その後[x]に変更_

- [x] 5. Obsidian設定読み取りモジュール作成
  - File: src/obsidian.rs
  - DailyNotesSettings, ThinoSettings構造体を定義
  - Obsidian設定JSONの読み取り関数を実装
  - Purpose: Obsidianの設定を自動読み取りする機能を提供
  - _Leverage: serde_json_
  - _Requirements: 3_
  - _Prompt: Implement the task for spec setup, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer | Task: src/obsidian.rsを作成し、DailyNotesSettings（folder, format）、ThinoSettings（insert_after）構造体と読み取り関数を実装。設定ファイルがない場合はデフォルト値を返す | Restrictions: .obsidian/daily-notes.json と .obsidian/plugins/obsidian-memos/data.json を読み取り | Success: Obsidian設定の読み取りが動作 | Instructions: タスク開始時にtasks.mdの[ ]を[-]に変更、完了後にlog-implementationツールで実装記録、その後[x]に変更_

- [x] 6. メモ追記モジュール作成（スケルトン）
  - File: src/memo.rs
  - append_memo関数のスケルトンを作成
  - Purpose: メモ追記機能の基盤を用意（実装は次のspecで）
  - _Leverage: config, obsidian modules_
  - _Requirements: 3_
  - _Prompt: Implement the task for spec setup, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer | Task: src/memo.rsを作成し、pub fn append_memo(content: &str) -> Result<(), Box<dyn std::error::Error>> のスケルトンを実装。現時点ではtodo!()マクロで未実装を示す | Restrictions: 実際のメモ追記ロジックは実装しない | Success: モジュールがコンパイルできる | Instructions: タスク開始時にtasks.mdの[ ]を[-]に変更、完了後にlog-implementationツールで実装記録、その後[x]に変更_

- [x] 7. main.rsでモジュール統合
  - File: src/main.rs
  - 各モジュールをmod宣言で読み込み
  - CLI引数に応じたディスパッチ処理を実装
  - Purpose: エントリポイントとして全モジュールを統合
  - _Leverage: cli, config, obsidian, memo modules_
  - _Requirements: 3, 4_
  - _Prompt: Implement the task for spec setup, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer | Task: src/main.rsを更新し、mod cli, mod config, mod obsidian, mod memoを宣言。main()でCli::parse()し、コマンドに応じて処理を分岐。init/configコマンドは基本実装、メモ追記は未実装メッセージ | Restrictions: エラー処理はeprint!とstd::process::exit(1)を使用 | Success: thn --help, thn --version が動作 | Instructions: タスク開始時にtasks.mdの[ ]を[-]に変更、完了後にlog-implementationツールで実装記録、その後[x]に変更_

- [x] 8. ビルド確認とclippy対応
  - File: 全ファイル
  - cargo build, cargo clippy, cargo fmt を実行
  - 警告やエラーを修正
  - Purpose: コード品質を確保
  - _Requirements: All_
  - _Prompt: Implement the task for spec setup, first run spec-workflow-guide to get the workflow guide then implement the task: Role: Rust Developer | Task: cargo build --release, cargo clippy, cargo fmt --check を実行し、警告・エラーを全て修正 | Restrictions: clippy警告はゼロにする | Success: 全コマンドが警告なしで成功 | Instructions: タスク開始時にtasks.mdの[ ]を[-]に変更、完了後にlog-implementationツールで実装記録、その後[x]に変更_
