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
            // タスク14: initコマンドの改善
            let vault_path = match path {
                Some(p) => p.clone(),
                None => {
                    // 対話形式でVaultパスを入力
                    match config::prompt_vault_path() {
                        Ok(p) => p,
                        Err(err) => {
                            eprintln!("error: {err}");
                            std::process::exit(1);
                        }
                    }
                }
            };

            // バリデーション
            if let Err(err) = config::validate_vault_path(&vault_path) {
                eprintln!("error: {err}");
                std::process::exit(1);
            }

            // 保存
            let config = config::Config { vault_path };
            if let Err(err) = config.save() {
                eprintln!("error: {err}");
                std::process::exit(1);
            }
        }
        Some(Commands::Config) => {
            // タスク13: config表示の改善
            // 設定を読み込み
            let config = match config::load() {
                Ok(c) => c,
                Err(err) => {
                    eprintln!("error: {err}");
                    std::process::exit(1);
                }
            };

            // Obsidian設定も読み込み
            let daily = obsidian::load_daily_notes_settings(&config.vault_path);

            // 表示
            println!("vault_path: {}", config.vault_path.display());
            println!("daily_folder: {}", daily.folder);
            println!("daily_format: {}", daily.format);
        }
        None => {
            // タスク15: メモコマンド
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
