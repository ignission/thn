mod cli;
mod config;
mod memo;
mod obsidian;

use clap::Parser;

use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { path }) => {
            // init コマンドの処理
            if let Some(vault_path) = path {
                // パスが指定されている場合は設定を保存
                let config = config::Config {
                    vault_path: vault_path.clone(),
                };
                if let Err(err) = config.save() {
                    eprintln!("error: {err}");
                    std::process::exit(1);
                }
            } else {
                // 対話形式は未実装
                eprintln!("error: interactive mode not implemented. please specify vault path");
                std::process::exit(1);
            }
        }
        Some(Commands::Config) => {
            // config コマンドの処理
            match config::load() {
                Ok(config) => {
                    println!("vault_path = {:?}", config.vault_path.display());
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    std::process::exit(1);
                }
            }
        }
        None => {
            // メモ追記の処理
            if let Some(memo_content) = &cli.memo {
                // 設定を読み込む
                let config = match config::load() {
                    Ok(c) => c,
                    Err(err) => {
                        eprintln!("error: {err}");
                        std::process::exit(1);
                    }
                };

                // メモを追記
                if let Err(err) = memo::append_memo(&config.vault_path, memo_content) {
                    eprintln!("error: {err}");
                    std::process::exit(1);
                }
            } else {
                // メモ内容がない場合のエラー
                eprintln!("error: memo content required");
                std::process::exit(1);
            }
        }
    }
}
