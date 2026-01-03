//! メモ追記ロジック
//!
//! デイリーノートへのメモ追記機能を提供する。
//! Thino（obsidian-memos）プラグイン互換のフォーマットで追記する。

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use chrono::Local;

use crate::config;
use crate::obsidian;

/// メモ操作に関するエラー
#[derive(Debug)]
pub enum MemoError {
    /// 設定読み込みエラー
    ConfigError(config::ConfigError),
    /// IO操作エラー
    Io(io::Error),
    /// ファイル書き込み失敗
    WriteFailed(PathBuf),
}

impl fmt::Display for MemoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoError::ConfigError(err) => write!(f, "{err}"),
            MemoError::Io(err) => write!(f, "{err}"),
            MemoError::WriteFailed(path) => write!(f, "failed to write: {}", path.display()),
        }
    }
}

impl std::error::Error for MemoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MemoError::ConfigError(err) => Some(err),
            MemoError::Io(err) => Some(err),
            MemoError::WriteFailed(_) => None,
        }
    }
}

impl From<config::ConfigError> for MemoError {
    fn from(err: config::ConfigError) -> Self {
        MemoError::ConfigError(err)
    }
}

impl From<io::Error> for MemoError {
    fn from(err: io::Error) -> Self {
        MemoError::Io(err)
    }
}

/// デイリーノートのパスを生成する
///
/// # 引数
///
/// * `vault_path` - Obsidian Vaultのパス
/// * `folder` - デイリーノートを保存するフォルダ（Vaultルートからの相対パス）
/// * `date_str` - 日付文字列（例: "2024-01-15"）
///
/// # 戻り値
///
/// デイリーノートのフルパス
fn daily_note_path(vault_path: &Path, folder: &str, date_str: &str) -> PathBuf {
    if folder.is_empty() {
        vault_path.join(format!("{date_str}.md"))
    } else {
        vault_path.join(folder).join(format!("{date_str}.md"))
    }
}

/// デイリーノートが存在しなければ作成する
///
/// # 引数
///
/// * `path` - デイリーノートのパス
///
/// # Errors
///
/// ファイル作成やディレクトリ作成に失敗した場合
fn ensure_daily_note(path: &Path) -> io::Result<()> {
    if path.exists() {
        return Ok(());
    }

    // 親ディレクトリが存在しなければ作成
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // 空ファイルを作成
    fs::File::create(path)?;

    Ok(())
}

/// メモ行をフォーマットする
///
/// Thino互換の形式でメモ行を生成する。
///
/// # 引数
///
/// * `content` - メモの内容
///
/// # 戻り値
///
/// "- HH:MM {content}" 形式の文字列
fn format_memo_line(content: &str) -> String {
    let now = Local::now();
    let time_str = now.format("%H:%M").to_string();
    format!("- {time_str} {content}")
}

/// 日付フォーマットを変換する
///
/// Obsidian形式（YYYY-MM-DD等）をchronoのstrftime形式に変換する。
///
/// # 引数
///
/// * `format` - Obsidian形式の日付フォーマット
///
/// # 戻り値
///
/// chrono strftime形式のフォーマット文字列
fn convert_date_format(format: &str) -> String {
    format
        .replace("YYYY", "%Y")
        .replace("MM", "%m")
        .replace("DD", "%d")
}

/// メモをデイリーノートに追記する
///
/// # 引数
///
/// * `vault_path` - Obsidian Vaultのパス
/// * `content` - 追記するメモの内容
///
/// # Errors
///
/// - `MemoError::ConfigError` - 設定読み込みに失敗した場合
/// - `MemoError::Io` - ファイル操作に失敗した場合
/// - `MemoError::WriteFailed` - ファイル書き込みに失敗した場合
pub fn append_memo(vault_path: &Path, content: &str) -> Result<(), MemoError> {
    // Obsidian設定を読み込む
    let daily_notes_settings = obsidian::load_daily_notes_settings(vault_path);

    // 今日の日付を取得
    let today = Local::now();
    let chrono_format = convert_date_format(&daily_notes_settings.format);
    let date_str = today.format(&chrono_format).to_string();

    // デイリーノートのパスを生成
    let note_path = daily_note_path(vault_path, &daily_notes_settings.folder, &date_str);

    // デイリーノートが存在しなければ作成
    ensure_daily_note(&note_path)?;

    // 既存のコンテンツを読み込む
    let existing_content = fs::read_to_string(&note_path).unwrap_or_default();

    // メモ行を生成
    let memo_line = format_memo_line(content);

    // ファイル末尾に追記
    let new_content = if existing_content.is_empty() {
        format!("{memo_line}\n")
    } else if existing_content.ends_with('\n') {
        format!("{existing_content}{memo_line}\n")
    } else {
        format!("{existing_content}\n{memo_line}\n")
    };

    // ファイルに書き込む
    fs::write(&note_path, new_content).map_err(|_| MemoError::WriteFailed(note_path.clone()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    // ========================================
    // タスク8: daily_note_path テスト
    // ========================================

    #[test]
    fn test_daily_note_path_with_folder() {
        let vault_path = Path::new("/vault");
        let folder = "Daily";
        let date_str = "2024-01-15";

        let result = daily_note_path(vault_path, folder, date_str);

        assert_eq!(result, PathBuf::from("/vault/Daily/2024-01-15.md"));
    }

    #[test]
    fn test_daily_note_path_without_folder() {
        let vault_path = Path::new("/vault");
        let folder = "";
        let date_str = "2024-01-15";

        let result = daily_note_path(vault_path, folder, date_str);

        assert_eq!(result, PathBuf::from("/vault/2024-01-15.md"));
    }

    #[test]
    fn test_daily_note_path_nested_folder() {
        let vault_path = Path::new("/vault");
        let folder = "Notes/Daily";
        let date_str = "2024-01-15";

        let result = daily_note_path(vault_path, folder, date_str);

        assert_eq!(result, PathBuf::from("/vault/Notes/Daily/2024-01-15.md"));
    }

    #[test]
    fn test_daily_note_path_slash_format() {
        let vault_path = Path::new("/vault");
        let folder = "";
        let date_str = "2024/01/15";

        let result = daily_note_path(vault_path, folder, date_str);

        assert_eq!(result, PathBuf::from("/vault/2024/01/15.md"));
    }

    // ========================================
    // タスク9: ensure_daily_note テスト
    // ========================================

    #[test]
    fn test_ensure_daily_note_creates_file() {
        let dir = tempdir().unwrap();
        let note_path = dir.path().join("2024-01-15.md");

        ensure_daily_note(&note_path).unwrap();

        assert!(note_path.exists());
        let content = fs::read_to_string(&note_path).unwrap();
        assert_eq!(content, "");
    }

    #[test]
    fn test_ensure_daily_note_creates_parent_dirs() {
        let dir = tempdir().unwrap();
        let note_path = dir.path().join("Daily").join("2024").join("2024-01-15.md");

        ensure_daily_note(&note_path).unwrap();

        assert!(note_path.exists());
        let content = fs::read_to_string(&note_path).unwrap();
        assert_eq!(content, "");
    }

    #[test]
    fn test_ensure_daily_note_existing_file_not_modified() {
        let dir = tempdir().unwrap();
        let note_path = dir.path().join("2024-01-15.md");

        // 既存のファイルを作成
        fs::write(&note_path, "existing content").unwrap();

        ensure_daily_note(&note_path).unwrap();

        // 既存のコンテンツが保持されている
        let content = fs::read_to_string(&note_path).unwrap();
        assert_eq!(content, "existing content");
    }

    // ========================================
    // タスク11: format_memo_line テスト
    // ========================================

    #[test]
    fn test_format_memo_line_format() {
        let content = "テストメモ";
        let result = format_memo_line(content);

        // "- HH:MM テストメモ" 形式をチェック
        assert!(result.starts_with("- "));
        assert!(result.ends_with(" テストメモ"));

        // 時刻部分のフォーマットをチェック（HH:MM）
        let time_part = &result[2..7];
        assert!(time_part.chars().nth(2) == Some(':'));
        assert!(time_part[..2].parse::<u32>().is_ok());
        assert!(time_part[3..].parse::<u32>().is_ok());
    }

    #[test]
    fn test_format_memo_line_with_special_chars() {
        let content = "メモ with special chars: @#$%";
        let result = format_memo_line(content);

        assert!(result.contains("メモ with special chars: @#$%"));
    }

    // ========================================
    // convert_date_format テスト
    // ========================================

    #[test]
    fn test_convert_date_format_standard() {
        assert_eq!(convert_date_format("YYYY-MM-DD"), "%Y-%m-%d");
    }

    #[test]
    fn test_convert_date_format_slash() {
        assert_eq!(convert_date_format("YYYY/MM/DD"), "%Y/%m/%d");
    }

    #[test]
    fn test_convert_date_format_compact() {
        assert_eq!(convert_date_format("YYYYMMDD"), "%Y%m%d");
    }

    #[test]
    fn test_convert_date_format_reversed() {
        assert_eq!(convert_date_format("DD-MM-YYYY"), "%d-%m-%Y");
    }

    // ========================================
    // MemoError テスト
    // ========================================

    #[test]
    fn test_memo_error_display_write_failed() {
        let err = MemoError::WriteFailed(PathBuf::from("/path/to/file.md"));
        assert_eq!(err.to_string(), "failed to write: /path/to/file.md");
    }

    #[test]
    fn test_memo_error_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        let memo_err: MemoError = io_err.into();
        assert!(matches!(memo_err, MemoError::Io(_)));
    }

    #[test]
    fn test_memo_error_from_config_error() {
        let config_err = config::ConfigError::NotFound;
        let memo_err: MemoError = config_err.into();
        assert!(matches!(memo_err, MemoError::ConfigError(_)));
    }

    // ========================================
    // タスク12: append_memo 統合テスト
    // ========================================

    #[test]
    fn test_append_memo_creates_daily_note() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path();

        // .obsidian ディレクトリを作成
        fs::create_dir_all(vault_path.join(".obsidian")).unwrap();

        // デイリーノート設定（デフォルト）
        let daily_notes_json = r#"{"folder": "", "format": "YYYY-MM-DD"}"#;
        fs::write(
            vault_path.join(".obsidian").join("daily-notes.json"),
            daily_notes_json,
        )
        .unwrap();

        append_memo(vault_path, "テストメモ").unwrap();

        // 今日の日付でファイルが作成されていることを確認
        let today = Local::now().format("%Y-%m-%d").to_string();
        let note_path = vault_path.join(format!("{today}.md"));
        assert!(note_path.exists());

        let content = fs::read_to_string(&note_path).unwrap();
        assert!(content.contains("テストメモ"));
    }

    #[test]
    fn test_append_memo_to_existing_file() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path();

        // Obsidian設定を作成
        fs::create_dir_all(vault_path.join(".obsidian")).unwrap();
        let daily_notes_json = r#"{"folder": "", "format": "YYYY-MM-DD"}"#;
        fs::write(
            vault_path.join(".obsidian").join("daily-notes.json"),
            daily_notes_json,
        )
        .unwrap();

        // 既存のデイリーノートを作成
        let today = Local::now().format("%Y-%m-%d").to_string();
        let note_path = vault_path.join(format!("{today}.md"));
        fs::write(&note_path, "# Existing content\n").unwrap();

        append_memo(vault_path, "新しいメモ").unwrap();

        let content = fs::read_to_string(&note_path).unwrap();
        assert!(content.contains("# Existing content"));
        assert!(content.contains("新しいメモ"));
    }

    #[test]
    fn test_append_memo_with_folder() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path();

        // Obsidian設定を作成
        fs::create_dir_all(vault_path.join(".obsidian")).unwrap();
        let daily_notes_json = r#"{"folder": "Daily", "format": "YYYY-MM-DD"}"#;
        fs::write(
            vault_path.join(".obsidian").join("daily-notes.json"),
            daily_notes_json,
        )
        .unwrap();

        append_memo(vault_path, "フォルダ内メモ").unwrap();

        let today = Local::now().format("%Y-%m-%d").to_string();
        let note_path = vault_path.join("Daily").join(format!("{today}.md"));
        assert!(note_path.exists());

        let content = fs::read_to_string(&note_path).unwrap();
        assert!(content.contains("フォルダ内メモ"));
    }

    #[test]
    fn test_append_memo_multiple() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path();

        // Obsidian設定を作成
        fs::create_dir_all(vault_path.join(".obsidian")).unwrap();
        let daily_notes_json = r#"{"folder": "", "format": "YYYY-MM-DD"}"#;
        fs::write(
            vault_path.join(".obsidian").join("daily-notes.json"),
            daily_notes_json,
        )
        .unwrap();

        // 複数のメモを追記
        append_memo(vault_path, "メモ1").unwrap();
        append_memo(vault_path, "メモ2").unwrap();

        let today = Local::now().format("%Y-%m-%d").to_string();
        let note_path = vault_path.join(format!("{today}.md"));
        let content = fs::read_to_string(&note_path).unwrap();

        assert!(content.contains("メモ1"));
        assert!(content.contains("メモ2"));
    }
}
