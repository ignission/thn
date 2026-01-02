# Project Structure

## Directory Organization

```
thn/
├── src/
│   ├── main.rs          # エントリポイント、CLIディスパッチ
│   ├── cli.rs           # clapによるCLI引数定義
│   ├── config.rs        # 設定ファイル管理（~/.config/thn/config.toml）
│   ├── obsidian.rs      # Obsidian設定読み取り（daily notes設定等）
│   └── memo.rs          # メモ追記ロジック（Thinoフォーマット）
├── tests/
│   └── integration/     # 統合テスト
├── Cargo.toml           # 依存関係定義
├── Cargo.lock           # 依存関係ロック
├── LICENSE              # MITライセンス
├── README.md            # プロジェクト説明
└── CLAUDE.md            # AI開発ガイド
```

## Naming Conventions

### Files
- **モジュール**: `snake_case.rs`（例: `daily_notes.rs`）
- **テスト**: `tests/` ディレクトリ配下、または `#[cfg(test)]` モジュール内

### Code
- **構造体/列挙型**: `PascalCase`（例: `Config`, `DailyNotesSettings`）
- **関数/メソッド**: `snake_case`（例: `read_config`, `append_memo`）
- **定数**: `UPPER_SNAKE_CASE`（例: `DEFAULT_DATE_FORMAT`）
- **変数**: `snake_case`（例: `vault_path`, `memo_content`）

## Import Patterns

### Import Order
1. 標準ライブラリ (`std::`)
2. 外部クレート (`clap`, `serde`, etc.)
3. 内部モジュール (`crate::`)

### 例
```rust
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use serde::Deserialize;

use crate::config::Config;
use crate::obsidian::ObsidianSettings;
```

## Code Structure Patterns

### モジュール構成
```rust
// 1. use文（import）
use std::path::PathBuf;

// 2. 定数
const DEFAULT_FORMAT: &str = "YYYY-MM-DD";

// 3. 型定義（構造体、列挙型）
#[derive(Debug, Deserialize)]
pub struct Config {
    pub vault_path: PathBuf,
}

// 4. implブロック
impl Config {
    pub fn load() -> Result<Self, Error> {
        // ...
    }
}

// 5. 関数
pub fn helper_function() {
    // ...
}

// 6. テスト
#[cfg(test)]
mod tests {
    // ...
}
```

### エラー処理パターン
```rust
// Result型を使用、?演算子で伝播
pub fn read_config() -> Result<Config, ConfigError> {
    let path = config_path()?;
    let content = fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
```

## Code Organization Principles

1. **Single Responsibility**: 各モジュールは単一の責務を持つ
   - `cli.rs`: CLI引数定義のみ
   - `config.rs`: thn設定の読み書きのみ
   - `obsidian.rs`: Obsidian設定読み取りのみ
   - `memo.rs`: メモ追記処理のみ

2. **Modularity**: 機能ごとにモジュール分離

3. **Testability**: 各モジュールは単体でテスト可能

4. **Consistency**: Rustの慣習に従う

## Module Boundaries

### 依存関係の方向
```
main.rs
    ↓ 使用
cli.rs ← config.rs ← obsidian.rs
                  ↘ memo.rs ←↙
```

- `main.rs`: 全モジュールを統合
- `cli.rs`: 引数定義のみ、他モジュールに依存しない
- `config.rs`: thn設定を提供
- `obsidian.rs`: Obsidian設定を提供（configに依存）
- `memo.rs`: メモ追記（config, obsidianに依存）

### Public API vs Internal
- `pub`: モジュール外から呼び出す関数/型
- `pub(crate)`: クレート内でのみ使用
- private（デフォルト）: モジュール内のみ

## Code Size Guidelines

- **ファイルサイズ**: 300行以下を目安
- **関数サイズ**: 50行以下を目安
- **ネスト深度**: 4レベル以下

## Documentation Standards

- **公開API**: `///` ドキュメントコメント必須
- **複雑なロジック**: インラインコメントで説明
- **言語**: 日本語（国際化しない限り）

### 例
```rust
/// 設定ファイルを読み込む
///
/// # Errors
/// - ファイルが存在しない場合
/// - パースに失敗した場合
pub fn load() -> Result<Config, ConfigError> {
    // ...
}
```
