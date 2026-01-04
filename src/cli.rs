//! CLI引数の定義
//!
//! clapのderiveマクロを使用してCLI引数を定義する。

use std::path::PathBuf;

use clap::Parser;

/// Obsidianデイリーノートにメモを追記するCLIツール（Thino互換）
#[derive(Parser)]
#[command(
    name = "thn",
    version,
    about = "CLI tool for appending memos to Obsidian daily notes (Thino compatible)"
)]
pub struct Cli {
    /// Vaultパスを設定（省略時は対話形式）
    #[arg(short = 'i', long, value_name = "PATH")]
    pub init: Option<Option<PathBuf>>,

    /// 現在の設定を表示
    #[arg(short = 'c', long)]
    pub config: bool,

    /// メモ内容（複数引数はスペースで結合）
    #[arg(trailing_var_arg = true)]
    pub memo: Vec<String>,
}
