//! Obsidian設定ファイル読み取り
//!
//! Obsidian Vault内の設定ファイルを読み取り、デイリーノートやThinoプラグインの
//! 設定を取得する。

use std::fs;
use std::path::Path;

use chrono::NaiveDate;
use serde::Deserialize;

/// デイリーノートプラグインの設定
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct DailyNotesSettings {
    /// デイリーノートを保存するフォルダ（Vaultルートからの相対パス）
    #[serde(default)]
    pub folder: String,

    /// 日付フォーマット（例: "YYYY-MM-DD"）
    #[serde(default = "default_format")]
    pub format: String,
}

impl Default for DailyNotesSettings {
    fn default() -> Self {
        Self {
            folder: String::new(),
            format: default_format(),
        }
    }
}

/// デフォルトの日付フォーマットを返す
#[allow(dead_code)]
fn default_format() -> String {
    "YYYY-MM-DD".to_string()
}

/// Thino（obsidian-memos）プラグインの設定
#[allow(dead_code)]
#[derive(Debug, Default, Deserialize)]
pub struct ThinoSettings {
    /// メモを挿入するヘッダー（例: "# Memos"）
    #[serde(default, rename = "InsertAfter")]
    pub insert_after: String,
}

/// デイリーノートプラグインの設定を読み込む
///
/// `.obsidian/daily-notes.json`から設定を読み取る。
/// ファイルが存在しない場合やパースに失敗した場合はデフォルト値を返す。
///
/// # 引数
///
/// * `vault_path` - Obsidian Vaultのパス
///
/// # 戻り値
///
/// デイリーノート設定。エラー時はデフォルト値。
#[allow(dead_code)]
pub fn load_daily_notes_settings(vault_path: &Path) -> DailyNotesSettings {
    let settings_path = vault_path.join(".obsidian").join("daily-notes.json");

    fs::read_to_string(&settings_path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

/// Thino（obsidian-memos）プラグインの設定を読み込む
///
/// `.obsidian/plugins/obsidian-memos/data.json`から設定を読み取る。
/// ファイルが存在しない場合やパースに失敗した場合はデフォルト値を返す。
///
/// # 引数
///
/// * `vault_path` - Obsidian Vaultのパス
///
/// # 戻り値
///
/// Thino設定。エラー時はデフォルト値。
#[allow(dead_code)]
pub fn load_thino_settings(vault_path: &Path) -> ThinoSettings {
    let settings_path = vault_path
        .join(".obsidian")
        .join("plugins")
        .join("obsidian-memos")
        .join("data.json");

    fs::read_to_string(&settings_path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

/// 未サポートの日付フォーマットパターンかどうかを判定
///
/// ddd（曜日）、MMM（月名）、wo（週番号）などのパターンが含まれる場合はtrueを返す
#[allow(dead_code)]
fn has_unsupported_pattern(format: &str) -> bool {
    // 未サポートパターンのリスト
    // ddd/dddd: 曜日
    // MMM/MMMM: 月名
    // wo: 週番号（序数）
    // ww/WW/W/w: 週番号
    // Do: 日（序数）
    // Mo: 月（序数）
    // Qo/Q: 四半期
    let unsupported = [
        "dddd", "ddd", "MMMM", "MMM", "wo", "ww", "WW", "Do", "Mo", "Qo", "Q", "W", "w",
    ];

    for pattern in unsupported {
        if format.contains(pattern) {
            return true;
        }
    }
    false
}

/// Obsidianの日付フォーマットを実際の日付文字列に変換
///
/// Obsidianで使用される日付フォーマット文字列（例: "YYYY-MM-DD"）を
/// 実際の日付値に基づいた文字列に変換する。
///
/// # 引数
///
/// * `format` - "YYYY-MM-DD" などのフォーマット文字列
/// * `date` - 変換する日付
///
/// # 戻り値
///
/// フォーマットされた日付文字列
///
/// # サポートするパターン
///
/// - `YYYY`: 4桁年（例: 2026）
/// - `MM`: 2桁月（ゼロ埋め、例: 01）
/// - `DD`: 2桁日（ゼロ埋め、例: 03）
///
/// # フォールバック
///
/// 未サポートパターン（ddd, MMM等）が含まれる場合は "YYYY-MM-DD" 形式にフォールバック。
///
/// # 例
///
/// ```
/// use chrono::NaiveDate;
/// use thn::obsidian::format_date;
///
/// let date = NaiveDate::from_ymd_opt(2026, 1, 3).unwrap();
/// assert_eq!(format_date("YYYY-MM-DD", date), "2026-01-03");
/// assert_eq!(format_date("YYYY/MM/DD", date), "2026/01/03");
/// assert_eq!(format_date("YYYYMMDD", date), "20260103");
/// assert_eq!(format_date("DD-MM-YYYY", date), "03-01-2026");
/// ```
#[allow(dead_code)]
pub fn format_date(format: &str, date: NaiveDate) -> String {
    // 未サポートパターンが含まれる場合はデフォルトフォーマットを使用
    if has_unsupported_pattern(format) {
        return date.format("%Y-%m-%d").to_string();
    }

    // サポートするパターンを置換
    format
        .replace("YYYY", &date.format("%Y").to_string())
        .replace("MM", &date.format("%m").to_string())
        .replace("DD", &date.format("%d").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_daily_notes_settings_default() {
        let settings = DailyNotesSettings::default();
        assert_eq!(settings.folder, "");
        assert_eq!(settings.format, "YYYY-MM-DD");
    }

    #[test]
    fn test_thino_settings_default() {
        let settings = ThinoSettings::default();
        assert_eq!(settings.insert_after, "");
    }

    #[test]
    fn test_default_format() {
        assert_eq!(default_format(), "YYYY-MM-DD");
    }

    #[test]
    fn test_load_daily_notes_settings_file_not_found() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path();

        let settings = load_daily_notes_settings(vault_path);

        // デフォルト値が返される
        assert_eq!(settings.folder, "");
        assert_eq!(settings.format, "YYYY-MM-DD");
    }

    #[test]
    fn test_load_daily_notes_settings_success() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path();

        // .obsidian ディレクトリを作成
        let obsidian_dir = vault_path.join(".obsidian");
        fs::create_dir_all(&obsidian_dir).unwrap();

        // daily-notes.json を作成
        let settings_path = obsidian_dir.join("daily-notes.json");
        let json = r#"{"folder": "Daily", "format": "YYYY/MM/DD"}"#;
        fs::write(&settings_path, json).unwrap();

        let settings = load_daily_notes_settings(vault_path);

        assert_eq!(settings.folder, "Daily");
        assert_eq!(settings.format, "YYYY/MM/DD");
    }

    #[test]
    fn test_load_daily_notes_settings_partial_json() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path();

        let obsidian_dir = vault_path.join(".obsidian");
        fs::create_dir_all(&obsidian_dir).unwrap();

        // folder のみ指定
        let settings_path = obsidian_dir.join("daily-notes.json");
        let json = r#"{"folder": "Notes"}"#;
        fs::write(&settings_path, json).unwrap();

        let settings = load_daily_notes_settings(vault_path);

        assert_eq!(settings.folder, "Notes");
        assert_eq!(settings.format, "YYYY-MM-DD"); // デフォルト値
    }

    #[test]
    fn test_load_daily_notes_settings_invalid_json() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path();

        let obsidian_dir = vault_path.join(".obsidian");
        fs::create_dir_all(&obsidian_dir).unwrap();

        // 不正なJSON
        let settings_path = obsidian_dir.join("daily-notes.json");
        fs::write(&settings_path, "not valid json").unwrap();

        let settings = load_daily_notes_settings(vault_path);

        // パース失敗時はデフォルト値
        assert_eq!(settings.folder, "");
        assert_eq!(settings.format, "YYYY-MM-DD");
    }

    #[test]
    fn test_load_thino_settings_file_not_found() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path();

        let settings = load_thino_settings(vault_path);

        assert_eq!(settings.insert_after, "");
    }

    #[test]
    fn test_load_thino_settings_success() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path();

        // プラグインディレクトリを作成
        let plugin_dir = vault_path
            .join(".obsidian")
            .join("plugins")
            .join("obsidian-memos");
        fs::create_dir_all(&plugin_dir).unwrap();

        // data.json を作成
        let settings_path = plugin_dir.join("data.json");
        let json = r##"{"InsertAfter": "# Memos"}"##;
        fs::write(&settings_path, json).unwrap();

        let settings = load_thino_settings(vault_path);

        assert_eq!(settings.insert_after, "# Memos");
    }

    #[test]
    fn test_load_thino_settings_with_other_fields() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path();

        let plugin_dir = vault_path
            .join(".obsidian")
            .join("plugins")
            .join("obsidian-memos");
        fs::create_dir_all(&plugin_dir).unwrap();

        // 他のフィールドも含むJSON
        let settings_path = plugin_dir.join("data.json");
        let json = r###"{"InsertAfter": "## Quick Notes", "someOtherField": "value"}"###;
        fs::write(&settings_path, json).unwrap();

        let settings = load_thino_settings(vault_path);

        assert_eq!(settings.insert_after, "## Quick Notes");
    }

    #[test]
    fn test_load_thino_settings_invalid_json() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path();

        let plugin_dir = vault_path
            .join(".obsidian")
            .join("plugins")
            .join("obsidian-memos");
        fs::create_dir_all(&plugin_dir).unwrap();

        let settings_path = plugin_dir.join("data.json");
        fs::write(&settings_path, "{broken").unwrap();

        let settings = load_thino_settings(vault_path);

        assert_eq!(settings.insert_after, "");
    }

    #[test]
    fn test_load_thino_settings_empty_insert_after() {
        let dir = tempdir().unwrap();
        let vault_path = dir.path();

        let plugin_dir = vault_path
            .join(".obsidian")
            .join("plugins")
            .join("obsidian-memos");
        fs::create_dir_all(&plugin_dir).unwrap();

        // InsertAfter が空文字列
        let settings_path = plugin_dir.join("data.json");
        let json = r#"{"InsertAfter": ""}"#;
        fs::write(&settings_path, json).unwrap();

        let settings = load_thino_settings(vault_path);

        assert_eq!(settings.insert_after, "");
    }

    // ===== format_date テスト =====

    #[test]
    fn test_format_date_standard() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 3).unwrap();
        assert_eq!(format_date("YYYY-MM-DD", date), "2026-01-03");
    }

    #[test]
    fn test_format_date_slash_separator() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 3).unwrap();
        assert_eq!(format_date("YYYY/MM/DD", date), "2026/01/03");
    }

    #[test]
    fn test_format_date_no_separator() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 3).unwrap();
        assert_eq!(format_date("YYYYMMDD", date), "20260103");
    }

    #[test]
    fn test_format_date_european_format() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 3).unwrap();
        assert_eq!(format_date("DD-MM-YYYY", date), "03-01-2026");
    }

    #[test]
    fn test_format_date_dot_separator() {
        let date = NaiveDate::from_ymd_opt(2026, 12, 25).unwrap();
        assert_eq!(format_date("DD.MM.YYYY", date), "25.12.2026");
    }

    #[test]
    fn test_format_date_year_month_only() {
        let date = NaiveDate::from_ymd_opt(2026, 7, 15).unwrap();
        assert_eq!(format_date("YYYY-MM", date), "2026-07");
    }

    #[test]
    fn test_format_date_zero_padding() {
        // 月と日が1桁の場合のゼロ埋め確認
        let date = NaiveDate::from_ymd_opt(2026, 1, 5).unwrap();
        assert_eq!(format_date("YYYY-MM-DD", date), "2026-01-05");
    }

    #[test]
    fn test_format_date_two_digit_month_day() {
        // 月と日が2桁の場合
        let date = NaiveDate::from_ymd_opt(2026, 11, 28).unwrap();
        assert_eq!(format_date("YYYY-MM-DD", date), "2026-11-28");
    }

    #[test]
    fn test_format_date_unsupported_weekday() {
        // ddd（曜日）が含まれる場合はデフォルトにフォールバック
        let date = NaiveDate::from_ymd_opt(2026, 1, 3).unwrap();
        assert_eq!(format_date("YYYY-MM-DD ddd", date), "2026-01-03");
    }

    #[test]
    fn test_format_date_unsupported_full_weekday() {
        // dddd（完全な曜日名）が含まれる場合はデフォルトにフォールバック
        let date = NaiveDate::from_ymd_opt(2026, 1, 3).unwrap();
        assert_eq!(format_date("dddd, YYYY-MM-DD", date), "2026-01-03");
    }

    #[test]
    fn test_format_date_unsupported_month_name() {
        // MMM（月名）が含まれる場合はデフォルトにフォールバック
        let date = NaiveDate::from_ymd_opt(2026, 1, 3).unwrap();
        assert_eq!(format_date("DD MMM YYYY", date), "2026-01-03");
    }

    #[test]
    fn test_format_date_unsupported_full_month_name() {
        // MMMM（完全な月名）が含まれる場合はデフォルトにフォールバック
        let date = NaiveDate::from_ymd_opt(2026, 1, 3).unwrap();
        assert_eq!(format_date("MMMM DD, YYYY", date), "2026-01-03");
    }

    #[test]
    fn test_format_date_unsupported_week_number() {
        // wo（週番号）が含まれる場合はデフォルトにフォールバック
        let date = NaiveDate::from_ymd_opt(2026, 1, 3).unwrap();
        assert_eq!(format_date("YYYY-wo", date), "2026-01-03");
    }

    #[test]
    fn test_format_date_unsupported_ordinal_day() {
        // Do（序数日）が含まれる場合はデフォルトにフォールバック
        let date = NaiveDate::from_ymd_opt(2026, 1, 3).unwrap();
        assert_eq!(format_date("MMMM Do, YYYY", date), "2026-01-03");
    }

    #[test]
    fn test_format_date_empty_format() {
        // 空のフォーマット文字列
        let date = NaiveDate::from_ymd_opt(2026, 1, 3).unwrap();
        assert_eq!(format_date("", date), "");
    }

    #[test]
    fn test_format_date_custom_text() {
        // フォーマットに任意のテキストを含む場合
        let date = NaiveDate::from_ymd_opt(2026, 1, 3).unwrap();
        assert_eq!(format_date("note_YYYY-MM-DD", date), "note_2026-01-03");
    }

    #[test]
    fn test_has_unsupported_pattern_ddd() {
        assert!(has_unsupported_pattern("YYYY-MM-DD ddd"));
    }

    #[test]
    fn test_has_unsupported_pattern_mmm() {
        assert!(has_unsupported_pattern("DD MMM YYYY"));
    }

    #[test]
    fn test_has_unsupported_pattern_wo() {
        assert!(has_unsupported_pattern("YYYY-wo"));
    }

    #[test]
    fn test_has_unsupported_pattern_supported_only() {
        assert!(!has_unsupported_pattern("YYYY-MM-DD"));
        assert!(!has_unsupported_pattern("YYYY/MM/DD"));
        assert!(!has_unsupported_pattern("YYYYMMDD"));
        assert!(!has_unsupported_pattern("DD-MM-YYYY"));
    }
}
