mod cli;
mod config;
mod memo;
mod obsidian;

use clap::Parser;

use cli::Cli;

fn main() {
    let cli = Cli::parse();

    if cli.init.is_some() {
        // --init: Vaultパスを設定
        let init_path = cli.init.unwrap();
        let vault_path = if init_path.as_os_str().is_empty() {
            // パス省略時は対話形式で入力
            match config::prompt_vault_path() {
                Ok(p) => p,
                Err(err) => {
                    eprintln!("error: {err}");
                    std::process::exit(1);
                }
            }
        } else {
            init_path
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
    } else if cli.config {
        // --config: 現在の設定を表示
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
    } else if !cli.memo.is_empty() {
        // メモを追記（複数引数をスペースで結合）
        let memo_content = cli.memo.join(" ");

        // 設定を読み込む
        let config = match config::load() {
            Ok(c) => c,
            Err(err) => {
                eprintln!("error: {err}");
                std::process::exit(1);
            }
        };

        // メモを追記
        if let Err(err) = memo::append_memo(&config.vault_path, &memo_content) {
            eprintln!("error: {err}");
            std::process::exit(1);
        }
    } else {
        // メモ内容がない場合のエラー
        eprintln!("error: memo content required");
        std::process::exit(1);
    }
}
