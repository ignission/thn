use std::path::Path;

/// メモをデイリーノートに追記する
///
/// # Arguments
/// * `vault_path` - Obsidian Vaultのパス
/// * `content` - 追記するメモの内容
///
/// # Errors
/// 未実装
pub fn append_memo(vault_path: &Path, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 実際のメモ追記ロジックは次のspecで実装
    let _ = (vault_path, content);
    todo!("メモ追記機能は次のspecで実装予定")
}
