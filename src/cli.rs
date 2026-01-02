//! CLI引数の定義
//!
//! clapのderiveマクロを使用してCLI引数を定義する。

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Obsidianデイリーノートにメモを追記するCLIツール（Thino互換）
#[derive(Parser)]
#[command(
    name = "thn",
    version,
    about = "CLI tool for appending memos to Obsidian daily notes (Thino compatible)"
)]
pub struct Cli {
    /// サブコマンド
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// メモ内容
    pub memo: Option<String>,
}

/// サブコマンドの定義
#[derive(Subcommand)]
pub enum Commands {
    /// Vaultパスを設定
    Init {
        /// Vaultのパス（省略時は対話形式）
        path: Option<PathBuf>,
    },
    /// 現在の設定を表示
    Config,
}
