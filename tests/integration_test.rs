//! 統合テスト
//!
//! thn CLIの統合テストを実装する。
//! コマンドラインからの実行をシミュレートし、エンドツーエンドの動作を検証する。

#![allow(deprecated)] // assert_cmd::Command::cargo_bin の警告を抑制

use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

/// テスト用のVault構造を作成するヘルパー関数
///
/// .obsidianディレクトリと必要な設定ファイルを含むVaultを作成する。
fn create_test_vault(vault_path: &std::path::Path) {
    let obsidian_dir = vault_path.join(".obsidian");
    fs::create_dir_all(&obsidian_dir).expect("failed to create .obsidian directory");

    // デイリーノート設定
    let daily_notes_json = r#"{"folder": "", "format": "YYYY-MM-DD"}"#;
    fs::write(obsidian_dir.join("daily-notes.json"), daily_notes_json)
        .expect("failed to write daily-notes.json");
}

/// テスト用のVault構造を作成するヘルパー関数（InsertAfter設定あり）
fn create_test_vault_with_thino(vault_path: &std::path::Path) {
    create_test_vault(vault_path);

    // Thino設定
    let plugin_dir = vault_path
        .join(".obsidian")
        .join("plugins")
        .join("obsidian-memos");
    fs::create_dir_all(&plugin_dir).expect("failed to create plugin directory");

    let thino_json = r##"{"InsertAfter": "# Memos"}"##;
    fs::write(plugin_dir.join("data.json"), thino_json).expect("failed to write thino data.json");
}

// ========================================
// テスト1: init -> memo -> config フロー
// ========================================

#[test]
fn test_init_memo_config_flow() {
    // 一時ディレクトリにVault構造を作成
    let vault_dir = tempdir().expect("failed to create temp vault directory");
    create_test_vault(vault_dir.path());

    // 設定ファイル用の一時ディレクトリ
    let config_dir = tempdir().expect("failed to create temp config directory");

    // HOME環境変数を一時ディレクトリに設定してテスト
    // NOTE: dirs::config_dir()はXDG_CONFIG_HOMEを参照する（Linuxの場合）
    // macOSの場合は ~/Library/Application Support を使用するが、
    // テスト環境ではXDG_CONFIG_HOMEを設定することで制御可能

    // Step 1: thn init {path} を実行
    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.env("XDG_CONFIG_HOME", config_dir.path())
        .env("HOME", config_dir.path())
        .arg("init")
        .arg(vault_dir.path());

    cmd.assert().success();

    // Step 2: thn "テストメモ" を実行
    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.env("XDG_CONFIG_HOME", config_dir.path())
        .env("HOME", config_dir.path())
        .arg("テストメモ");

    cmd.assert().success();

    // Step 3: thn config を実行して設定確認
    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.env("XDG_CONFIG_HOME", config_dir.path())
        .env("HOME", config_dir.path())
        .arg("config");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("vault_path:"))
        .stdout(predicate::str::contains("daily_folder:"))
        .stdout(predicate::str::contains("daily_format:"))
        .stdout(predicate::str::contains("insert_after:"));

    // Step 4: デイリーノートファイルの内容を確認
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let note_path = vault_dir.path().join(format!("{today}.md"));

    assert!(note_path.exists(), "デイリーノートが作成されていない");

    let content = fs::read_to_string(&note_path).expect("failed to read daily note");
    assert!(
        content.contains("テストメモ"),
        "メモ内容がデイリーノートに含まれていない"
    );
}

#[test]
fn test_init_memo_config_flow_with_thino() {
    // InsertAfter設定ありのVaultを作成
    let vault_dir = tempdir().expect("failed to create temp vault directory");
    create_test_vault_with_thino(vault_dir.path());

    let config_dir = tempdir().expect("failed to create temp config directory");

    // init
    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.env("XDG_CONFIG_HOME", config_dir.path())
        .env("HOME", config_dir.path())
        .arg("init")
        .arg(vault_dir.path());

    cmd.assert().success();

    // memo
    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.env("XDG_CONFIG_HOME", config_dir.path())
        .env("HOME", config_dir.path())
        .arg("Thino形式テスト");

    cmd.assert().success();

    // デイリーノートの内容を確認
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let note_path = vault_dir.path().join(format!("{today}.md"));

    let content = fs::read_to_string(&note_path).expect("failed to read daily note");

    // ヘッダーが作成されていることを確認
    assert!(
        content.contains("# Memos"),
        "InsertAfterヘッダーが含まれていない"
    );
    assert!(
        content.contains("Thino形式テスト"),
        "メモ内容が含まれていない"
    );
}

#[test]
fn test_multiple_memos() {
    let vault_dir = tempdir().expect("failed to create temp vault directory");
    create_test_vault(vault_dir.path());

    let config_dir = tempdir().expect("failed to create temp config directory");

    // init
    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.env("XDG_CONFIG_HOME", config_dir.path())
        .env("HOME", config_dir.path())
        .arg("init")
        .arg(vault_dir.path());

    cmd.assert().success();

    // 複数のメモを追記
    for i in 1..=3 {
        let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
        cmd.env("XDG_CONFIG_HOME", config_dir.path())
            .env("HOME", config_dir.path())
            .arg(format!("メモ{i}"));

        cmd.assert().success();
    }

    // デイリーノートの内容を確認
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let note_path = vault_dir.path().join(format!("{today}.md"));

    let content = fs::read_to_string(&note_path).expect("failed to read daily note");

    assert!(content.contains("メモ1"), "メモ1が含まれていない");
    assert!(content.contains("メモ2"), "メモ2が含まれていない");
    assert!(content.contains("メモ3"), "メモ3が含まれていない");
}

// ========================================
// テスト2: エラーケース
// ========================================

#[test]
fn test_error_memo_content_required() {
    // メモ内容なしで実行
    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error: memo content required"));
}

#[test]
fn test_error_vault_not_found() {
    // 存在しないパスでinit
    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.arg("init").arg("/nonexistent/path/to/vault");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error: vault not found"));
}

#[test]
fn test_error_not_obsidian_vault() {
    // .obsidianディレクトリがないディレクトリでinit
    let temp_dir = tempdir().expect("failed to create temp directory");

    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.arg("init").arg(temp_dir.path());

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error: not an obsidian vault"));
}

#[test]
fn test_error_not_configured() {
    // 設定なしでメモを追記しようとする
    let config_dir = tempdir().expect("failed to create temp config directory");

    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.env("XDG_CONFIG_HOME", config_dir.path())
        .env("HOME", config_dir.path())
        .arg("テストメモ");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error: not configured"));
}

#[test]
fn test_error_not_configured_config_command() {
    // 設定なしでconfigコマンドを実行
    let config_dir = tempdir().expect("failed to create temp config directory");

    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.env("XDG_CONFIG_HOME", config_dir.path())
        .env("HOME", config_dir.path())
        .arg("config");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error: not configured"));
}

// ========================================
// 追加のエラーケース
// ========================================

#[test]
fn test_init_with_valid_vault() {
    // 正常なVaultでinit
    let vault_dir = tempdir().expect("failed to create temp vault directory");
    create_test_vault(vault_dir.path());

    let config_dir = tempdir().expect("failed to create temp config directory");

    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.env("XDG_CONFIG_HOME", config_dir.path())
        .env("HOME", config_dir.path())
        .arg("init")
        .arg(vault_dir.path());

    // initコマンドが成功することを確認
    cmd.assert().success();

    // NOTE: macOSでは dirs::config_dir() は ~/Library/Application Support を使用するため、
    // 環境変数での制御が困難。ここではコマンドの成功を確認するのみとする。
}

#[test]
fn test_config_shows_vault_path() {
    let vault_dir = tempdir().expect("failed to create temp vault directory");
    create_test_vault(vault_dir.path());

    let config_dir = tempdir().expect("failed to create temp config directory");

    // init
    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.env("XDG_CONFIG_HOME", config_dir.path())
        .env("HOME", config_dir.path())
        .arg("init")
        .arg(vault_dir.path());

    cmd.assert().success();

    // config
    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.env("XDG_CONFIG_HOME", config_dir.path())
        .env("HOME", config_dir.path())
        .arg("config");

    // Vaultパスが出力に含まれることを確認
    let vault_path_str = vault_dir.path().to_string_lossy();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(vault_path_str.as_ref()));
}

#[test]
fn test_memo_format_contains_time() {
    let vault_dir = tempdir().expect("failed to create temp vault directory");
    create_test_vault(vault_dir.path());

    let config_dir = tempdir().expect("failed to create temp config directory");

    // init
    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.env("XDG_CONFIG_HOME", config_dir.path())
        .env("HOME", config_dir.path())
        .arg("init")
        .arg(vault_dir.path());

    cmd.assert().success();

    // memo
    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.env("XDG_CONFIG_HOME", config_dir.path())
        .env("HOME", config_dir.path())
        .arg("時刻テスト");

    cmd.assert().success();

    // デイリーノートの内容を確認
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let note_path = vault_dir.path().join(format!("{today}.md"));

    let content = fs::read_to_string(&note_path).expect("failed to read daily note");

    // "- HH:MM 時刻テスト" 形式になっていることを確認
    assert!(content.contains("- "), "メモがリスト形式になっていない");
    assert!(content.contains(":"), "時刻が含まれていない");
    assert!(content.contains("時刻テスト"), "メモ内容が含まれていない");

    // 正規表現でフォーマットを確認
    let re = regex::Regex::new(r"- \d{2}:\d{2} 時刻テスト").unwrap();
    assert!(
        re.is_match(&content),
        "メモフォーマットが正しくない: {content}"
    );
}

// ========================================
// ヘルプとバージョン
// ========================================

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("thn"))
        .stdout(predicate::str::contains("Thino compatible"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("thn").expect("failed to find thn binary");
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("thn"));
}
