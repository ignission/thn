//! 設定ファイル管理
//!
//! thn設定ファイル（`{config_dir}/thn/config.toml`）の読み書きを行う。
//! パスはOSによって異なる（macOS: `~/Library/Application Support/`, Linux: `~/.config/`）。

use std::fmt;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// 設定構造体
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Obsidian Vaultのパス
    pub vault_path: PathBuf,
}

/// 設定関連のエラー
#[derive(Debug)]
pub enum ConfigError {
    /// IO操作に失敗
    Io(io::Error),
    /// TOMLのデシリアライズに失敗
    TomlDeserialize(toml::de::Error),
    /// TOMLのシリアライズに失敗
    TomlSerialize(toml::ser::Error),
    /// 設定ファイルが見つからない
    NotFound,
    /// Vaultパスが存在しない
    VaultNotFound(PathBuf),
    /// ObsidianのVaultではない（.obsidianディレクトリがない）
    NotObsidianVault(PathBuf),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(err) => write!(f, "io error: {err}"),
            ConfigError::TomlDeserialize(err) => write!(f, "toml parse error: {err}"),
            ConfigError::TomlSerialize(err) => write!(f, "toml serialize error: {err}"),
            ConfigError::NotFound => {
                write!(f, "not configured. run 'thn --init [<PATH>]' first")
            }
            ConfigError::VaultNotFound(path) => write!(f, "vault not found: {}", path.display()),
            ConfigError::NotObsidianVault(path) => {
                write!(f, "not an obsidian vault: {}", path.display())
            }
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io(err) => Some(err),
            ConfigError::TomlDeserialize(err) => Some(err),
            ConfigError::TomlSerialize(err) => Some(err),
            ConfigError::NotFound
            | ConfigError::VaultNotFound(_)
            | ConfigError::NotObsidianVault(_) => None,
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        if err.kind() == io::ErrorKind::NotFound {
            ConfigError::NotFound
        } else {
            ConfigError::Io(err)
        }
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::TomlDeserialize(err)
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(err: toml::ser::Error) -> Self {
        ConfigError::TomlSerialize(err)
    }
}

/// 設定ファイルのパスを返す
///
/// `{config_dir}/thn/config.toml` のパスを返す。
/// パスはOSによって異なる（macOS: `~/Library/Application Support/`, Linux: `~/.config/`）。
/// `dirs::config_dir()` が利用できない環境ではパニックする。
pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .expect("could not determine config directory")
        .join("thn")
        .join("config.toml")
}

/// 設定ファイルを読み込む
///
/// # Errors
///
/// - `ConfigError::NotFound` - 設定ファイルが存在しない場合
/// - `ConfigError::Io` - ファイル読み込みに失敗した場合
/// - `ConfigError::TomlDeserialize` - TOMLのパースに失敗した場合
pub fn load() -> Result<Config, ConfigError> {
    let path = config_path();
    let content = fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

/// Vaultパスを検証する
///
/// 指定されたパスが存在し、Obsidian Vaultであることを確認する。
///
/// # Errors
///
/// - `ConfigError::VaultNotFound` - パスが存在しない場合
/// - `ConfigError::NotObsidianVault` - .obsidianディレクトリがない場合
pub fn validate_vault_path(path: &Path) -> Result<(), ConfigError> {
    // パスが存在するかチェック
    if !path.exists() {
        return Err(ConfigError::VaultNotFound(path.to_path_buf()));
    }

    // .obsidianディレクトリが存在するかチェック
    let obsidian_dir = path.join(".obsidian");
    if !obsidian_dir.exists() {
        return Err(ConfigError::NotObsidianVault(path.to_path_buf()));
    }

    Ok(())
}

/// 入力された文字列をVaultパスとしてパースする
///
/// `~` で始まるパスはホームディレクトリに展開される。
/// 空文字列の場合はエラーを返す。
///
/// # Errors
///
/// - 入力が空の場合
pub fn parse_vault_path(input: &str) -> Result<PathBuf, io::Error> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "vault path is required",
        ));
    }

    if trimmed == "~" {
        return Ok(dirs::home_dir().unwrap_or_else(|| PathBuf::from("~")));
    }

    if let Some(rest) = trimmed.strip_prefix("~/") {
        let path = dirs::home_dir()
            .map(|home| home.join(rest))
            .unwrap_or_else(|| PathBuf::from(trimmed));
        return Ok(path);
    }

    Ok(PathBuf::from(trimmed))
}

/// 対話形式でVaultパスを入力
///
/// "Vault path: " を表示してstdinから読み取る。
/// `~` で始まるパスはホームディレクトリに展開される。
///
/// # Errors
///
/// - 標準入力からの読み取りに失敗した場合
/// - 入力が空の場合
pub fn prompt_vault_path() -> Result<PathBuf, io::Error> {
    print!("Vault path: ");
    io::stdout().flush()?;

    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;

    parse_vault_path(&line)
}

impl Config {
    /// 設定をファイルに保存する
    ///
    /// ディレクトリが存在しない場合は作成する。
    ///
    /// # Errors
    ///
    /// - `ConfigError::Io` - ファイル書き込みに失敗した場合
    /// - `ConfigError::TomlSerialize` - TOMLのシリアライズに失敗した場合
    pub fn save(&self) -> Result<(), ConfigError> {
        let path = config_path();

        // 親ディレクトリが存在しない場合は作成
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_path_ends_with_expected_path() {
        let path = config_path();
        assert!(path.ends_with("thn/config.toml"));
    }

    #[test]
    fn test_config_serialize_deserialize() {
        let config = Config {
            vault_path: PathBuf::from("/path/to/vault"),
        };

        let toml_str = toml::to_string(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(parsed.vault_path, config.vault_path);
    }

    #[test]
    fn test_config_error_display() {
        let err = ConfigError::NotFound;
        assert_eq!(
            err.to_string(),
            "not configured. run 'thn --init [<PATH>]' first"
        );
    }

    #[test]
    fn test_io_error_not_found_converts_to_config_not_found() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let config_err: ConfigError = io_err.into();
        assert!(matches!(config_err, ConfigError::NotFound));
    }

    #[test]
    fn test_io_error_other_converts_to_config_io() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        let config_err: ConfigError = io_err.into();
        assert!(matches!(config_err, ConfigError::Io(_)));
    }

    #[test]
    fn test_vault_not_found_error_display() {
        let path = PathBuf::from("/nonexistent/path");
        let err = ConfigError::VaultNotFound(path);
        assert_eq!(err.to_string(), "vault not found: /nonexistent/path");
    }

    #[test]
    fn test_not_obsidian_vault_error_display() {
        let path = PathBuf::from("/some/directory");
        let err = ConfigError::NotObsidianVault(path);
        assert_eq!(err.to_string(), "not an obsidian vault: /some/directory");
    }

    #[test]
    fn test_validate_vault_path_not_found() {
        let path = Path::new("/nonexistent/vault/path/12345");
        let result = validate_vault_path(path);
        assert!(matches!(result, Err(ConfigError::VaultNotFound(_))));
    }

    #[test]
    fn test_validate_vault_path_not_obsidian_vault() {
        // 一時ディレクトリを作成（.obsidianなし）
        let temp_dir = tempfile::tempdir().unwrap();
        let result = validate_vault_path(temp_dir.path());
        assert!(matches!(result, Err(ConfigError::NotObsidianVault(_))));
    }

    #[test]
    fn test_validate_vault_path_success() {
        // 一時ディレクトリを作成し、.obsidianディレクトリを追加
        let temp_dir = tempfile::tempdir().unwrap();
        let obsidian_dir = temp_dir.path().join(".obsidian");
        fs::create_dir(&obsidian_dir).unwrap();

        let result = validate_vault_path(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_vault_path_home_only() {
        let home = dirs::home_dir().unwrap();
        let result = parse_vault_path("~").unwrap();
        assert_eq!(result, home);
    }

    #[test]
    fn test_parse_vault_path_with_tilde() {
        let home = dirs::home_dir().unwrap();
        let result = parse_vault_path("~/dev/note").unwrap();
        assert_eq!(result, home.join("dev/note"));
    }

    #[test]
    fn test_parse_vault_path_absolute_unchanged() {
        let result = parse_vault_path("/absolute/path").unwrap();
        assert_eq!(result, PathBuf::from("/absolute/path"));
    }

    #[test]
    fn test_parse_vault_path_relative_unchanged() {
        let result = parse_vault_path("relative/path").unwrap();
        assert_eq!(result, PathBuf::from("relative/path"));
    }

    #[test]
    fn test_parse_vault_path_empty_error() {
        let result = parse_vault_path("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_vault_path_whitespace_only_error() {
        let result = parse_vault_path("   ");
        assert!(result.is_err());
    }
}
