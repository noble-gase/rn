use std::{env, fs};

use crate::core::{self, gen_members};

pub fn run(apps: Vec<String>, axum: bool) {
    // 检查Cargo.toml是否存在
    if fs::metadata("Cargo.toml").is_err() {
        println!("👿 Cargo.toml does not exist, please confirm!");
        return;
    }

    let members = gen_members(&apps, None);

    // 获取当前目录
    let dir = env::current_dir().unwrap().canonicalize().unwrap();

    if axum {
        core::build_axum_app(&dir, apps);
    } else {
        core::build_salvo_app(&dir, apps);
    }

    println!(
        "🦀 The app is now created! Please add `{}` to workspace members",
        members
    );
}
