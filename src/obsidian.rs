//! Obsidian設定ファイル読み取り
//!
//! Obsidian Vault内の設定ファイルを読み取り、デイリーノートやThinoプラグインの
//! 設定を取得する。

use std::fs;
use std::path::Path;

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
}
