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
                println!("👿 The directory({:?}) is not empty, please confirm!", root);
                return;
            }
            // 创建项目目录
            fs::create_dir_all(root.clone()).unwrap();
            (root, v)
        }
        None => {
            // 判断当前目录是否存在Cargo.toml
            if cur_dir.join("Cargo.toml").exists() {
                println!("👿 The current directory already exists Cargo.toml, please confirm!");
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

    println!("🦀 Project creation completed! please read README")
}
