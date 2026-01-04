mod cli;
mod config;
mod memo;
mod obsidian;

use std::path::PathBuf;

use clap::Parser;

use cli::Cli;

fn main() {
    let cli = Cli::parse();

    if let Some(init_arg) = cli.init {
        return run_init(init_arg);
    }

    if cli.config {
        return run_config();
    }

    if cli.memo.is_empty() {
        exit_with_error("memo content required");
    }

    run_memo(&cli.memo);
}

fn run_init(init_arg: Option<PathBuf>) {
    let vault_path = init_arg.unwrap_or_else(|| {
        config::prompt_vault_path().unwrap_or_else(|err| exit_with_error(&err.to_string()))
    });

    if let Err(err) = config::validate_vault_path(&vault_path) {
        exit_with_error(&err.to_string());
    }

    let config = config::Config { vault_path };
    if let Err(err) = config.save() {
        exit_with_error(&err.to_string());
    }
}

fn run_config() {
    let config = config::load().unwrap_or_else(|err| exit_with_error(&err.to_string()));
    let daily = obsidian::load_daily_notes_settings(&config.vault_path);

    println!("vault_path: {}", config.vault_path.display());
    println!("daily_folder: {}", daily.folder);
    println!("daily_format: {}", daily.format);
}

fn run_memo(args: &[String]) {
    let memo_content = args.join(" ");
    let config = config::load().unwrap_or_else(|err| exit_with_error(&err.to_string()));

    if let Err(err) = memo::append_memo(&config.vault_path, &memo_content) {
        exit_with_error(&err.to_string());
    }
}

fn exit_with_error(message: &str) -> ! {
    eprintln!("error: {message}");
    std::process::exit(1);
}
