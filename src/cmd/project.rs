use std::{env, fs};

use crate::internal::{self, is_empty_dir};

pub fn run(name: Option<String>, axum: bool, apps: Vec<String>) {
    // 获取当前目录
    let cur_dir = env::current_dir().unwrap().canonicalize().unwrap();

    let (root, name) = match name {
        Some(v) => {
            let root = cur_dir.join(&v);
            // 判断目录是否为空
            if !is_empty_dir(&root) {
                println!("👿 目录({:?})不为空，请确认！", root);
                return;
            }
            // 创建项目目录
            fs::create_dir_all(root.clone()).unwrap();
            (root, v)
        }
        None => {
            // 判断当前目录是否存在Cargo.toml
            if cur_dir.join("Cargo.toml").exists() {
                println!("👿 当前目录({:?})已经存在Cargo.toml，请确认！", cur_dir);
                return;
            }
            let v = cur_dir.file_name().unwrap().to_string_lossy().to_string();
            (cur_dir, v)
        }
    };

    // 创建项目
    if axum {
        internal::build_axum_project(&root, &name, &apps);
    } else {
        internal::build_salvo_project(&root, &name, &apps);
    }

    println!("🍺 项目创建完成！请阅读README")
}
