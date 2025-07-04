mod cmd;
mod internal;

use clap::Parser;

fn main() {
    // 解析command
    let cli = cmd::Cli::parse();
    // 处理command
    match cli.command {
        Some(cmd::Command::New { name, axum, app }) => cmd::project::run(name, axum, app),
        Some(cmd::Command::App { name, axum }) => cmd::app::run(name, axum),
        _ => {
            println!("欢迎使用noble-gase[Rust]脚手架");
        }
    }
}
