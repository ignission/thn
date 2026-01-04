# thn - 仕様書

## 1. コマンドラインインターフェース

### 1.1 コマンド一覧

```
thn <メモ内容>              メモをデイリーノートに追記
thn init [<vault_path>]     初期設定
thn config                  現在の設定を表示
thn --help                  ヘルプ表示
thn --version               バージョン表示
```

### 1.2 メモ追記: `thn <メモ内容>`

**説明**: デイリーノートにタイムスタンプ付きメモを追記する。

**引数**:
- `<メモ内容>`: 追記するメモの本文（必須）

**動作**:
1. `~/.config/thn/config.toml` から `vault_path` を読み込む
2. 設定ファイルが存在しない場合、対話形式で `thn init` を実行
3. Obsidian設定を読み込む
4. デイリーノートファイルを特定（なければ作成）
5. `InsertAfter` ヘッダーを検索
6. ヘッダー末尾に `- HH:MM メモ内容` を追記

**出力**:
- 成功時: 出力なし、終了コード 0
- 失敗時: stderrにエラーメッセージ、終了コード 1

**例**:
```bash
$ thn "今日のミーティングで決定した内容"
$ echo $?
0
```

### 1.3 初期設定: `thn init [<vault_path>]`

**説明**: Vaultパスを設定する。

**引数**:
- `<vault_path>`: Obsidian Vaultのパス（省略時は対話形式）

**動作**:
1. 引数があればそのパスを使用
2. 引数がなければプロンプトでパス入力を求める
3. パスの存在確認
4. `.obsidian` ディレクトリの存在確認
5. `~/.config/thn/config.toml` に保存

**出力**:
- 対話形式時: プロンプト表示
- 成功時: 出力なし、終了コード 0
- 失敗時: stderrにエラーメッセージ、終了コード 1

**例**:
```bash
# 引数指定
$ thn init /Users/username/Documents/MyVault

# 対話形式
$ thn init
Vault path: /Users/username/Documents/MyVault
```

### 1.4 設定表示: `thn config`

**説明**: 現在の設定を表示する。

**出力形式**:
```
vault_path: /Users/username/Documents/MyVault
daily_folder: Daily
daily_format: YYYY-MM-DD
insert_after: # Journal
```

**注**: `daily_folder`, `daily_format`, `insert_after` はObsidian設定から読み取った値。

## 2. 設定ファイル

### 2.1 thn設定: `~/.config/thn/config.toml`

**形式**: TOML

**内容**:
```toml
vault_path = "/Users/username/Documents/MyVault"
```

**フィールド**:
| フィールド | 型 | 必須 | 説明 |
|-----------|-----|------|------|
| vault_path | String | Yes | Obsidian Vaultの絶対パス |

### 2.2 Obsidian Daily Notes設定

**パス**: `{vault}/.obsidian/daily-notes.json`

**読み取るフィールド**:
| フィールド | デフォルト値 | 説明 |
|-----------|-------------|------|
| folder | "" (Vaultルート) | デイリーノートの保存フォルダ |
| format | "YYYY-MM-DD" | ファイル名の日付フォーマット |

### 2.3 Thino設定

**パス**: `{vault}/.obsidian/plugins/obsidian-memos/data.json`

**読み取るフィールド**:
| フィールド | デフォルト値 | 説明 |
|-----------|-------------|------|
| InsertAfter | "" (ファイル末尾) | メモを挿入するヘッダー |

## 3. メモ追記ロジック

### 3.1 デイリーノートパスの決定

```
{vault_path}/{folder}/{date}.md
```

- `folder`: daily-notes.json の `folder` 値
- `date`: 現在日付を `format` でフォーマット

**例**:
- vault_path: `/Users/user/Vault`
- folder: `Daily`
- format: `YYYY-MM-DD`
- 現在日付: 2026-01-02

→ `/Users/user/Vault/Daily/2026-01-02.md`

### 3.2 ファイル自動作成

デイリーノートファイルが存在しない場合:

1. 親ディレクトリを再帰的に作成
2. 空のファイルを作成
3. `InsertAfter` ヘッダーがあれば、そのヘッダー行を追記

### 3.3 InsertAfterヘッダー検索

`InsertAfter` が設定されている場合:

1. ファイルを行ごとに読み込む
2. `InsertAfter` と完全一致する行を検索
3. 見つかった場合、そのセクションの末尾位置を特定
   - 次の同レベル以上のヘッダーの直前
   - またはファイル末尾
4. 見つからない場合、ファイル末尾に追記

**ヘッダーレベル判定**:
```
# = レベル1
## = レベル2
### = レベル3
...
```

### 3.4 メモ行の生成

**形式**:
```
- HH:MM メモ内容
```

- `HH`: 24時間表記の時（00-23）
- `MM`: 分（00-59）
- 時刻とメモ内容の間はスペース1つ

**例**:
```
- 14:30 今日のミーティングで決定した内容
```

### 3.5 追記処理

1. 挿入位置を決定
2. 挿入位置に改行 + メモ行を挿入
3. ファイルを上書き保存

## 4. エラー処理

### 4.1 エラーメッセージ形式

```
error: {メッセージ}
```

### 4.2 エラー一覧

| 状況 | メッセージ | 終了コード |
|------|-----------|-----------|
| メモ内容未指定 | `error: memo content required` | 1 |
| 設定ファイルなし | `error: not configured. run 'thn --init [<PATH>]' first` | 1 |
| Vaultパス不正 | `error: vault not found: {path}` | 1 |
| .obsidianなし | `error: not an obsidian vault: {path}` | 1 |
| 書き込み失敗 | `error: failed to write: {path}` | 1 |

## 5. 日付フォーマット対応

### 5.1 サポートするフォーマット

| パターン | 説明 | 例 |
|---------|------|-----|
| YYYY | 4桁年 | 2026 |
| MM | 2桁月（ゼロ埋め） | 01 |
| DD | 2桁日（ゼロ埋め） | 02 |

**サポートする組み合わせ例**:
- `YYYY-MM-DD` → `2026-01-02`
- `YYYY/MM/DD` → `2026/01/02`
- `YYYYMMDD` → `20260102`
- `DD-MM-YYYY` → `02-01-2026`

### 5.2 未サポートのフォーマット

以下は未サポート（デフォルト値にフォールバック）:
- `ddd`, `dddd` (曜日)
- `MMM`, `MMMM` (月名)
- `Qo` (四半期)
- `wo` (週番号)

## 6. 終了コード

| コード | 意味 |
|-------|------|
| 0 | 成功 |
| 1 | エラー |

## 7. ファイルエンコーディング

- **読み込み**: UTF-8
- **書き込み**: UTF-8
- **改行コード**: LF (Unix)

## 8. Rustクレート依存

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
chrono = "0.4"
dirs = "5"
```

## 9. スコープ

### In Scope

- メモの追記（デイリーノートへ）
- 初期設定（Vaultパス設定）
- 設定確認
- Obsidian設定ファイルの自動読み取り
- デイリーノートファイルの自動作成

### Out of Scope

- メモの検索・編集・削除
- 複数Vault対応
- タグ自動付与
- 標準入力（パイプ）からの入力
- GUI/TUI
- Obsidian以外のノートアプリ対応

## 10. マイルストーン

1. **v0.1.0**: 基本機能（メモ追記、init、config）
2. **v0.2.0**: Homebrew tap公開
3. **v1.0.0**: 安定版リリース

## 11. Obsidian設定ファイル詳細

### daily-notes.json

```json
{
  "folder": "Daily",
  "format": "YYYY-MM-DD"
}
```

- `folder`: デイリーノートの保存フォルダ（Vault相対パス）
- `format`: ファイル名の日付フォーマット（moment.js形式）

### obsidian-memos/data.json

```json
{
  "InsertAfter": "# Journal",
  "DefaultTimePrefix": "HH:mm"
}
```

- `InsertAfter`: メモを挿入するヘッダー
- `DefaultTimePrefix`: 時刻フォーマット（参考情報、thnでは`HH:MM`固定）

## 12. 制約事項

1. **moment.js日付フォーマット**: `YYYY-MM-DD`形式のみサポート。複雑なフォーマットは未対応。
2. **ヘッダーレベル**: `InsertAfter`は`# `から`###### `まで対応。
3. **文字コード**: UTF-8のみ対応。
4. **改行コード**: LF（Unix形式）を使用。
