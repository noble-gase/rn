use std::{env, fs};

use tera::Context;

use crate::internal::{self, App, AppMode};

pub const TEMPLATE: &str = r#"
🦀 The app is now created! Please add the following configuration to Cargo.toml:
{% for app in apps %}
[[bin]]
name = "{{ app.name }}"
path = "{{ app.mainfile }}"
{% endfor %}
"#;

pub fn run(apps: Vec<String>, axum: bool) {
    // 检查Cargo.toml是否存在
    if fs::metadata("Cargo.toml").is_err() {
        println!("👿 Cargo.toml does not exist, please confirm!");
        return;
    }

    // 获取当前目录
    let dir = env::current_dir().unwrap().canonicalize().unwrap();

    let mut bins = Vec::<App>::new();
    for name in apps {
        bins.push(App {
            name: name.clone(),
            mainfile: format!("src/app/{}/main.rs", name),
        });
    }

    if axum {
        internal::build_axum_app(&dir, &bins, AppMode::Multi);
    } else {
        internal::build_salvo_app(&dir, &bins, AppMode::Multi);
    }

    let mut tera = tera::Tera::default();
    tera.add_raw_template("app", TEMPLATE).unwrap();

    let mut ctx = Context::new();
    ctx.insert("apps", &bins);

    println!("{}", tera.render("app", &ctx).unwrap());
}
