mod cli;
mod config;
mod memo;
mod obsidian;

use clap::Parser;

use cli::Cli;

fn main() {
    let cli = Cli::parse();

    // --init: Vaultパスを設定
    if let Some(init_arg) = cli.init {
        let vault_path = init_arg.unwrap_or_else(|| {
            config::prompt_vault_path().unwrap_or_else(|err| {
                eprintln!("error: {err}");
                std::process::exit(1);
            })
        });

        if let Err(err) = config::validate_vault_path(&vault_path) {
            eprintln!("error: {err}");
            std::process::exit(1);
        }

        let config = config::Config { vault_path };
        if let Err(err) = config.save() {
            eprintln!("error: {err}");
            std::process::exit(1);
        }
        return;
    }

    // --config: 現在の設定を表示
    if cli.config {
        let config = config::load().unwrap_or_else(|err| {
            eprintln!("error: {err}");
            std::process::exit(1);
        });

        let daily = obsidian::load_daily_notes_settings(&config.vault_path);

        println!("vault_path: {}", config.vault_path.display());
        println!("daily_folder: {}", daily.folder);
        println!("daily_format: {}", daily.format);
        return;
    }

    // メモを追記
    if cli.memo.is_empty() {
        eprintln!("error: memo content required");
        std::process::exit(1);
    }

    let memo_content = cli.memo.join(" ");

    let config = config::load().unwrap_or_else(|err| {
        eprintln!("error: {err}");
        std::process::exit(1);
    });

    if let Err(err) = memo::append_memo(&config.vault_path, &memo_content) {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
